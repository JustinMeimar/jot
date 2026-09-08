use crate::config::Config;
use clap::{ArgAction, CommandFactory, FromArgMatches, Parser, Subcommand};

pub const RESERVED_NAMES: &[&str] =
    &["init", "list", "open", "rename", "remove", "search"];

#[derive(Parser)]
#[command(name = "jot", about = "dated note taker with configurable variants")]
struct RawCli {
    #[command(subcommand)]
    command: Cmd,
}

#[derive(Subcommand)]
pub enum Cmd {
    #[command(about = "initialize ~/.jot")]
    Init,
    #[command(about = "list variants and their most recent notes")]
    List {
        #[arg(short = 'k', long = "limit", default_value_t = 5, value_name = "K")]
        limit: usize,
    },
    #[command(about = "open ~/.jot in $EDITOR")]
    Open,
    #[command(about = "rename a variant")]
    Rename { from: String, to: String },
    #[command(about = "remove a variant")]
    Remove { key: String },
    #[command(about = "search entries by tag or content")]
    Search { key: String },
}

pub enum Action {
    Static(Cmd),
    Variant { key: String, tags: Vec<String> },
}

pub fn parse(config: &Config) -> Action {
    let mut app = RawCli::command();
    for (key, variant) in &config.jot {
        let mut sub = clap::Command::new(key.clone())
            .visible_alias(variant.subcommand.clone())
            .arg(
                clap::Arg::new("tag")
                    .short('t')
                    .long("tag")
                    .value_delimiter(',')
                    .action(ArgAction::Append)
                    .help("tag the entry (repeatable or comma-separated)"),
            );
        if let Some(desc) = &variant.description {
            sub = sub.about(desc.clone());
        }
        app = app.subcommand(sub);
    }
    let matches = app.get_matches();
    let name = matches
        .subcommand_name()
        .expect("subcommand_required")
        .to_string();
    if RESERVED_NAMES.contains(&name.as_str()) {
        let raw = RawCli::from_arg_matches(&matches).expect("static decode");
        Action::Static(raw.command)
    } else {
        let sub_matches = matches
            .subcommand_matches(&name)
            .expect("subcommand matched");
        let tags: Vec<String> = sub_matches
            .get_many::<String>("tag")
            .map(|vs| vs.cloned().collect())
            .unwrap_or_default();
        Action::Variant { key: name, tags }
    }
}
