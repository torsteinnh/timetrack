use chrono::prelude::*;

use std::time::Duration;
use std::collections::BTreeMap;

use super::show_default;
use crate::events::{Sheet, Event, JobIdentifier};
use crate::options::{LogType, JobType, Options};


pub type ParsedSheet = Vec<WeeksWork>;
pub type DaysWork = BTreeMap<usize, DaysProjectWork>;


pub fn show(sheet: Sheet, config: &Options) {
    let (parsed, project_identifier) = parse_sheet(sheet, config);

    match config.default_output {
        LogType::Default => show_default::show(parsed, project_identifier, config)
    }
}


fn parse_sheet(sheet: Sheet, config: &Options) -> (ParsedSheet, JobIdentifier) {
    let mut parsed_sheet: ParsedSheet = vec![];

    let mut current_week_work: WeeksWork = WeeksWork::default();
    let mut current_day_work: DaysWork = DaysWork::default();

    let mut last_dow: usize = 0; // Last parsed day of week
    let mut cpid: usize = JobType::default().internal_id; // Current logical ID

    
    // Handler logic for allowing event switching before first begin in sheet
    let mut begun: bool = false;

    for event in sheet {
        match (event, begun) {
            (Event::BEGIN(time), false) => {
                begun = true;
                current_week_work.week_number = time.iso_week().week();
                last_dow = time.weekday().num_days_from_monday() as usize;
                current_day_work.insert(cpid, DaysProjectWork { total_day: Duration::from_secs(0), start: Some(time) });
            },

            (Event::BEGIN(time), true) => {

                if time.iso_week().week() != current_week_work.week_number {
                    if current_day_work.get(&cpid).unwrap().total_day == Duration::from_secs(0) {
                        eprintln!("Work was not finished on project {} last week, work on that project that day is ignored.", cpid);
                    }

                    current_week_work.days[last_dow] = current_day_work;
                    parsed_sheet.push(current_week_work);
                    
                    current_week_work = WeeksWork::default();
                    current_week_work.week_number = time.iso_week().week();
                    current_day_work = DaysWork::default();
                    current_day_work.insert(cpid, DaysProjectWork { total_day: Duration::from_secs(0), start: Some(time) });

                    last_dow = time.weekday().num_days_from_monday() as usize;

                } else if time.weekday().num_days_from_monday() as usize != last_dow {
                    if current_day_work.get(&cpid).unwrap().total_day == Duration::from_secs(0) {
                        eprintln!("Work was not finished on project {} last day, work on that project that day is ignored.", cpid);
                    }

                    current_week_work.days[last_dow] = current_day_work;
                    current_day_work = DaysWork::default();
                    current_day_work.insert(cpid, DaysProjectWork { total_day: Duration::from_secs(0), start: Some(time) });

                    last_dow = time.weekday().num_days_from_monday() as usize;

                } else {
                    current_day_work.get_mut(&cpid).unwrap().start = Some(time - chrono::Duration::from_std(current_day_work.get(&cpid).unwrap().total_day).unwrap());
                }
            },

            (Event::SWITCH(_, job_id), false) => {
                cpid = job_id.get_jobtype(config).unwrap().internal_id;
            },

            (Event::SWITCH(time, job_id), true) => {
                if job_id.get_jobtype(config).unwrap().internal_id == cpid { continue; }

                if time.iso_week().week() != current_week_work.week_number {
                    if current_day_work.get(&cpid).unwrap().total_day == Duration::from_secs(0) {
                        eprintln!("Work was not finished on project {} last week, work on that project that day is ignored.", cpid);
                    }

                    current_week_work.days[last_dow] = current_day_work;
                    parsed_sheet.push(current_week_work);

                    cpid = job_id.get_jobtype(config).unwrap().internal_id;
                    
                    current_week_work = WeeksWork::default();
                    current_week_work.week_number = time.iso_week().week();
                    current_day_work = DaysWork::default();
                    current_day_work.insert(cpid, DaysProjectWork { total_day: Duration::from_secs(0), start: Some(time) });

                    last_dow = time.weekday().num_days_from_monday() as usize;

                } else if time.weekday().num_days_from_monday() as usize != last_dow {
                    if current_day_work.get(&cpid).unwrap().total_day == Duration::from_secs(0) {
                        eprintln!("Work was not finished on project {} last day, work on that project that day is ignored.", cpid);
                    }

                    current_week_work.days[last_dow] = current_day_work;

                    cpid = job_id.get_jobtype(config).unwrap().internal_id;

                    current_day_work = DaysWork::default();
                    current_day_work.insert(cpid, DaysProjectWork { total_day: Duration::from_secs(0), start: Some(time) });

                    last_dow = time.weekday().num_days_from_monday() as usize;

                } else {
                    current_day_work.get_mut(&cpid).unwrap().total_day = (time - current_day_work.get(&cpid).unwrap().start.unwrap()).to_std().unwrap();

                    cpid = job_id.get_jobtype(config).unwrap().internal_id;

                    if let Some(sofar) = current_day_work.get_mut(&cpid) {
                        if sofar.total_day == Duration::from_secs(0) {
                            eprintln!("Work was not finished on project {} this day, work on this project today is ignored.", cpid);
                        }
                        sofar.start = Some(time - chrono::Duration::from_std(sofar.total_day).unwrap());
                        sofar.total_day = Duration::from_secs(0);
                    } else {
                        current_day_work.insert(cpid, DaysProjectWork { total_day: Duration::from_secs(0), start: Some(time) });
                    }
                }
            }

            (Event::END(time), true) => {
                if time.iso_week().week() != current_week_work.week_number || time.weekday().num_days_from_monday() as usize != last_dow {
                    panic!("Illegal end on start of new day")
                } else {
                    current_day_work.get_mut(&cpid).unwrap().total_day = (time - current_day_work.get(&cpid).unwrap().start.unwrap()).to_std().unwrap();
                }
            },

            (Event::PAUSE(interval), true) => {
                let project_start = current_day_work.get(&cpid).unwrap().start.unwrap();
                current_day_work.get_mut(&cpid).unwrap().start = Some(project_start + chrono::Duration::from_std(interval).expect(&format!("Pause duration {:?} is not a valid chrono duration", interval)));
            },

            (illegal_event @ (Event::END(_) | Event::PAUSE(_)), false) => { panic!("Illegal event {} before event BEGIN in timesheet.", illegal_event) }
        }
    };

    let current_time = Local::now();
    if current_week_work.week_number == current_time.iso_week().week() && last_dow == current_time.weekday().num_days_from_monday() as usize && current_day_work.get(&cpid).unwrap().total_day == Duration::from_secs(0) {
        current_day_work.get_mut(&cpid).unwrap().total_day = (current_time - current_day_work.get(&cpid).unwrap().start.unwrap()).to_std().unwrap();
    }

    current_week_work.days[last_dow] = current_day_work;
    parsed_sheet.push(current_week_work);
    (parsed_sheet, JobIdentifier::InternalId(cpid))
}


#[derive(Default, Clone, Copy)]
pub struct DaysProjectWork {
    pub total_day: Duration,
    start: Option<DateTime<Local>>
}


#[derive(Default, Clone)]
pub struct WeeksWork {
    pub days: [DaysWork; 7],
    pub week_number: u32
}

impl WeeksWork {
    pub fn transpose(self) -> TransposedWeeksWork {
        let mut transposed = TransposedWeeksWork::default();
        transposed.week_number = self.week_number;

        for day in 0..7 as usize{
            let mut total = Duration::from_secs(0);
            for (cpid, project_day) in self.days[day].iter() {
                total += project_day.total_day;
                if let Some(project_week) = transposed.projects.get_mut(cpid) {
                    project_week.days[day] = *project_day;
                } else {
                    transposed.projects.insert(*cpid, TransposedWeeksProjectWork::default());
                    transposed.projects.get_mut(cpid).unwrap().days[day] = *project_day;
                }
            }
            transposed.total_time += total;
            transposed.total.days[day].total_day = total;
        }

        transposed
    }
}


#[derive(Default)]
pub struct TransposedWeeksWork {
    pub total: TransposedWeeksProjectWork,
    pub total_time: Duration,
    pub projects: BTreeMap<usize, TransposedWeeksProjectWork>,
    pub week_number: u32
}

#[derive(Default)]
pub struct TransposedWeeksProjectWork {
    pub days: [DaysProjectWork; 7]
}
