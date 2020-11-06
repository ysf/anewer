use anyhow::{anyhow, Context, Result};
use memchr::memchr;
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
        content =
            fs::read(&filename).with_context(|| anyhow!("Failed to open file: {:?}", filename))?;

        let has_newline = !content.is_empty() && content[content.len() - 1] == b'\n';

        let mut remaining = &content[..];
        loop {
            if let Some(idx) = memchr(b'\n', remaining) {
                refs.insert(Cow::Borrowed(&remaining[..idx]));
                remaining = &remaining[idx + 1..];
            } else {
                if !remaining.is_empty() {
                    refs.insert(Cow::Borrowed(remaining));
                }
                break;
            }
        }

        if !args.dry_run {
            let mut file2 = OpenOptions::new()
                .create(true)
                .write(true)
                .append(true)
                .open(filename)
                .context("Could not create/write/open file")?;
            if !has_newline {
                file2.write_all(b"\n")?;
            }

            file = Some(file2);
        }
    }

    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    let mut stdout = io::stdout();

    loop {
        let mut line = Vec::new();
        let mut n = stdin.read_until(b'\n', &mut line)?;

        if n == 0 {
            break;
        }

        if !line[n - 1] == b'\n' {
            n += 1;
            line.push(b'\n')
        }

        let slice = &line[..n - 1];
        if !refs.contains(slice) {
            if let Some(file) = &mut file {
                file.write_all(&line).context("Could not write to file")?;
            }

            if !args.quiet && stdout.write_all(&line).is_err() {
                break;
            }

            line.pop();
            refs.insert(Cow::Owned(line));
        }
    }

    Ok(())
}
