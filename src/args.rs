use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Display version info
    Version,
    /// List files in an encrypted directory
    Ls {
        /// Directory to list
        dir: PathBuf,
        /// Decryption password
        password: String,
        /// An optional salt
        salt: Option<String>,
    },
    /// List directories in an encrypted directory
    Lsd {
        /// Directory to list
        dir: PathBuf,
        /// Decryption password
        password: String,
        /// An optional salt
        salt: Option<String>,
    },
    /// A Unix-like cat command
    Cat {
        /// Path to encrypted directory
        dir: PathBuf,
        /// Path inside enrypted directory
        file: PathBuf,
        /// Decryption password
        password: String,
        /// An optional salt
        salt: Option<String>,
    },
    /// Copy files from/into an encrypted directory
    Cp {
        /// Path to encrypted directory
        dir: PathBuf,
        /// Path inside enrypted directory
        file: PathBuf,
        /// Destination path
        dest: PathBuf,
        /// Decryption password
        password: String,
        /// An optional salt
        salt: Option<String>,
        /// Reverse mode - Copy file into the encrypted directory
        #[arg(long, default_value_t = false)]
        reverse: bool,
    },
    /// A Unix-like head command.
    Head {
        /// Path to encrypted directory
        dir: PathBuf,
        /// Path inside enrypted directory
        file: PathBuf,
        /// Decryption password
        password: String,
        /// An optional salt
        salt: Option<String>,
        /// Amount of lines to display
        #[arg(short, default_value_t = 10)]
        n: usize,
    },
    /// A Unix-like tail command.
    Tail {
        /// Path to encrypted directory
        dir: PathBuf,
        /// Path inside enrypted directory
        file: PathBuf,
        /// Decryption password
        password: String,
        /// An optional salt
        salt: Option<String>,
        /// Amount of lines to display
        #[arg(short, default_value_t = 10)]
        n: usize,
    },
    /// Move files from/into an encrypted directory
    Mv {
        /// Path to encrypted directory
        dir: PathBuf,
        /// Path inside enrypted directory
        file: PathBuf,
        /// Destination path
        dest: PathBuf,
        /// Decryption password
        password: String,
        /// An optional salt
        salt: Option<String>,
        /// Reverse mode - Move file into the encrypted directory
        #[arg(long, default_value_t = false)]
        reverse: bool,
    },
    /// Delete a file from an encrypted directory
    Rm {
        /// Path to encrypted directory
        dir: PathBuf,
        /// Path inside enrypted directory
        file: PathBuf,
        /// Decryption password
        password: String,
        /// An optional salt
        salt: Option<String>,
    },
    /// A Unix-like touch command
    Touch {
        /// Path to encrypted directory
        dir: PathBuf,
        /// Path inside enrypted directory
        file: PathBuf,
        /// Decryption password
        password: String,
        /// An optional salt
        salt: Option<String>,
    },
    /// Read part of a file
    Read {
        /// Path to encrypted directory
        dir: PathBuf,
        /// Path inside enrypted directory
        file: PathBuf,
        /// Decryption password
        password: String,
        /// An optional salt
        salt: Option<String>,
        /// Offset where to start reading
        offset: usize,
        /// Amount of bytes to read
        amount: usize,
    },
    /// Count size of a file
    Sizeof {
        /// Path to encrypted directory
        dir: PathBuf,
        /// Path inside enrypted directory
        file: PathBuf,
        /// Decryption password
        password: String,
        /// An optional salt
        salt: Option<String>,
    },
    /// Make directories
    Mkdir {
        /// Path to encrypted directory
        dir: PathBuf,
        /// Path inside enrypted directory
        path: PathBuf,
        /// Decryption password
        password: String,
        /// An optional salt
        salt: Option<String>,
    },
    /// Encrypts/Decrypts file names and paths
    Cryptdecode {
        /// File name or path to encrypt/decrypt
        filename: PathBuf,
        /// Decryption password
        password: String,
        /// An optional salt
        salt: Option<String>,
        /// Reverse (encrypt mode)
        #[arg(long, default_value_t = false)]
        reverse: bool,
    },
    /// Obscures the given password
    Obscure {
        /// Text to obscure
        value: String,
    },
    /// Reveals an obscured password
    Reveal {
        /// Text to reveal
        value: String,
    },
    #[cfg(feature = "mount")]
    /// Mount an encrypted folder as a virtual drive
    Mount {
        /// Path to encrypted directory
        dir: PathBuf,
        /// Mount point
        mnt_point: PathBuf,
        /// Decryption password
        password: String,
        /// An optional salt
        salt: Option<String>,
        /// Name of the virtual drive
        volname: Option<String>,
        #[arg(long, default_value_t = false)]
        read_only: bool,
    },
    /// Prints the total size and number of objects in the target directory
    Size { target: PathBuf },
}
