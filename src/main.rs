mod args;
mod colors;
mod config;
mod processor;

use std::env;
use crate::args::Args;

fn main() {
    let args = Args::from_app_args(env::args().collect());

    if args.has_option('h') {
        println!("Usage: loco -d DEFINITION [-h] [-l] [-w] [-c CONFIG_FILE] [FILE]\n\n-c: Location of the configuration file. Default: /etc/loco/loco.yml\n-d: Specifies the log definition, described in the configuration file\n-h: Displays this message\n-l: Colors the whole line instead of keywords\n-w: Watches for file changes\nFILE: The name of your file. If not provided, loco will listen to the standard input.");
        std::process::exit(0);
    }

    if !args.has_option('d') {
        println!("You must specify a log definition.\nloco -h to display help");
        std::process::exit(-1);
    }

    let def = config::Definition::from_config_file(
        args.get_option_value('c'),
        args.get_option_value('d').expect("You must specify a definition")
    );

    let processor = processor::FileProcessor::from_args_and_definition(&args, def);

    processor.process_file(args.argument);
}
