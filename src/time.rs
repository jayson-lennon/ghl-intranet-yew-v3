// src/time.rs

use chrono::prelude::*;

pub fn get_current_year() -> i32 {
    Local::now().year()
}

pub fn get_current_date() -> DateTime<Local> {
    Local::now()
}