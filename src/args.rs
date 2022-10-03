use clap::Parser;
use std::env;

#[derive(Parser, Debug)]
struct Args {
    #[arg(help = "Read from a file instead of stdin")]
    infile: Option<String>,

    #[arg(short, long, help = "Write output to a file instead of stdout")]
    outfile: Option<String>,

    #[arg(short, long, help = "Prevent messages in stderr")]
    silent: bool,
}

pub struct ParsedArgs {
    pub infile: String,
    pub outfile: String,
    pub silent: bool,
}

impl ParsedArgs {
    pub fn parse() -> Self {
        let args = Args::parse();
        let infile = args.infile.unwrap_or_default();
        let outfile = args.outfile.unwrap_or_default();
        let silent = if args.silent {
            true
        } else {
            !env::var("PV_SILENT").unwrap_or_default().is_empty()
        };

        Self {
            infile,
            outfile,
            silent,
        }
    }
}
