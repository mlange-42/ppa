//! Command line options
use std::fmt;
use std::str::FromStr;
use structopt::StructOpt;

/// Point pattern analysis command line tool
///
/// Use `ppa -h`     for help, or
///     `ppa --help` for more detailed help.
///
/// For more documentation and explanation of the algorithms, see the GitHub page:
///      https://mlange-42.github.io/ppa/
#[derive(StructOpt)]
#[structopt(verbatim_doc_comment)]
pub struct Cli {
    /// File search pattern. ** MUST be quoted on Unix systems! **
    #[structopt(short, long)]
    pattern: String,
    /// Output file name(s) prefix
    #[structopt(short, long, value_name = "path")]
    output: String,

    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(StructOpt)]
enum Command {
    /// Jaccard similarity between two sets of points
    Jaccard {
        /// Path to file with reference points
        #[structopt(name = "ref", long, short, value_name = "path")]
        reference: String,
    },
    /// Average nearest neighbor distance of a set of points
    #[structopt(name = "avg-nn")]
    AvgNN {},
}

impl FromStr for Cli {
    type Err = ParseCliError;

    /// Parses a string into a Cli.
    fn from_str(str: &str) -> Result<Self, Self::Err> {
        let quote_parts: Vec<_> = str.split('"').collect();
        let mut args: Vec<String> = vec![];
        for (i, part) in quote_parts.iter().enumerate() {
            let part = part.trim();
            if i % 2 == 0 {
                args.extend(
                    part.split(' ')
                        .map(|s| s.trim().to_string())
                        .filter(|s| !s.is_empty()),
                );
            } else {
                args.push(part.to_string());
            }
        }
        Ok(Cli::from_iter(args.iter()))
    }
}

/// Error type for failed parsing command line argument.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseCliError(String);

impl fmt::Display for ParseCliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
