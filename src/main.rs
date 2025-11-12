mod cli;
mod uuidcmd;
use crate::uuidcmd::uuid_generate::generate_uuid;
use clap::Parser;
use cli::Cli;
fn main() {
    let arg = Cli::parse();
    match &arg.command {
        cli::Commands::Uuid(args) => {
            for _i in 0..args.repeat {
                println!("{}", generate_uuid(args.no_hyphen, args.version));
            }
        }
    }
}
