use clap::Parser;
use std::io::{self, Write};

/// Einfache Struktur für das Marco Polo Spiel
#[derive(Parser)]
#[command(name = "hello-marco")]
struct Args {
    /// Name des Spielers
    #[arg(short, long)]
    name: Option<String>,
}

fn main() {
    let args = Args::parse();
    let name = match args.name {
        Some(name) => name,
        None => {
            print!("Wie heißt du? ");
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            input.trim().to_string()
        }
    };

    let result = hello_marco::marco_polo(&name);
    println!("Hallo, {}! {}", name, result);
}
