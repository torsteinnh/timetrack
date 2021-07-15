use clap::{App, load_yaml};
use dirs;

use std::fs;

use timetrack::options::{Options, new_job, open_sheet};
use timetrack::events;
use timetrack::views::viewer;


fn main() {
    let cli_yaml = load_yaml!("timetrack.yaml");
    let args = App::from_yaml(cli_yaml).get_matches();

    let verbose = args.is_present("verbose");
    let muted = args.is_present("mute");

    let usr_path = dirs::home_dir()
            .expect("Unable to find homedir, cannot make default config with path to timesheet.")
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
        ("begin", Some(_sub_args)) => { events::begin(config, verbose);}

        ("end", Some(_sub_args)) => { events::end(config, verbose); }

        ("pause", Some(sub_args)) => {
            let pause_time = sub_args.value_of("duration").unwrap();
            events::pause(config, pause_time, verbose);
        }

        ("switch", Some(sub_args)) => {
            let into = String::from(sub_args.value_of("project").unwrap());
            events::switch(config, into, verbose);
        }

        ("nevermind", Some(_sub_args)) => { events::nevermind(config, muted); }

        // Output
        ("show", Some(_sub_args)) => {
            let sheet = events::read_sheet(&config.timesheet);
            viewer::show(sheet, config.default_output);
        }
        
        // Manipulating config
        ("new", Some(_sub_args)) => { new_job(config) }

        ("sheet", Some(sub_args)) => {
            let sheet_name = sub_args.value_of("sheet_name").unwrap();
            open_sheet(config, sheet_name);
        }

        // Error handling
        _ => unreachable!()
    }
}
