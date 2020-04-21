use ppa::cli::Cli;
use std::{env, fs};
use structopt::StructOpt;

fn main() {
    let test = false;

    let args: Vec<String> = if test {
        vec!["ppa".to_string(), "jaccard".to_string()]
    } else {
        env::args().collect()
    };

    let _args: Cli = if args.len() == 2 && !args[1].starts_with('-') {
        let mut content = fs::read_to_string(&args[1]).expect(&format!(
            "Something went wrong reading the options file {:?}",
            &args[1]
        ));
        content = "ppa ".to_string() + &content.replace("\r\n", " ").replace("\n", " ");
        content.parse().unwrap()
    } else {
        Cli::from_args()
    };
}
