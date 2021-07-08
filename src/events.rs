use serde::{Serialize, Deserialize};
use chrono::prelude::*;
use parse_duration;

use std::fs;
use std::time::Duration;

use super::options::Options;


type Sheet = Vec<Event>;

pub fn begin(config: Options, verbose: bool) {
    let begin_event = Event::BEGIN(Local::now());

    let mut sheet = read_sheet(&config.timesheet);
    sheet.push(begin_event);
    write_sheet(sheet, &config.timesheet);

    if verbose { println!("Wrote begin to timesheet at {}", &config.timesheet); }
}

pub fn end(config: Options, verbose: bool) {
    let end_event = Event::END(Local::now());

    let mut sheet = read_sheet(&config.timesheet);
    sheet.push(end_event);
    write_sheet(sheet, &config.timesheet);

    if verbose { println!("Wrote end to timesheet at {}", &config.timesheet); }
}

pub fn pause(config: Options, pause_time: &str, verbose: bool) {
    let pause_duration = parse_duration::parse(pause_time)
        .expect(&format!("Unable to parse {} into a duration.", pause_time));

    let pause_event = Event::PAUSE(pause_duration);

    let mut sheet = read_sheet(&config.timesheet);
    sheet.push(pause_event);
    write_sheet(sheet, &config.timesheet);

    if verbose { println!("Wrote pause {} to timesheet at {}", &pause_time, &config.timesheet); }
}

pub fn switch(config: Options, into: String, verbose: bool) {
    // TODO Make logic that verifies if into is a valid u_name for a JobType in config.
    let switch_event = Event::SWITCH(Local::now(), into.clone());

    let mut sheet = read_sheet(&config.timesheet);
    sheet.push(switch_event);
    write_sheet(sheet, &config.timesheet);

    if verbose { println!("Wrote switch to {} to timesheet at {}", &into, &config.timesheet); }
}

pub fn nevermind(config: Options, muted: bool) {
    let mut sheet = read_sheet(&config.timesheet);
    let popped = sheet.pop();
    write_sheet(sheet, &config.timesheet);

    if !muted {println!("Removed event {:?} from timesheet {}", popped, &config.timesheet)}
}


#[derive(Serialize, Deserialize, Debug)]
enum Event {
    BEGIN(DateTime<Local>),
    END(DateTime<Local>),
    PAUSE(Duration),
    SWITCH(DateTime<Local>, String)
}


fn read_sheet(path: &str) -> Sheet {
    let sheet_str = fs::read_to_string(path)
        .expect(&format!("Unable to read timesheet file at {}", path));
    let config: Sheet = serde_json::from_str(&sheet_str)
        .expect(&format!("Timesheet file at {} with content {} was unreadable as a timesheet.", path, sheet_str));
    config
}

fn write_sheet(sheet: Sheet, path: &str) {
    let sheet_str = serde_json::to_string_pretty(&sheet).unwrap();
    fs::write(path, sheet_str).expect(&format!("Unable to write timesheet to file {}", path))
}
