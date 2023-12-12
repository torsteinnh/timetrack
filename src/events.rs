use serde::{Serialize, Deserialize};
use chrono::prelude::*;
use parse_duration;

use std::fs;
use std::time::Duration;
use std::process::exit;
use std::fmt;

use crate::options::{JobType, Options};


pub type Sheet = Vec<Event>;

fn check_begun(sheet: &Sheet, time: Option<DateTime<Local>>) -> Option<bool> {
    let now = time.unwrap_or(Local::now());
    for event in sheet.iter().rev() {
        match event {
            Event::BEGIN(time) => {
                if time < &now { return Some(true); } else { continue; };
            },
            Event::END(time) => {
                if time < &now { return Some(false); } else { continue; };
            },
            _ => continue
        }
    }
    None
}


pub fn begin(config: &Options, duration: &Option<String>, verbose: bool) {
    let offset = if let Some(str_duration) = duration {
        chrono::Duration::from_std(parse_duration::parse(str_duration).expect(&format!("Unable to parse {} into a duration.", str_duration))).unwrap()
    } else {
        chrono::Duration::from_std(parse_duration::parse("0m").unwrap()).unwrap()
    };
    
    let begin_event = Event::BEGIN(Local::now() - offset);
    let mut sheet = read_sheet(&config.timesheet);
    
    if let Some(true) = check_begun(&sheet, Some(Local::now() - offset)) {
        eprintln!("Illegal event BEGIN while sheet in begun state, event not written.");
        exit(1);
    }

    sheet.push(begin_event);
    write_sheet(sheet, &config.timesheet);

    if verbose { println!("Wrote begin to timesheet at {}", &config.timesheet); }
}

pub fn end(config: &Options, duration: &Option<String>, verbose: bool) {
    let offset = if let Some(str_duration) = duration {
        chrono::Duration::from_std(parse_duration::parse(str_duration).expect(&format!("Unable to parse {} into a duration.", str_duration))).unwrap()
    } else {
        chrono::Duration::from_std(parse_duration::parse("0m").unwrap()).unwrap()
    };

    let end_event = Event::END(Local::now() + offset);

    let mut sheet = read_sheet(&config.timesheet);

    if let Some(false) = check_begun(&sheet, Some(Local::now() + offset)) {
        eprintln!("Illegal event END while sheet in ended state, event not written.");
        exit(1);
    }
    
    sheet.push(end_event);
    write_sheet(sheet, &config.timesheet);

    if verbose { println!("Wrote end to timesheet at {}", &config.timesheet); }
}

pub fn pause(config: &Options, pause_time: &str, verbose: bool) {
    let pause_duration = parse_duration::parse(pause_time)
        .expect(&format!("Unable to parse {} into a duration.", pause_time));

    let pause_event = Event::PAUSE(pause_duration);

    let mut sheet = read_sheet(&config.timesheet);

    if let Some(false) = check_begun(&sheet, None) {
        eprintln!("Illegal event PAUSE while sheet in ended state, event not written.");
        exit(1);
    }

    sheet.push(pause_event);
    write_sheet(sheet, &config.timesheet);

    if verbose { println!("Wrote pause {} to timesheet at {}", &pause_time, &config.timesheet); }
}

pub fn switch(config: &Options, into: String, verbose: bool) {
    let job_id = match into.clone().parse::<usize>() {
        Ok(id) => JobIdentifier::InternalId(id),
        Err(_) => JobIdentifier::UName(into.clone())
    };

    if let None = job_id.get_jobtype(config) {
        eprintln!("Could not find project identified by {}. Try creating job first with \"timetrack new\" or list projects with \"timetrack projects\". Event not written.", &into);
        exit(1);
    }

    let switch_event = Event::SWITCH(Local::now(), job_id);

    let mut sheet = read_sheet(&config.timesheet);
    sheet.push(switch_event);
    write_sheet(sheet, &config.timesheet);

    if verbose { println!("Wrote switch to {} to timesheet at {}", &into, &config.timesheet); }
}

pub fn nevermind(config: &Options) {
    let mut sheet = read_sheet(&config.timesheet);
    let popped = sheet.pop();
    write_sheet(sheet, &config.timesheet);

    println!("Removed event {:?} from timesheet {}", popped, &config.timesheet)
}


#[derive(Serialize, Deserialize, Debug)]
pub enum Event {
    BEGIN(DateTime<Local>),
    END(DateTime<Local>),
    PAUSE(Duration),
    SWITCH(DateTime<Local>, JobIdentifier)
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::BEGIN(..) => write!(f, "BEGIN"),
            Self::END(..) => write!(f, "END"),
            Self::PAUSE(..) => write!(f, "PAUSE"),
            Self::SWITCH(..) => write!(f, "SWITCH")
        }
    }
}

impl PartialEq for Event {
    fn eq(&self, other: &Self) -> bool {
        use Event::*;
        
        match (&self, other) {
            (BEGIN(_), BEGIN(_)) => true,
            (END(_), END(_)) => true,
            (PAUSE(_), PAUSE(_)) => true,
            (SWITCH(_, _), SWITCH(_, _)) => true,
            _ => false
        }
    }
}


#[derive(Serialize, Deserialize, Debug)]
pub enum JobIdentifier {
    UName(String),
    InternalId(usize)
}

impl JobIdentifier {
    pub fn get_jobtype(&self, config: &Options) -> Option<JobType> {
        for job in &config.projects {
            match self {
                Self::UName(name) => { if name == &job.u_name { return Some(job.clone()) } },
                Self::InternalId(id) => { if id == &job.internal_id { return Some(job.clone()) } }
            }
        }
        None
    }
}


pub fn read_sheet(path: &str) -> Sheet {
    let sheet_str = fs::read_to_string(path)
        .unwrap_or(String::from("[]"));
    let config: Sheet = serde_json::from_str(&sheet_str)
        .expect(&format!("Timesheet file at {} with content {} was unreadable as a timesheet.", path, sheet_str));
    config
}

fn write_sheet(sheet: Sheet, path: &str) {
    let sheet_str = serde_json::to_string_pretty(&sheet).unwrap();
    fs::write(path, sheet_str).expect(&format!("Unable to write timesheet to file {}", path))
}
