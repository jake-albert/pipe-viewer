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

use clap::Parser;
use std::env;
use std::fs::File;
use std::io::{self, BufReader, BufWriter, ErrorKind, Read, Result, Write};

// 16 KiB to begin with. Adjust later if needed.
const CHUNK_SIZE: usize = 16 * 1024;

fn main() -> Result<()> {
    let (mut reader, mut writer, silent) = process_args()?;

    let mut total_bytes = 0;
    let mut buffer = [0; CHUNK_SIZE];
    loop {
        let num_read = match reader.read(&mut buffer) {
            Ok(0) => break,
            Ok(x) => x,
            Err(_) => break,
        };
        total_bytes += num_read;
        if !silent {
            eprint!("\r{}", total_bytes);
        }
        if let Err(e) = writer.write_all(&buffer[..num_read]) {
            if e.kind() == ErrorKind::BrokenPipe {
                break;
            }
            return Err(e);
        }
    }

    Ok(())
}

#[derive(Parser, Debug)]
struct Args {
    #[arg(help = "Read from a file instead of stdin")]
    infile: Option<String>,

    #[arg(short, long, help = "Write output to a file instead of stdout")]
    outfile: Option<String>,

    #[arg(short, long, help = "Prevent messages in stderr")]
    silent: bool,
}

type ParsedArgs = (Box<dyn std::io::Read>, Box<dyn std::io::Write>, bool);

fn process_args() -> Result<ParsedArgs> {
    let args = Args::parse();
    let infile = args.infile.unwrap_or_default();
    let outfile = args.outfile.unwrap_or_default();
    let silent = if args.silent {
        true
    } else {
        !env::var("PV_SILENT").unwrap_or_default().is_empty()
    };

    let reader: Box<dyn Read> = if !infile.is_empty() {
        Box::new(BufReader::new(File::open(infile)?))
    } else {
        Box::new(BufReader::new(io::stdin()))
    };

    let writer: Box<dyn Write> = if !outfile.is_empty() {
        Box::new(BufWriter::new(File::create(outfile)?))
    } else {
        Box::new(BufWriter::new(io::stdout()))
    };

    Ok((reader, writer, silent))
}
