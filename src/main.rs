use anyhow::{anyhow, Context, Result};
use std::borrow::Cow;
use std::collections::HashSet;
use std::fs;
use std::fs::OpenOptions;
use std::io::{self, Write};
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
                .open(filename)
                .context("Could not create/write/open file")?;
            if !has_newline {
                file2.write_all(b"\n")?;
            }

            file = Some(file2);
        }
    }

    let stdin = io::stdin();
    let mut stdout = io::stdout();

    let mut line = String::new();

    loop {
        let mut n = stdin.read_line(&mut line)?;

        if n == 0 {
            break;
        }

        if !line.ends_with('\n') {
            n += 1;
            line.push('\n')
        }

        let slice = &line[..n - 1];
        if !refs.contains(slice) {
            if let Some(file) = &mut file {
                file.write_all(line.as_bytes())
                    .context("Could not write to file")?;
            }

            if !args.quiet && stdout.write_all(line.as_bytes()).is_err() {
                break;
            }

            refs.insert(Cow::Owned(slice.to_owned()));
        }
        line.clear();
    }

    Ok(())
}
