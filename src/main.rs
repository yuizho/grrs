use std::io::{self, Write};

use exitfailure::ExitFailure;
use failure::ResultExt;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Cli {
    pattern: String,
    // PathBuf is like a String but for file system paths
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() -> Result<(), ExitFailure> {
    let args = Cli::from_args();
    let content = std::fs::read_to_string(&args.path)
        .with_context(|_| format!("could not read file {:?}", &args.path))?;
    let stdout = io::stdout();
    let mut handle = io::BufWriter::new(stdout);
    for line in content.lines() {
        if line.contains(&args.pattern) {
            writeln!(handle, "{}", line);
        }
    }
    Ok(())
}