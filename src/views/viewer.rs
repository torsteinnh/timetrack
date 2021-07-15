use super::show_msdynamics;

use crate::events::Sheet;
use crate::options::LogType;


pub fn show(sheet: Sheet, log_type: LogType) {
    match log_type {
        LogType::MSDynamics => show_msdynamics::show(sheet)
    }
}
