use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(about = "Devtools for developer chores", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// generate uuid
    Uuid(UUIDArgs),
}

#[derive(Args)]
pub struct UUIDArgs {
    #[arg(short, long)]
    /// generate uuid with out hyphen
    pub no_hyphen: bool,
    #[arg(short, long, default_value_t=4, value_parser = value_in_range)]
    /// choose the version if uuid. Only 4 and 7 supported
    pub version: u8,
    #[arg(short, long, default_value_t=1, value_parser= validate_max_repeat_value)]
    /// number of uuid to be generated
    pub repeat: u32,
}

fn value_in_range(s: &str) -> Result<u8, String> {
    let accepted_version = vec![4, 7];
    let version: u32 = s.parse().expect("version is not a number");
    if accepted_version.contains(&version) {
        Ok(version as u8)
    } else {
        Err(format!("version must be either 4 or 7"))
    }
}

fn validate_max_repeat_value(s: &str) -> Result<u32, String> {
    let repeat: u32 = s.parse().expect("repeat is not a number");
    if repeat < 100 {
        Ok(repeat as u32)
    } else {
        Err(format!("repeat can be maximum 100"))
    }
}
