use prettytable::{Table, Row, Cell, row, cell, format};

use super::viewer::{ParsedSheet};
use crate::events::JobIdentifier;
use crate::options::Options;


pub fn show(parsed: ParsedSheet, current_project_identifier: JobIdentifier, config: &Options) {
    println!("Using default formatting for timesheet:");

    for parsed_week in parsed {
        let week = parsed_week.transpose();

        let mut table = Table::new();
        table.set_titles(row![H11 -> format!("Report for week {}:", week.week_number)]);
        table.add_row(row!["Name", "ID", "Category", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun", "Total"]);

        for (key, project_week) in week.projects.iter() {
            let mut cell_vec: Vec<Cell> = Vec::new();

            let project = JobIdentifier::InternalId(*key).get_jobtype(config).unwrap();

            cell_vec.push(cell!(project.u_name));
            cell_vec.push(cell!(r -> project.project_id));
            cell_vec.push(cell!(r -> project.category));
            let mut accumulated_project = 0;
            for day in project_week.days.iter() {
                cell_vec.push(cell!(r -> format!("{:>2}h, {:>2}m", day.total_day.as_secs() / 3600, (day.total_day.as_secs() / 60) % 60)));
                accumulated_project += day.total_day.as_secs();
            }
            cell_vec.push(cell!(r -> format!("{:>2}h, {:>2}m", accumulated_project / 3600, (accumulated_project / 60) % 60)));
            table.add_row(Row::new(cell_vec));
        }

        let mut cell_vec = vec![cell!(H3c -> "In total")];
        let mut accumulated_project = 0;
        for day in week.total.days.iter() {
            cell_vec.push(cell!(r -> format!("{:>2}h, {:>2}m", day.total_day.as_secs() / 3600, (day.total_day.as_secs() / 60) % 60)));
            accumulated_project += day.total_day.as_secs();
        }
        cell_vec.push(cell!(r -> format!("{:>2}h, {:>2}m", accumulated_project / 3600, (accumulated_project / 60) % 60)));
        table.add_row(Row::new(cell_vec));

        table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
        println!();
        table.printstd();
        println!("Total work: {}h, {}m", week.total_time.as_secs() / 3600, (week.total_time.as_secs() / 60) % 60);
    }

    let current_project = current_project_identifier.get_jobtype(config).unwrap();
    println!();
    println!("Working on project {} with ID {}, category {} and internal id {}.", current_project.u_name, current_project.project_id, current_project.category, current_project.internal_id);
}
