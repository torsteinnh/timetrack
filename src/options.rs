use serde::{Serialize, Deserialize};
use dirs;

use std::{fs, io::{self, Write}};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Options {
    pub timesheet: String,
    pub default_output: LogType,
    pub projects: Vec<JobType>,
    pub current_project: String,
    config_path: String
}

impl Default for Options {
    fn default() -> Self {
        let usr_path = dirs::home_dir()
            .expect("Unable to find homedir, cannot make default config with path to timesheet.")
            .to_str()
            .expect("Unable to read homedir as a str, please fix.")
            .to_owned();

        Options {
            timesheet: String::from(usr_path.clone() + "/Timesheet.time"),
            default_output: LogType::default(),
            projects: vec![JobType::default()],
            current_project: JobType::default().u_name,
            config_path: String::from(usr_path + "/.config/timetrack")
        }
    }
}

impl Options {
    fn save(self) {
        let options_str = serde_json::to_string_pretty(&self).unwrap();
        fs::write(self.config_path.clone(), options_str)
            .expect(&format!("Unable to write configuration to file {}", self.config_path))
    }
}


#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum LogType {
    MSDynamics
}

impl Default for LogType {
    fn default() -> Self { LogType::MSDynamics }
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JobType {
    pub u_name: String,
    pub project_id: usize,
    pub category: usize,
    pub description: String
}

impl Default for JobType {
    fn default() -> Self {
        JobType{
            u_name: String::from("example"),
            project_id: 0,
            category: 0,
            description: String::from("A phony job type for when none is given.")
        }
    }
}


pub fn new_job(mut config: Options) {
    println!("Welcome to the wizard for creating a new job/project!
A job concists of a unique name, a unique project id, a nonunique category id and a description.");

    let mut iobuff: String = "".to_string();
    
    print!("Input unique name (string): ");
    io::stdout().flush().unwrap();
    iobuff.drain(..);
    io::stdin().read_line(&mut iobuff).unwrap();
    iobuff = iobuff.trim().to_string();
    let u_name: String = iobuff.clone();
    
    print!("Input unique project ID (int): ");
    io::stdout().flush().unwrap();
    iobuff.drain(..);
    io::stdin().read_line(&mut iobuff).unwrap();
    iobuff = iobuff.trim().to_string();
    let project_id: usize = iobuff.parse().unwrap();
    
    print!("Input project category (int): ");
    io::stdout().flush().unwrap();
    iobuff.drain(..);
    io::stdin().read_line(&mut iobuff).unwrap();
    iobuff = iobuff.trim().to_string();
    let category: usize = iobuff.parse().unwrap();
    
    print!("Input descriptive string for project (string): ");
    io::stdout().flush().unwrap();
    iobuff.drain(..);
    io::stdin().read_line(&mut iobuff).unwrap();
    iobuff = iobuff.trim().to_string();
    let description: String = iobuff;
    
    let new_job = JobType{u_name, project_id, category, description};


    config.projects.push(new_job);
    config.save();
}


pub fn open_sheet(mut config: Options, sheet_name: &str) {
    config.timesheet = String::from(sheet_name);
    config.save();
}
