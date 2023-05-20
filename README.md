# Rclone Crypt CLI

**r**clone **c**rypt **C**LI is a command-line utility which allows you to work with directories encrypted using rclone. It support over 10 commands (cat, copy, move, etc.)

I'm using a [custom fork](https://github.com/br0kenpixel/rclone-crypt-rs) of [Hidendra/rclone-crypt-rs](https://github.com/Hidendra/rclone-crypt-rs) which allows encrypting and decrypting files easily using a [Read](https://doc.rust-lang.org/std/io/trait.Read.html)-like interface.

## Example usage
```sh
# List a directory
rcc ls path/to/encrypted/dir mypass123 mysalt123

# Copy a file from an encrypted directory
rcc cp path/to/encrypted/dir some/encrypted/file.txt ~/Desktop/file.txt mypass123 mysalt123

# Copy a file into an encrypted directory
rcc cp path/to/encrypted/dir some/encrypted/file.txt ~/Desktop/file.txt mypass123 mysalt123 --reverse

# Unix-line cat command
rcc cat path/to/encrypted/dir some/encrypted/file.txt mypass123 mysalt123

# ...and more
```

## Cross-compilation
Unfortunately it seems like cross-compiling will not be possible due to sodium being used by the library.

## Supported commands
- cat
    - A unix-like `cat` command.
- copy
    - Copy files from/into encrypted directories
- head
    - A unix-like `head` command.
- ls
    - List files inside an encrypted directory
- move
    - Move files from/into encrypted directories 
- read
    - Similar to `cat` but allows specifying an start offset and an amount.
- rm
    - Deletes files
- sizeof
    - Counts the size of an encrypted file (files are decrypted first)
- tail
    - A unix-like `tail` command.
- touch
    - A unix-like `touch` command.
- mkdir
    - Creates directories

## Limitations
- Currently it is not possible to copy/move files between two encrypted directories. If you want to do this, you need to temporarily copy/move said file to a location outside the encrypted directory, then copy/move it into the destination (encrypted) directory.
    - Example:
    ```sh
    rcc cp first/encrypted/dir some/encrypted/file.txt /tmp/file.txt mypass123 mysalt123

    rcc cp second/encrypted/dir some/encrypted/file.txt /tmp/file.txt mypass123 mysalt123 --reverse
    ```