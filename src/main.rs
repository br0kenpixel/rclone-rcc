#![allow(
    clippy::cast_precision_loss,
    clippy::needless_pass_by_value,
    clippy::cast_possible_wrap
)]
mod args;
mod commands;
mod macros;

use crate::args::Commands;
use args::Cli;
use clap::Parser;
use std::process::exit;

fn main() {
    let args = Cli::parse();

    let exit_code = match args.command {
        Commands::Version => {
            println!("rcc v.{}", env!("CARGO_PKG_VERSION"));
            0
        }
        Commands::Ls {
            dir,
            password,
            salt,
        } => commands::ls(dir, password, salt),
        Commands::Lsd {
            dir,
            password,
            salt,
        } => commands::lsd(dir, password, salt),
        Commands::Cat {
            dir,
            file,
            password,
            salt,
        } => commands::cat(dir, file, password, salt),
        Commands::Cp {
            dir,
            file,
            dest,
            password,
            salt,
            reverse,
        } => commands::cp(dir, file, dest, password, salt, reverse, false),
        Commands::Head {
            dir,
            file,
            password,
            salt,
            n,
        } => commands::head(dir, file, password, salt, n),
        Commands::Tail {
            dir,
            file,
            password,
            salt,
            n,
        } => commands::tail(dir, file, password, salt, n),
        Commands::Mv {
            dir,
            file,
            dest,
            password,
            salt,
            reverse,
        } => commands::move_(dir, file, dest, password, salt, reverse),
        Commands::Rm {
            dir,
            file,
            password,
            salt,
        } => commands::rm(dir, file, password, salt),
        Commands::Touch {
            dir,
            file,
            password,
            salt,
        } => commands::touch(dir, file, password, salt),
        Commands::Read {
            dir,
            file,
            password,
            salt,
            offset,
            amount,
        } => commands::read(dir, file, password, salt, offset, amount),
        Commands::Sizeof {
            dir,
            file,
            password,
            salt,
        } => commands::sizeof(dir, file, password, salt),
        Commands::Mkdir {
            dir,
            path,
            password,
            salt,
        } => commands::mkdir(dir, path, password, salt),
        Commands::Cryptdecode {
            filename,
            password,
            salt,
            reverse,
        } => commands::cryptdecode(filename, password, salt, reverse),
        Commands::Obscure { value } => commands::obscure(value),
        Commands::Reveal { value } => commands::reveal(value),
        #[cfg(feature = "mount")]
        Commands::Mount {
            dir,
            mnt_point,
            password,
            salt,
            volname,
            read_only: _,
        } => commands::mount(dir, mnt_point, password, salt, volname, true),
        Commands::Size { target } => commands::size(target),
    };

    exit(exit_code);
}
