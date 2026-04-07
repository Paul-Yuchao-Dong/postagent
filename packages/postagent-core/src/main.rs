mod cli;
mod commands;
mod config;
mod error;
mod formatter;
mod token;

use clap::{CommandFactory, Parser};
use cli::{Cli, Commands};

fn main() {
    let cli = Cli::parse();

    let result = match &cli.command {
        Commands::Search { query } => commands::search::run(query, &cli.format),
        Commands::Help {
            project,
            resource,
            action,
        } => {
            let result = commands::help::run(
                project.as_deref(),
                resource.as_deref(),
                action.as_deref(),
                &cli.format,
            );
            // Handle the special "show_help" case (help with no args)
            if let Err(ref e) = result {
                if e.to_string() == "show_help" {
                    Cli::command().print_help().ok();
                    println!();
                    return;
                }
            }
            result
        }
        Commands::Auth { project } => commands::auth::run(project),
        Commands::Request {
            url,
            method,
            header,
            data,
        } => commands::request::run(url, method.as_deref(), header, data.as_deref()),
    };

    if let Err(e) = result {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
