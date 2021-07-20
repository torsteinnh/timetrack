use chrono::prelude::*;

use std::time::Duration;
use std::collections::HashMap;

use super::show_msdynamics;
use crate::events::{Sheet, Event};
use crate::options::{LogType, JobType, Options};


pub type ParsedSheet = Vec<WeeksWork>;


pub fn show(sheet: Sheet, config: Options) {
    let (parsed, config) = parse_sheet(sheet, config);

    match config.default_output {
        LogType::MSDynamics => show_msdynamics::show(parsed, config)
    }
}


fn parse_sheet(mut sheet: Sheet, config: Options) -> (ParsedSheet, Options) {
    let mut parsed_sheet = vec![];

    let mut current_week_work = WeeksWork::default();
    let mut current_project: JobType = JobType::default();

    let mut dow: usize = 0;
    let mut cpid: &usize = &current_project.project_id;

    
    let mut head = 0;
    for event in &mut sheet {
        head += 1;
        match event {
            Event::BEGIN(time) => {
                dow = time.weekday().num_days_from_monday() as usize;

                current_week_work.week_number = time.iso_week().week();
                current_week_work.total.days[dow].start = Some(*time);
                
                current_week_work.projects.insert(current_project.project_id, current_week_work.total);
                break
            }

            Event::SWITCH(_, job_id) => {
                current_project = job_id.get_jobtype(&config).unwrap();
                cpid = &current_project.project_id;
            }

            _ => panic!("Illegal event other than switch before begin.")
        }
    }
    sheet.drain(0..head);


    for event in sheet {
        match event {
            Event::BEGIN(time) => {
                dow = time.weekday().num_days_from_monday() as usize;

                if time.iso_week().week() != current_week_work.week_number {
                    parsed_sheet.push(current_week_work);
                    current_week_work = WeeksWork::default();
                    current_week_work.week_number = time.iso_week().week();
                    current_week_work.projects.insert(*cpid, WeeksProjectWork::default());
                }


                if let Some(end_time) = current_week_work.total.days[dow].end {
                    current_week_work.total.days[dow].breaks += (time - end_time).to_std().unwrap();
                    current_week_work.total.days[dow].end = None;
                    
                    let project_end_time = current_week_work.projects.get_mut(cpid).unwrap().days[dow].end.unwrap();
                    current_week_work.projects.get_mut(cpid).unwrap().days[dow].breaks += (time - project_end_time).to_std().unwrap();
                    current_week_work.projects.get_mut(cpid).unwrap().days[dow].end = None;
                } else {
                    current_week_work.total.days[dow].start = Some(time);

                    current_week_work.projects.get_mut(cpid).unwrap().days[dow].start = Some(time);
                }
            },

            Event::END(time) => {
                dow = time.weekday().num_days_from_monday() as usize;

                current_week_work.total.days[dow].end = Some(time);
                current_week_work.total.days[dow].total_day =
                    (time - current_week_work.total.days[dow].start.unwrap()).to_std().unwrap()
                    - current_week_work.total.days[dow].breaks;
                
                current_week_work.projects.get_mut(cpid).unwrap().days[dow].end = Some(time);
                current_week_work.projects.get_mut(cpid).unwrap().days[dow].total_day =
                    (time - current_week_work.projects.get_mut(cpid).unwrap().days[dow].start.unwrap()).to_std().unwrap()
                    - current_week_work.projects.get_mut(cpid).unwrap().days[dow].breaks;
            },

            Event::PAUSE(interval) => {
                current_week_work.total.days[dow].breaks += interval;
                current_week_work.projects.get_mut(cpid).unwrap().days[dow].breaks += interval;
            },

            Event::SWITCH(time, job_id) => {
                dow = time.weekday().num_days_from_monday() as usize;
                
                current_week_work.projects.get_mut(cpid).unwrap().days[dow].end = Some(time);
                current_week_work.projects.get_mut(cpid).unwrap().days[dow].total_day =
                    (time - current_week_work.projects.get_mut(cpid).unwrap().days[dow].start.unwrap()).to_std().unwrap()
                    - current_week_work.projects.get_mut(cpid).unwrap().days[dow].breaks;

                current_project = job_id.get_jobtype(&config).expect(&format!("No project found for id {:?}", job_id));
                cpid = &current_project.project_id;
                current_week_work.projects.insert(*cpid, WeeksProjectWork::default());

                current_week_work.projects.get_mut(cpid).unwrap().days[dow].start = Some(time);
            }
        }
    };

    parsed_sheet.push(current_week_work);
    (parsed_sheet, config)
}


#[derive(Default, Clone, Copy)]
pub struct DaysWork {
    pub total_day: Duration,
    start: Option<DateTime<Local>>,
    end: Option<DateTime<Local>>,
    breaks: Duration
}


#[derive(Default, Clone, Copy)]
pub struct WeeksProjectWork {
    pub days: [DaysWork; 7]
}


#[derive(Default)]
pub struct WeeksWork {
    pub total: WeeksProjectWork,
    pub projects: HashMap<usize, WeeksProjectWork>,
    pub week_number: u32
}