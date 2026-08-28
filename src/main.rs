mod cli;
mod commands;
mod config;
mod error;
mod frontmatter;
mod io;
mod paths;
mod resolve;

use cli::{Action, Cmd};
use error::Result;

fn main() {
    if let Err(e) = run() {
        eprintln!("jot: {e}");
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let home = paths::jot_home()?;
    let config = config::load(&home)?;
    match cli::parse(&config) {
        Action::Static(Cmd::Init) => commands::init(&home),
        Action::Static(Cmd::List) => commands::list(&config),
        Action::Static(Cmd::Rename { from, to }) => {
            commands::rename(&home, &from, &to)
        }
        Action::Static(Cmd::Remove { key }) => commands::remove(&home, &key),
        Action::Static(Cmd::Search { key }) => commands::search(&home, &key),
        Action::Variant { key, tags } => {
            commands::create_entry(&home, &key, tags)
        }
    }
}
