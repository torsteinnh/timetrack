use serde::{Serialize, Deserialize};
use dirs;


#[derive(Serialize, Deserialize, Debug)]
pub struct Options {
    pub timesheet: String,
    pub default_output: LogType,
    pub projects: Vec<JobType>,
    pub current_project: String
}

impl Default for Options {
    fn default() -> Self {
        let usr_path = dirs::home_dir()
            .expect("Unable to find homedir, cannot make default config with path to timesheet.")
            .to_str()
            .expect("Unable to read homedir as a str, please fix.")
            .to_owned();

        Options {
            timesheet: String::from(usr_path + "/Timesheet.time"),
            default_output: LogType::default(),
            projects: vec![JobType::default()],
            current_project: JobType::default().u_name
        }
    }
}


#[derive(Serialize, Deserialize, Debug)]
pub enum LogType {
    MSDynamics
}

impl Default for LogType {
    fn default() -> Self { LogType::MSDynamics }
}


#[derive(Serialize, Deserialize, Debug)]
pub struct JobType {
    u_name: String,
    project_id: usize,
    category: usize,
    description: String
}

impl Default for JobType {
    fn default() -> Self {
        JobType{
            u_name: String::from("example"),
            project_id: 0,
            category: 0,
            description: String::from("A phony job type for illustration purposes.")
        }
    }
}