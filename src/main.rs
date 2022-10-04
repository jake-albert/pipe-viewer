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
use std::sync::{Arc, Mutex};
use std::thread;

fn main() -> Result<()> {
    let args = ParsedArgs::parse();
    let ParsedArgs {
        infile,
        outfile,
        silent,
    } = args;

    let quit_read = Arc::new(Mutex::new(false));
    let (quit_stats, quit_write) = (quit_read.clone(), quit_read.clone());

    let read_handle = thread::spawn(move || read::read_loop(&infile, quit_read));
    let stats_handle = thread::spawn(move || stats::stats_loop(silent, quit_stats));
    let write_handle = thread::spawn(move || write::write_loop(&outfile, quit_write));

    // crash if any threads have crashed
    let read_io_result = read_handle.join().unwrap();
    let stats_io_result = stats_handle.join().unwrap();
    let write_io_result = write_handle.join().unwrap();

    read_io_result?;
    stats_io_result?;
    write_io_result?;

    Ok(())
}
