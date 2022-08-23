use clap::{App, load_yaml};
use dirs;

use std::fs;

use timetrack::options::{Options, new_job, open_sheet, show_jobs};
use timetrack::events;
use timetrack::views::viewer;


fn main() {
    let cli_yaml = load_yaml!("timetrack.yaml");
    let args = App::from_yaml(cli_yaml)
        .name(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .get_matches();

    let verbose = args.is_present("verbose");

    let usr_path = dirs::home_dir()
            .expect("Unable to find homedir for user, this is needed by the config file system.")
            .to_str()
            .expect("Unable to read homedir as a str, please fix.")
            .to_owned();
    let config_default_path = &(usr_path + "/.config/timetrack");
    let config_path = match args.value_of("config") {
        Some(path) => path,
        None => config_default_path
    };
    let config_content = fs::read_to_string(config_path).unwrap_or(String::from(""));
    let config = serde_json::from_str(&config_content).unwrap_or(Options::default());


    if verbose { println!("Current config content: {:?}", &config); }


    match args.subcommand() {
        // Events
        Some(("begin", _sub_args)) => { events::begin(&config, verbose);}

        Some(("end", _sub_args)) => { events::end(&config, verbose); }

        Some(("pause", sub_args)) => {
            let pause_time = sub_args.value_of("duration").unwrap();
            events::pause(&config, pause_time, verbose);
        }

        Some(("switch", sub_args)) => {
            let into = String::from(sub_args.value_of("project").unwrap());
            events::switch(&config, into, verbose);
        }

        Some(("nevermind", _sub_args)) => { events::nevermind(&config); }

        // Output
        Some(("show", _sub_args)) => {
            let sheet = events::read_sheet(&config.timesheet);
            viewer::show(sheet, &config);
        }

        Some(("projects", _sub_args)) => { show_jobs(config) }
        
        // Manipulating config
        Some(("new", _sub_args)) => { new_job(config) }

        Some(("sheet", sub_args)) => {
            let sheet_name = sub_args.value_of("sheet_name").unwrap();
            open_sheet(config, sheet_name);
        }

        // Error handling
        _ => println!("Timetrack needs a subcommand, run \"timetrack help\" for help.")
    }
}
