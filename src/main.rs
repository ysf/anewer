use anyhow::{anyhow, Context, Result};
use std::borrow::Cow;
use std::collections::HashSet;
use std::fs;
use std::fs::OpenOptions;
use std::io::{self, BufRead, Write};
use std::path::PathBuf;
use structopt::clap::AppSettings;
use structopt::StructOpt;

/// Appends lines from stdin to a file, if they don't already exist in the file.
#[derive(Debug, StructOpt)]
#[structopt(global_settings = &[AppSettings::ColoredHelp])]
struct Args {
    /// path to file, will be created if needed
    filename: Option<PathBuf>,

    /// Quiet, won't print to stdout.
    #[structopt(short, long)]
    quiet: bool,

    /// Dry run, will leave the file as it is.
    #[structopt(short = "n", long)]
    dry_run: bool,
}

fn main() -> Result<()> {
    let content;
    let args = Args::from_args();
    let mut file = None;
    let mut refs = HashSet::new();

    if let Some(filename) = args.filename {
        content = fs::read_to_string(&filename)
            .with_context(|| anyhow!("Failed to open file: {:?}", filename))?;

        let has_newline = content.ends_with('\n');

        refs.extend(content.split('\n').map(Cow::Borrowed));

        if !args.dry_run {
            let mut file2 = OpenOptions::new()
                .create(true)
                .write(true)
                .append(true)
                .open(filename)?;
            if !has_newline {
                file2.write_all(b"\n")?;
            }

            file = Some(file2);
        }
    }

    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        let line = line?;

        if refs.contains(line.as_str()) {
            continue;
        }

        if let Some(file) = &mut file {
            file.write_all(&format!("{}\n", line).as_bytes())?;
        }

        if !args.quiet {
            println!("{}", line);
        }

        refs.insert(Cow::Owned(line));
    }

    Ok(())
}
