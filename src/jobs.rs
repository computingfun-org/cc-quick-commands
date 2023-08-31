use std::str::FromStr;

use cc_core::{JobNumber, JobURL};

pub fn from_clipboard() -> Option<JobNumber> {
    let clip = cli_clipboard::get_contents().ok()?;
    JobNumber::from_str(clip.trim()).ok()
}

pub fn open(num: Option<JobNumber>) -> Result<bool, std::io::Error> {
    match num {
        Some(n) => match open::that(JobURL::from(n).as_str()) {
            Ok(_) => Ok(true),
            Err(err) => Err(err),
        },
        None => Ok(false),
    }
}
