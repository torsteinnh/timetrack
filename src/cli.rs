use clap::{Parser, Subcommand};


#[derive(Parser)]
#[clap(author, version, about)]
pub struct Cli {

    /// Prints extra information during run.
    #[clap(short, long, action, global = true)]
    pub verbose: bool,
    /// Optional path to other config file, default in ~/.config/timetrack.
    #[clap(short, long, value_parser, value_name = "CONFIG_FILE", global = true)]
    pub config: Option<String>,

    #[clap(subcommand)]
    pub subcommand: Subcommands
}


#[derive(Subcommand)]
pub enum Subcommands {

    /// Command for creating a start work event.
    Begin {
        /// Optional duration between the start of the day and the command was entered, positive delay implies an earlier start.
        #[clap(short, long, value_parser, value_name = "DURATION")]
        duration: Option<String>
    },

    /// Command for creating an end work event.
    End {
        /// Optional duration between the end of the day and the command was entered, positive delay implies a later end.
        #[clap(short, long, value_parser, value_name = "DURATION")]
        duration: Option<String>
    },

    /// Command for entering a pause event, such as a lunch break.
    Pause {
        /// Duration of break as a parable string, for example "30m".
        #[clap(value_parser)]
        duration: String
    },

    /// Command for switching between projects.
    Switch {
        /// Unique name of project to switch to.
        #[clap(value_parser)]
        project: String
    },

    /// Command to erase the last event, useful in case of mistyping.
    Nevermind,

    /// Command to open the new project wizard.
    New,

    /// Command to list registered projects.
    Projects,

    /// The main tool for generating nice timetracking reports.
    Show,

    /// Command for switching between different timesheets.
    Sheet {
        /// Absolute path to timesheet file, regular or empty text file.
        #[clap(value_name = "SHEET_FILE", value_parser)]
        sheet_name: String
    }
}