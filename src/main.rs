use clap::{App, load_yaml};

use std::fs;

use timetrack::options::Options;
use timetrack::events;


fn main() {
    let cli_yaml = load_yaml!("timetrack.yaml");
    let args = App::from_yaml(cli_yaml).get_matches();

    let verbose = args.is_present("verbose");
    let muted = args.is_present("mute");

    let config_path = match args.value_of("config") {
        Some(path) => path,
        None => "~/.config/timetrack"
    };
    let config_content = fs::read_to_string(config_path).unwrap_or(String::from(""));
    let config = serde_json::from_str(&config_content).unwrap_or(Options::default());


    if verbose { println!("Current config content: {:?}", &config); }


    match args.subcommand() {
        ("begin", Some(_sub_args)) => { events::begin(config, verbose); }

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


        _ => panic!("Unimplemented or unsupported subcommand.")
    }
}
