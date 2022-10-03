//! Reads bytes from stdin or a given infile, and writes to stdout or a given outfile.
//!
//! TODO(jake): Determine best way to test these cases:
//! - stdin to stdout
//! ```shell
//! echo "hello" | cargo run -- > /dev/null
//! yes | cargo run -- > /dev/null
//! ```
//! - stdin to outfile
//! ```shell
//! echo "hello" | cargo run -- -o hello.txt
//! yes | cargo run -- -o yes.txt
//! ```
//! - infile to stdout
//! ```shell
//! cargo run -- file.txt > /dev/null
//! ```
//! - infile to outfile
//! ```shell
//! cargo run -- file.txt -o file2.txt
//! ```

use pipe_viewer::{args::ParsedArgs, read, stats, write};
use std::io::Result;

fn main() -> Result<()> {
    let args = ParsedArgs::parse();
    let mut total_bytes = 0;
    loop {
        let buffer = match read::read(&args.infile) {
            Ok(x) if x.is_empty() => break,
            Ok(x) => x,
            Err(_) => break,
        };
        stats::stats(args.silent, buffer.len(), &mut total_bytes, false);
        if !write::write(&args.outfile, &buffer)? {
            break;
        }
    }
    stats::stats(args.silent, 0, &mut total_bytes, true);
    Ok(())
}
