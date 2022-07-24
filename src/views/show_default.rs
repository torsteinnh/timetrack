use prettytable::{Table, Row, Cell, row, cell, format};

use super::viewer::ParsedSheet;
use crate::events::JobIdentifier;
use crate::options::Options;


pub fn show(parsed: ParsedSheet, config: &Options) {
    println!("Report for timesheet formatted for MS Dynamic:");

    for week in parsed {
        let mut table = Table::new();
        table.set_titles(row![H10 -> format!("Report for week {}:", week.week_number)]);
        table.add_row(row!["Name", "ID", "Category", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"]);

        for (key, project_week) in week.projects.iter() {
            let mut cell_vec: Vec<Cell> = Vec::new();

            let project = JobIdentifier::ProjectId(*key).get_jobtype(config).unwrap();

            cell_vec.push(cell!(project.u_name));
            cell_vec.push(cell!(r -> project.project_id));
            cell_vec.push(cell!(r -> project.category));
            for day in project_week.days.iter() {
                cell_vec.push(cell!(r -> format!("{}h, {}m", day.total_day.as_secs() / 3600, (day.total_day.as_secs() / 60) % 60)));
            }
            
            table.add_row(Row::new(cell_vec));
        }

        let mut cell_vec = vec![cell!(H3c -> "In total")];
        for day in week.total.days.iter() {
            cell_vec.push(cell!(r -> format!("{}h, {}m", day.total_day.as_secs() / 3600, (day.total_day.as_secs() / 60) % 60)));
        }
        table.add_row(Row::new(cell_vec));

        table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
        println!();
        table.printstd();
    }
    println!();
}
