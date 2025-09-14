mod cli;
mod generator;

use cli::parse_args;
use generator::generate_password;

fn main() {
    let cli = parse_args();
    let password = generate_password(cli.length, cli.symbols);

    println!("{}", password)
}
