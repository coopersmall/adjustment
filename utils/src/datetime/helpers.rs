use super::primatives::*;
use super::Date;

// Comparison Functions

pub fn is_same_date(date1: &Date, date2: &Date) -> bool {
    date1.day() == date2.day() && date1.month() == date2.month() && date1.year() == date2.year()
}

pub fn is_weekday(date: &Date) -> bool {
    date.weekday() != 0 && date.weekday() != 6
}

// Addition Functions

pub fn add_days(date: &mut Date, days: u32) {
    let days_in_month = days_in_month(date.month(), date.year());
    let total_days = date.day() as u32 + days;
    if total_days > days_in_month {
        date.day() = total_days % days_in_month;
        add_months(date, total_days / days_in_month);
    } else {
        date.day() = total_days as u8;
    }
}

pub fn add_weeks(date: &mut Date, weeks: u32) {
    add_days(date, weeks * 7);
}

pub fn add_business_days(date: &mut Date, business_days: u32) {
    let mut remaining_days = business_days;
    while remaining_days > 0 {
        add_days(date, 1);
        if is_weekday(date) {
            remaining_days -= 1;
        }
    }
}

// Subtraction Functions

pub fn subtract_days(date: &mut Date, days: u32) {
    let total_days = date.day() as i32 - days as i32;
    if total_days > 0 {
        date.day() = total_days as u8;
    } else {
        subtract_months(date, (-total_days / 30) as u32);
        let days_in_month = days_in_month(date.month(), date.year());
        date.day() = (days_in_month - (-total_days % days_in_month) as u32) as u8;
    }
}

pub fn subtract_weeks(date: &mut Date, weeks: u32) {
    subtract_days(date, weeks * 7);
}

pub fn subtract_business_days(date: &mut Date, business_days: u32) {
    let mut remaining_days = business_days;
    while remaining_days > 0 {
        subtract_days(date, 1);
        if is_weekday(date) {
            remaining_days -= 1;
        }
    }
}

// Multiplication Functions

pub fn multiply_days(date: &mut Date, multiplier: u32) {
    let total_days = date.day() as u32 * multiplier;
    add_months(date, total_days / days_in_month(date.month(), date.year()));
    date.day() = (total_days % days_in_month(date.month(), date.year())) as u8;
}

pub fn multiply_weeks(date: &mut Date, multiplier: u32) {
    multiply_days(date, multiplier * 7);
}

pub fn multiply_business_days(date: &mut Date, multiplier: u32) {
    let mut remaining_days = multiplier;
    let original_date = *date;
    while remaining_days > 1 {
        add_days(date, 1);
        if is_weekday(date) {
            remaining_days -= 1;
        }
    }
    *date = original_date;
}

// Helper Functions

fn days_in_month(month: u8, year: i32) -> u8 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if is_leap_year(year) {
                29
            } else {
                28
            }
        }
        _ => 0,
    }
}

fn normalize_date(date: &mut Date) {
    let days_in_month = days_in_month(date.month(), date.year());
    if date.day() > days_in_month {
        date.day() = days_in_month as u8;
    }
}

// Comparison Functions

pub fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

pub fn is_future_date(date: &Date) -> bool {
    let today = Date::now();
    date > &today
}

// Addition Functions

pub fn add_months(date: &mut Date, months: u32) {
    let total_months = date.year() as u32 * 12 + date.month() as u32 + months;
    date.year() = (total_months / 12) as i32;
    date.month() = (total_months % 12) as u8;
    normalize_date(date);
}

pub fn add_years(date: &mut Date, years: u32) {
    date.year() += years as i32;
    normalize_date(date);
}

// Subtraction Functions

pub fn subtract_months(date: &mut Date, months: u32) {
    let total_months = date.year() as u32 * 12 + date.month() as u32 + 1;
    if months >= total_months {
        date.year() = 0;
        date.month() = 1;
    } else {
        let remaining_months = total_months - months;
        date.year() = (remaining_months / 12) as i32;
        date.month() = (remaining_months % 12) as u8;
    }
}

pub fn subtract_years(date: &mut Date, years: u32) {
    if years >= date.year() as u32 {
        date.year() = 0;
        date.month() = 1;
        date.day() = 1;
    } else {
        date.year() -= years as i32;
    }
}

// Multiplication Functions

pub fn multiply_months(date: &mut Date, multiplier: u32) {
    let total_months = date.year() as u32 * 12 + date.month() as u32 + 1;
    let multiplied_months = total_months * multiplier;
    date.year() = (multiplied_months / 12) as i32;
    date.month() = (multiplied_months % 12) as u8;
    normalize_date(date);
}

pub fn multiply_years(date: &mut Date, multiplier: u32) {
    date.year() *= multiplier as i32;
    normalize_date(date);
}
