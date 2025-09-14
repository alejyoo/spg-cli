use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    #[arg(short, long, default_value_t = 12)]
    pub length: usize,

    #[arg(short, long, default_value_t = true)]
    pub symbols: bool,
}

pub fn parse_args() -> Cli {
    Cli::parse()
}
