mod cli;
mod clipboard;
mod generator;

use cli::parse_args;
use clipboard::clip_copy;
use generator::generate_password;

fn main() {
    let cli = parse_args();
    let password = generate_password(cli.length, cli.symbols);
    clip_copy(&password);
}
