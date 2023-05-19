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
}
