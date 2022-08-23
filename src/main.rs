use clap::Parser;
use dirs;

use std::fs;

use timetrack::cli::{Cli, Subcommands};
use timetrack::options::{Options, new_job, open_sheet, show_jobs};
use timetrack::events;
use timetrack::views::viewer;


fn main() {
    let args = Cli::parse();

    let verbose = args.verbose;

    let usr_path = dirs::home_dir()
            .expect("Unable to find homedir for user, this is needed by the config file system.")
            .to_str()
            .expect("Unable to read homedir as a str, please fix.")
            .to_owned();
    let config_default_path = usr_path + "/.config/timetrack";
    let config_path = match args.config {
        Some(path) => path,
        None => config_default_path
    };
    let config_content = fs::read_to_string(config_path).unwrap_or(String::from(""));
    let config = serde_json::from_str(&config_content).unwrap_or(Options::default());


    if verbose { println!("Current config content: {:?}", &config); }


    match args.subcommand {
        // Events
        Subcommands::Begin => { events::begin(&config, verbose);}

        Subcommands::End => { events::end(&config, verbose); }

        Subcommands::Pause { duration } => {
            events::pause(&config, &duration, verbose);
        }

        Subcommands::Switch { project } => {
            events::switch(&config, project, verbose);
        }

        Subcommands::Nevermind => { events::nevermind(&config); }

        // Output
        Subcommands::Show => {
            let sheet = events::read_sheet(&config.timesheet);
            viewer::show(sheet, &config);
        }

        Subcommands::Projects => { show_jobs(config) }
        
        // Manipulating config
        Subcommands::New => { new_job(config) }

        Subcommands::Sheet { sheet_name } => {
            open_sheet(config, &sheet_name);
        }
    }
}
