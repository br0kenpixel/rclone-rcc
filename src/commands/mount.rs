#![allow(clippy::unused_io_amount)]
use fuse_rs::{
    fs::{DirEntry, FileInfo, FileStat, OpenFileInfo},
    Filesystem,
};
use nix::{errno::Errno, fcntl::OFlag, sys::stat::SFlag};
use rclone_crypt::{cipher::Cipher, stream::EncryptedReader};
use spinoff::{spinners, Color, Spinner};
use std::{
    ffi::OsString,
    fs::{self, OpenOptions},
    io::Read,
    path::Path,
    path::PathBuf,
};

macro_rules! into_fuse_err {
    ($e: expr, $error: expr) => {
        $e.ok_or($error)?
    };
}

struct CryptFs {
    origin_path: Option<PathBuf>,
    cipher: Option<Cipher>,
}

pub fn mount(
    dir: PathBuf,
    mnt_point: PathBuf,
    password: String,
    salt: Option<String>,
    volname: Option<String>,
    _read_only: bool,
) -> i32 {
    if !dir.is_dir() {
        eprintln!("invalid directory");
        return 1;
    }

    if !mnt_point.is_dir() {
        eprintln!("invalid mount point");
        return 1;
    }

    let volname = volname.unwrap_or(gen_volume_name(&dir));
    if !volname.chars().all(|c| c.is_ascii_alphanumeric()) {
        eprintln!("invalid volume name");
        return 1;
    }

    let spinner = Spinner::new(spinners::Dots, "Creating cipher...", Color::White);
    let cipher = match Cipher::new(password, salt) {
        Ok(c) => c,
        Err(e) => {
            spinner.fail(&format!("Failed to create cipher: {e}"));
            return 1;
        }
    };
    spinner.success("Created cipher");

    let spinner = Spinner::new(spinners::Dots, "Mounting...", Color::White);
    let mnt_point = mnt_point.canonicalize().unwrap();

    let opts = vec![
        OsString::from("-s"),
        OsString::from("-f"),
        OsString::from("-d"),
        OsString::from("-o"),
        OsString::from(format!("volname={}", volname)),
        OsString::from("-o"),
        OsString::from("ro"),
    ];
    static mut FS: CryptFs = CryptFs {
        origin_path: None,
        cipher: None,
    };
    unsafe {
        FS.cipher = Some(cipher);
        FS.origin_path = Some(dir)
    };
    spinner.success("Mounted");

    unsafe {
        fuse_rs::mount(
            std::env::args_os().next().unwrap(),
            mnt_point,
            &mut FS,
            opts,
        )
    }
    .unwrap();

    0
}

fn gen_volume_name<P: AsRef<Path>>(dir: P) -> String {
    let mut volume_name = dir
        .as_ref()
        .file_stem()
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned();
    if volume_name.contains(' ') {
        volume_name = volume_name.replace(' ', "-");
    }
    volume_name
}

impl CryptFs {
    fn get_cipher(&self) -> &Cipher {
        self.cipher.as_ref().unwrap()
    }

    fn get_origin(&self) -> &PathBuf {
        self.origin_path.as_ref().unwrap()
    }

    fn real_path<P: AsRef<Path>>(&self, other: P) -> PathBuf {
        self.get_origin().join(other)
    }

    /// Returns the real path to a file/folder inside an encrypted directory.
    /// `to` - "Real" (encrypted) file/path
    /// If `to` starts with a `/`, it will be stripped.
    fn get_encrypted_path<P: AsRef<Path>>(&self, to: P) -> Option<PathBuf> {
        let to = to.as_ref().strip_prefix("/").unwrap();
        let encrypted = self.get_cipher().encrypt_path(to).ok();
        Some(self.real_path(encrypted.as_ref()?))
    }

    /// Returns the fake path to a file/folder inside an encrypted directory.
    /// `to` - "Fake" (decrypted) file/path
    /// If `to` starts with a `/`, it will be stripped.
    fn get_decrypted_path<P: AsRef<Path>>(&self, to: P) -> PathBuf {
        let to = to.as_ref().strip_prefix("/").unwrap();
        let to = self.get_cipher().decrypt_path(to).unwrap();
        self.real_path(to)
    }
}

impl Filesystem for CryptFs {
    fn metadata(&self, path: &Path) -> fuse_rs::Result<FileStat> {
        let mut stat = FileStat::new();

        match path.to_str().expect("path") {
            "/" => {
                stat.st_mode = SFlag::S_IFDIR.bits() | 0o755;
                stat.st_nlink = 3;
            }
            other => {
                let real_path = into_fuse_err!(self.get_encrypted_path(other), Errno::ENOENT);

                if real_path.is_file() {
                    stat.st_mode = SFlag::S_IFREG.bits() | 0o644;
                    stat.st_nlink = 1;
                    stat.st_size = fs::metadata(real_path).unwrap().len() as _;
                } else if real_path.is_dir() {
                    stat.st_mode = SFlag::S_IFDIR.bits() | 0o755;
                    stat.st_nlink = 1;
                } else {
                    return Err(Errno::ENOENT);
                }
            }
        }

        Ok(stat)
    }

    fn read_dir(
        &mut self,
        path: &Path,
        _offset: u64,
        _file_info: FileInfo,
    ) -> fuse_rs::Result<Vec<DirEntry>> {
        let real_path = if path == Path::new("/") {
            self.get_origin().clone()
        } else {
            into_fuse_err!(self.get_encrypted_path(path), Errno::ENOENT)
        };

        Ok(real_path
            .read_dir()
            .unwrap()
            .map(Result::unwrap)
            .map(|entry| entry.file_name().to_str().unwrap().to_owned())
            .map(|entry| self.get_decrypted_path(entry))
            .map(|entry| DirEntry {
                name: entry.into_os_string(),
                metadata: None,
                offset: None,
            })
            .collect())
    }

    fn open(&mut self, _path: &Path, file_info: &mut OpenFileInfo) -> fuse_rs::Result<()> {
        // force read-only
        if (file_info.flags().unwrap_or(OFlag::empty()) & OFlag::O_ACCMODE) != OFlag::O_RDONLY {
            return Err(Errno::EACCES);
        }

        Ok(())
    }

    fn read(
        &mut self,
        path: &Path,
        buf: &mut [u8],
        offset: u64,
        _file_info: FileInfo,
    ) -> fuse_rs::Result<usize> {
        let real_path = into_fuse_err!(self.get_encrypted_path(path), Errno::ENOENT);

        let file = OpenOptions::new().read(true).open(real_path).unwrap();
        let mut reader = EncryptedReader::new_with_cipher(file, self.get_cipher().clone()).unwrap();
        let mut content = Vec::new();

        reader.read_to_end(&mut content).unwrap();

        let offset = offset as usize;
        let cap = if offset + buf.len() > content.len() {
            content.len() - offset
        } else {
            buf.len()
        };

        (&content[offset..(offset + cap)]).read(buf).unwrap();
        Ok(cap)
    }
}
