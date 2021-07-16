use chrono::{IsoWeek, prelude::*};

use std::time::Duration;

use crate::events::{Sheet, Event, JobIdentifier};
use crate::options::JobType;


pub fn show(sheet: Sheet) {
    let parsed = parse_sheet(sheet);

    println!("Report for timesheet formated for MS Dynamicd:");

    for week in parsed {
        println!("\nReport for week {}:", week.week_number);

        print!("Project \"{}\",\tID {},\tCategory {}\t", "Total", 0, 0);
        for day in week.days.iter() {
            print!("| {}h, {}m\t", day.total_day.as_secs() / 3600, (day.total_day.as_secs() / 60) % 60);
        }
        println!("|");
    }
}


fn parse_sheet(sheet: Sheet) -> Vec<WeeksWork> {
    let mut parsed_sheet = vec![];

    let mut current_week_work = WeeksWork::default();
    let mut current_week: IsoWeek = Local::now().iso_week();
    let mut current_dow = 0;
    let mut current_start: DateTime<Local> = Local::now();
    let mut current_end: DateTime<Local> = Local::now();
    let mut current_breaks: Duration = Duration::new(0, 0);

    let mut begin_parsing = false;


    for event in sheet {
        match event {
            Event::BEGIN(time) => {
                if time.day() == current_dow {
                    current_breaks += (time - current_end).to_std().unwrap();
                }
                else {
                    current_dow = time.weekday().num_days_from_monday();
                    current_start = time;
                    current_breaks = Duration::new(0, 0);

                    if current_start.iso_week() != current_week {
                        if !begin_parsing { begin_parsing = true; } else { parsed_sheet.push(current_week_work); }
                        
                        current_week = current_start.iso_week();
                        current_week_work = WeeksWork::default();
                    }
                }

                begin_parsing = true;
            },

            Event::END(time) => {
                assert_eq!(current_dow, time.weekday().num_days_from_monday(), "End event first on day at time {}.", time);
                assert!(begin_parsing, "End event first in sheet, should have begun with Begin.");

                current_end = time;

                current_week_work.days[current_dow as usize].total_day += (current_end - current_start).to_std().unwrap() - current_breaks;
            },

            Event::PAUSE(interval) => {
                assert!(begin_parsing, "End event first in sheet, should have begun with Begin.");

                current_breaks += interval;
            },

            Event::SWITCH(time, job_id) => {
                assert!(begin_parsing, "End event first in sheet, should have begun with Begin.");

                // TODO Implement handling of different projects
            }
        }
    };

    assert!(begin_parsing, "Sheet did not contain a single event.");
    parsed_sheet.push(current_week_work);

    parsed_sheet
}


#[derive(Default)]
struct DaysWork {
    total_day: Duration,
    // TODO per_project: Vec<(Duration, JobIdentifier)>
}


#[derive(Default)]
struct WeeksWork {
    days: [DaysWork; 7],
    week_number: u32
}
