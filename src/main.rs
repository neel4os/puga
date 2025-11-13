mod cli;
mod uuidcmd;
mod b64ed;
use crate::{cli::{B64Commands, UrlCommands}, uuidcmd::uuid_generate::generate_uuid};
use clap::Parser;
use cli::Cli;
use urlencoding::{decode, encode};
fn main() {
    let arg = Cli::parse();
    match &arg.command {
        cli::Commands::Uuid(args) => {
            for _i in 0..args.repeat {
                println!("{}", generate_uuid(args.no_hyphen, args.version));
            }
        }
        cli::Commands::Url { command } => match command {
            UrlCommands::Decode { url } => {
                let decoded_string = decode(url).expect("failed to decode");
                println!("{}", decoded_string.into_owned())
            }
            UrlCommands::Encode { url } => {
                println!("{}", encode(url).into_owned())
            }
        },
        cli::Commands::B64 { command } => match command{
            B64Commands::Encode { text } => {
               println!("{}",b64ed::b64_ops::encode(text))
            }
            B64Commands::Decode { text } => {
                println!("{}",b64ed::b64_ops::decode(text))
            }
        }
    }
}
