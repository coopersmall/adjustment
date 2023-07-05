use chrono::{Datelike, NaiveDate};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use super::primatives::*;
use super::{DateFormatResult, DateTimeFormat, Format, FormatLocal, FormatNow};
use crate::errors::{Error, ErrorCode};
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Eq, Ord, Serialize, Deserialize)]
pub struct Date {
    year: Year,
    month: Month,
    day: Day,
    weekday: Weekday,
}

impl Date {
    pub fn new(year: i32, month: u8, day: u8) -> Result<Self, Error> {
        if !Date::is_valid(year, month, day) {
            return Err(Error::new("Invalid date provided", ErrorCode::Invalid));
        }

        let weekday = Weekday::from_values(year, month, day)?;
        let year = Year::from_i32(year)?;
        let month = Month::from_u8(month)?;
        let day = Day::from_u8(day)?;

        let date = Self {
            year,
            month,
            day,
            weekday,
        };

        Ok(date)
    }

    pub fn now() -> Self {
        let date = OffsetDateTime::now_utc();

        let (month, day, year) = (date.month() as u8, date.day(), date.year());

        let weekday = Weekday::dangerously_from_values(year, month, day);
        let month = Month::dangerously_from_u8(month);
        let year = Year::dangerously_from_i32(year);
        let day = Day::dangerously_from_u8(day);

        let date = Self {
            year,
            month,
            day,
            weekday,
        };

        date
    }

    pub fn local() -> Result<Self, Error> {
        let date = match OffsetDateTime::now_local() {
            Ok(date) => date,
            Err(err) => {
                return Err(
                    Error::new("Failed to get local date", ErrorCode::Internal).with_cause(err)
                )
            }
        };

        let (month, day, year) = (date.month() as u8, date.day(), date.year());

        let weekday = Weekday::dangerously_from_values(year, month, day);
        let month = Month::dangerously_from_u8(month);
        let year = Year::dangerously_from_i32(year);
        let day = Day::dangerously_from_u8(day);

        let date = Self {
            year,
            month,
            day,
            weekday,
        };

        Ok(date)
    }

    pub(super) fn from_offset_time(offset_time: &OffsetDateTime) -> Self {
        let (month, day, year) = (
            offset_time.month() as u8,
            offset_time.day(),
            offset_time.year(),
        );

        let weekday = Weekday::dangerously_from_values(year, month, day);
        let month = Month::dangerously_from_u8(month);
        let year = Year::dangerously_from_i32(year);
        let day = Day::dangerously_from_u8(day);

        let date = Self {
            year,
            month,
            day,
            weekday,
        };

        date
    }

    pub fn year(&self) -> &Year {
        &self.year
    }

    pub fn month(&self) -> &Month {
        &self.month
    }

    pub fn day(&self) -> &Day {
        &self.day
    }

    pub fn weekday(&self) -> &Weekday {
        &self.weekday
    }

    /// Parses a date from a string.
    /// The date must be in one of the following formats:
    /// - YYYY-MM-DD
    /// - MM-DD-YYYY
    /// - YYYY/MM/DD
    /// - MM/DD/YYYY
    ///
    /// # Examples
    /// ```
    /// use utils::datetime::date::{Date, Month, Weekday};
    /// let date = Date::from_str("2019-01-01").unwrap();
    ///
    /// assert_eq!(date.year(), 2019);
    /// assert_eq!(date.month(), Month::January as u8);
    /// assert_eq!(date.day(), 1);
    /// assert_eq!(date.weekday(), Weekday::Tuesday as u8);
    /// ```

    pub fn from_str(date_str: &str) -> Result<Self, Error> {
        let formats = [
            "%Y-%m-%d", "%m-%d-%Y", "%Y/%m/%d", "%m/%d/%Y", // Add more formats as needed
        ];

        for format in &formats {
            if let Ok(parsed) = NaiveDate::parse_from_str(date_str, format) {
                let year = parsed.year();
                let month = parsed.month() as u8;
                let day = parsed.day() as u8;
                return Date::new(year, month, day)
                    .map_err(|err| Error::new(err.to_string().as_str(), ErrorCode::Invalid));
            }
        }

        Err(Error::new("Invalid date format", ErrorCode::Invalid))
    }

    pub fn unix(&self) -> u32 {
        let mut unix = 0;

        unix += self.year().unix();
        unix += self.month().unix(self.year());
        unix += self.day().unix(self.year(), self.month());

        unix
    }

    pub fn primatives(&self) -> (i32, u8, u8) {
        (
            self.year().as_i32(),
            self.month().as_u8(),
            self.day().as_u8(),
        )
    }

    pub fn is_same_date(&self, date2: &Date) -> bool {
        self.day() == date2.day() && self.month() == date2.month() && self.year() == date2.year()
    }

    pub fn is_today(&self) -> bool {
        let today = Date::now();
        self.is_same_date(&today)
    }

    pub fn is_yesterday(&self) -> bool {
        let yesterday = Date::now().sub_days(1);
        self.is_same_date(&yesterday)
    }

    pub fn is_tomorrow(&self) -> bool {
        let tomorrow = Date::now().add_days(1);
        self.is_same_date(&tomorrow)
    }

    pub fn is_weekday(&self) -> bool {
        self.weekday() != &Weekday::Saturday && self.weekday() != &Weekday::Sunday
    }

    pub fn is_past_date(&self, date: &Date) -> bool {
        date < &self
    }

    pub fn is_future_date(&self, date: &Date) -> bool {
        date > &self
    }

    pub fn is_last_day_of_month(&self) -> bool {
        self.month().is_last_day(&self.year(), &self.day())
    }

    pub fn is_last_day_of_year(&self) -> bool {
        self.month() == &Month::December && self.day() == &31
    }

    pub fn is_first_day_of_month(&self) -> bool {
        self.day() == &1
    }

    pub fn is_first_day_of_year(&self) -> bool {
        self.month() == &Month::January && self.day() == &1
    }

    pub fn is_leap_year_day(&self) -> bool {
        self.month() == &Month::February && self.day() == &29
    }

    pub fn is_valid(year: i32, month: u8, day: u8) -> bool {
        Self::is_valid_month(month)
            && Self::is_valid_day(month, day, year)
            && Self::is_valid_year(year)
    }

    pub fn is_valid_month(month: u8) -> bool {
        month >= 1 && month <= 12
    }

    pub fn is_valid_day(month: u8, day: u8, year: i32) -> bool {
        if !Self::is_valid_month(month) {
            return false;
        }

        let month = match Month::from_u8(month) {
            Ok(month) => month,
            Err(_) => return false,
        };

        month.is_valid_day(&day, &year)
    }

    pub fn is_valid_year(year: i32) -> bool {
        year >= 0
    }

    pub fn days_between_count(&self, date2: &Date) -> u32 {
        let mut days = 0;

        let mut date = self;

        if date > date2 {
            while date > date2 {
                days += 1;
                date.sub_days(1);
            }
        }

        if date < date2 {
            while date < date2 {
                days += 1;
                date.add_days(1);
            }
        }

        days
    }

    pub fn weekdays_before_weekday(&self, weekday: &Weekday) -> u8 {
        let mut days = 0;

        let mut date = self;
        while date.weekday() != weekday {
            if date.is_weekday() {
                days += 1;
            }
            date.add_days(1);
        }

        days
    }

    pub fn weekdays_after_weekday(&self, weekday: &Weekday) -> u8 {
        if self.is_weekday() {
            return 0;
        }

        let mut days = 0;

        let mut date = self;
        while date.weekday() != weekday {
            if date.is_weekday() {
                days += 1;
            }
            date.sub_days(1);
        }

        days
    }

    pub fn weekdays_until_next_weekday(&self) -> u8 {
        let mut days = 0;

        let mut date = self;
        while date.is_weekday() {
            days += 1;
            date.add_days(1);
        }

        days
    }
}

impl Date {
    pub fn add_days(&mut self, days: u8) -> Self {
        for _ in 0..days {
            if self.is_last_day_of_month() {
                self.month.next();
            }

            if self.day() > &self.month().last_day(self.year()) {
                self.day = Day::first();
            } else {
                self.day.next();
            }

            if self.is_first_day_of_year() {
                self.year.next();
            }
        }

        *self
    }

    pub fn sub_days(&mut self, days: u8) -> Self {
        for _ in 0..days {
            if self.is_first_day_of_month() {
                self.month.next_back();
            }

            if self.day() == &1 {
                self.day = self.month().last_day(self.year());
            } else {
                self.day.next_back();
            }

            if self.is_last_day_of_year() {
                self.year.next_back();
            }
        }

        *self
    }

    pub fn add_weeks(&mut self, weeks: u8) -> Self {
        self.add_days(weeks * 7)
    }

    pub fn sub_weeks(&mut self, weeks: u8) -> Self {
        self.sub_days(weeks * 7)
    }

    pub fn add_months(&mut self, months: u8) -> Self {
        let is_last_day_of_month = self.is_last_day_of_month();

        for _ in 0..months {
            if self.month() == &Month::December {
                self.year.next();
            }

            let last_day = self.month().last_day(self.year());

            if is_last_day_of_month {
                self.day = last_day;
            } else if self.day() > &last_day {
                self.day = last_day;
            }

            self.month.next();
        }

        *self
    }

    pub fn sub_months(&mut self, months: u8) -> Self {
        let is_last_day_of_month = self.is_last_day_of_month();

        for _ in 0..months {
            if self.month() == &Month::January {
                self.year.next_back();
            }

            let last_day = self.month().last_day(self.year());

            if is_last_day_of_month {
                self.day = last_day;
            } else if self.day() > &last_day {
                self.day = last_day;
            }

            self.month.next_back();
        }

        *self
    }

    pub fn add_years(&mut self, years: u32) -> Self {
        let is_leap_year_day = self.is_leap_year_day();

        for _ in 0..years {
            let is_leap_year = self.year().is_leap_year();

            if is_leap_year_day && is_leap_year {
                self.day = Day::dangerously_from_u8(29);
            } else if is_leap_year_day && !is_leap_year {
                self.day = Day::dangerously_from_u8(28);
            }

            self.year.next();
        }

        *self
    }

    pub fn sub_years(&mut self, years: u32) -> Self {
        let is_leap_year_day = self.is_leap_year_day();

        for _ in 0..years {
            let is_leap_year = self.year().is_leap_year();

            if is_leap_year_day && is_leap_year {
                self.day = Day::dangerously_from_u8(29);
            } else if is_leap_year_day && !is_leap_year {
                self.day = Day::dangerously_from_u8(28);
            }

            self.year.next_back();
        }

        *self
    }

    pub fn next_day(&self) -> Date {
        self.add_days(1)
    }

    pub fn prev_day(&self) -> Date {
        self.sub_days(1)
    }

    pub fn next_week(&self) -> Date {
        self.add_weeks(1)
    }

    pub fn prev_week(&self) -> Date {
        self.sub_weeks(1)
    }

    pub fn next_month(&self) -> Date {
        self.add_months(1)
    }

    pub fn prev_month(&self) -> Date {
        self.sub_months(1)
    }

    pub fn next_year(&self) -> Date {
        self.add_years(1)
    }

    pub fn prev_year(&self) -> Date {
        self.sub_years(1)
    }

    pub fn next_days(&self, days: u8) -> Box<[&Date]> {
        let mut dates = Vec::with_capacity(days as usize);
        let mut date = self;

        for _ in 0..days {
            date = &date.next_day();
            dates.push(date);
        }

        dates.into_boxed_slice()
    }

    pub fn prev_days(&self, days: u8) -> Box<[&Date]> {
        let mut dates = Vec::with_capacity(days as usize);
        let mut date = self;

        for _ in 0..days {
            date = &date.prev_day();
            dates.push(date);
        }

        dates.into_boxed_slice()
    }

    pub fn next_weekday(&self) -> Date {
        let mut date = self.next_day();

        while date.weekday() == &Weekday::Saturday || date.weekday() == &Weekday::Sunday {
            date = date.next_day();
        }

        date
    }

    pub fn prev_weekday(&self) -> Date {
        let mut date = self.prev_day();

        while date.weekday() == &Weekday::Saturday || date.weekday() == &Weekday::Sunday {
            date = date.prev_day();
        }

        date
    }

    pub fn days_between(&self, other: &Date) -> Box<[Date]> {
        let mut dates = Vec::new();
        let mut date = self.clone();

        if date > *other {
            while date != *other {
                dates.push(date);
                date = date.prev_day();
            }
        }

        if date < *other {
            while date != *other {
                dates.push(date);
                date = date.next_day();
            }
        }

        dates.into_boxed_slice()
    }
}

impl Date {}

impl Date {}

impl PartialEq<Date> for Date {
    fn eq(&self, other: &Date) -> bool {
        self.year == other.year
            && self.month == other.month
            && self.day == other.day
            && self.weekday == other.weekday
    }
}

impl PartialOrd<Date> for Date {
    fn partial_cmp(&self, other: &Date) -> Option<std::cmp::Ordering> {
        if self.year > other.year {
            return Some(std::cmp::Ordering::Greater);
        } else if self.year < other.year {
            return Some(std::cmp::Ordering::Less);
        }

        if self.month > other.month {
            return Some(std::cmp::Ordering::Greater);
        } else if self.month < other.month {
            return Some(std::cmp::Ordering::Less);
        }

        if self.day > other.day {
            return Some(std::cmp::Ordering::Greater);
        } else if self.day < other.day {
            return Some(std::cmp::Ordering::Less);
        }

        Some(std::cmp::Ordering::Equal)
    }
}

impl Display for Date {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }
}

impl Format for Date {
    fn format(&self, format: &DateTimeFormat) -> DateFormatResult {
        Ok(self.shared_format(format))
    }
}

impl FormatNow for Date {
    fn format_now(format: &DateTimeFormat) -> Box<str> {
        let now = Date::now();
        now.shared_format(format)
    }
}

impl FormatLocal for Date {
    fn format_local(format: &DateTimeFormat) -> DateFormatResult {
        let now = Date::local()?;
        Ok(now.shared_format(format))
    }
}

impl Iterator for Date {
    type Item = Date;

    fn next(&mut self) -> Option<Self::Item> {
        let date = self.next_day();
        self.year = date.year;
        self.month = date.month;
        self.day = date.day;
        self.weekday = date.weekday;

        Some(date)
    }
}

impl DoubleEndedIterator for Date {
    fn next_back(&mut self) -> Option<Self::Item> {
        let date = self.prev_day();
        self.year = date.year;
        self.month = date.month;
        self.day = date.day;
        self.weekday = date.weekday;

        Some(date)
    }
}

impl Date {
    fn shared_format(&self, format: &DateTimeFormat) -> Box<str> {
        match format {
            DateTimeFormat::ISO8601 => {
                return format!("{:04}-{:02}-{:02}", self.year, self.month.as_u8(), self.day)
                    .into();
            }

            DateTimeFormat::PRETTY => {
                return format!(
                    "{}, {} {} {:04}",
                    self.weekday.as_short(),
                    self.month.as_long(),
                    self.day.pretty_format(),
                    self.year
                )
                .into();
            }

            DateTimeFormat::RFC3339 => {
                return format!("{:04}-{:02}-{:02}", self.year, self.month.as_u8(), self.day)
                    .into();
            }

            DateTimeFormat::RFC2822 => {
                return format!(
                    "{}, {:02} {} {:04}",
                    self.weekday.as_short(),
                    self.day,
                    self.month.as_short(),
                    self.year
                )
                .into();
            }
        }
    }
}
