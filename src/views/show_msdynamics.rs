use super::viewer::ParsedSheet;
use crate::events::JobIdentifier;
use crate::options::Options;


pub fn show(parsed: ParsedSheet, config: Options) {
    println!("Report for timesheet formated for MS Dynamic:");

    for week in parsed {
        println!("\nReport for week {}:", week.week_number);

        for (key, project_week) in week.projects.iter() {
            let project = JobIdentifier::ProjectId(*key).get_jobtype(&config).unwrap();

            print!("Project \"{}\",\tID {},\tCategory {}\t", project.u_name, project.project_id, project.category);
            for day in project_week.days.iter() {
                print!("| {}h, {}m\t", day.total_day.as_secs() / 3600, (day.total_day.as_secs() / 60) % 60);
            }
            println!("|");
        }
        print!("In total:\t\t\t\t\t");
        for day in week.total.days.iter() {
            print!("| {}h, {}m\t", day.total_day.as_secs() / 3600, (day.total_day.as_secs() / 60) % 60);
        }
        println!("|");
    }
}
