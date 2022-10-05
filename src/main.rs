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

use crossbeam::channel::{bounded, unbounded};
use pipe_viewer::{args::ParsedArgs, read, stats, write};
use std::io::Result;
use std::thread;

fn main() -> Result<()> {
    let args = ParsedArgs::parse();
    let ParsedArgs {
        infile,
        outfile,
        silent,
    } = args;

    // name of senders and receivers is based on name of *receiver*
    let (stats_tx, stats_rx) = unbounded();
    let (write_tx, write_rx) = bounded(1024);

    let read_handle = thread::spawn(move || read::read_loop(&infile, stats_tx, write_tx));
    let stats_handle = thread::spawn(move || stats::stats_loop(silent, stats_rx));
    let write_handle = thread::spawn(move || write::write_loop(&outfile, write_rx));

    // crash if any threads have crashed
    let read_io_result = read_handle.join().unwrap();
    let stats_io_result = stats_handle.join().unwrap();
    let write_io_result = write_handle.join().unwrap();

    read_io_result?;
    stats_io_result?;
    write_io_result?;

    Ok(())
}
