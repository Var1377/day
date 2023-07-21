mod config;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(name = "day.rs", version = "0.1.0", author = "Varun Latthe (Var1337)", about, after_help = "Day.rs is a command line tool to help you maximise efficiency around an already packed day. To see what it can do, run `day config`0")]
struct Cli {
    #[clap(subcommand)]
    subcmd: SubCommand,
}


#[derive(Subcommand, Debug)]
enum SubCommand {
    #[clap(visible_aliases = &["cfg", "c"])]
    /// Show or change configuration values
    Config(config::ConfigArgs),
}   

fn main(){
    let cli = Cli::parse();
    match cli.subcmd {
        SubCommand::Config(cfg) => {
            let config = config::Config::new();
            println!("Config: {:?}", config);
        }
    }
}
