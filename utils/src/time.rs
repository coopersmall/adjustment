use std::fmt::Display;
use std::fmt::Formatter;
use std::str::FromStr;

use chrono::NaiveDateTime;
use chrono::{Datelike, FixedOffset, NaiveDate, TimeZone as ChronoTimeZone};
use chrono_tz::Tz;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::errors::{Error, ErrorCode};

pub trait TimerExt {
    fn elapsed(&self) -> time::Duration;
}

pub struct Timer {
    start: time::Instant,
}

impl Timer {
    pub fn new() -> Self {
        Self {
            start: time::Instant::now(),
        }
    }
}

impl TimerExt for Timer {
    fn elapsed(&self) -> time::Duration {
        self.start.elapsed()
    }
}

pub enum DateTimeFormat {
    ISO8601,
    RFC2822,
    RFC3339,
}

pub trait Format {
    fn format(&self, format: &DateTimeFormat) -> Box<str>;
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Date {
    month: u8,
    day: u8,
    weekday: u8,
    year: i32,
}

impl Date {
    pub fn now() -> Result<Self, Error> {
        let date = OffsetDateTime::now_utc();

        let (month, day, year) = (date.month() as u8, date.day(), date.year());

        if !Date::is_valid(year, month, day) {
            return Err(Error::new("Invalid date provided", ErrorCode::Invalid));
        }

        let weekday = Date::get_day_of_week(year, month, day);

        let date = Self {
            month,
            day,
            weekday,
            year,
        };

        Ok(date)
    }

    pub fn new(year: i32, month: u8, day: u8) -> Result<Self, Error> {
        if !Date::is_valid(year, month, day) {
            return Err(Error::new("Invalid date provided", ErrorCode::Invalid));
        }

        let weekday = Date::get_day_of_week(year, month, day);

        let date = Self {
            month,
            day,
            weekday,
            year,
        };

        Ok(date)
    }

    pub fn month(&self) -> u8 {
        self.month
    }

    pub fn day(&self) -> u8 {
        self.day
    }

    pub fn weekday(&self) -> u8 {
        self.weekday
    }

    pub fn year(&self) -> i32 {
        self.year
    }

    pub(self) fn from_date_offset(date: &OffsetDateTime) -> Self {
        let (month, day, year) = (date.month() as u8, date.day(), date.year());
        let weekday = Date::get_day_of_week(year, month, day);

        Self {
            month,
            day,
            weekday,
            year,
        }
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
    /// use utils::time::Date;
    /// let date = Date::from_str("2019-01-01").unwrap();
    ///
    /// assert_eq!(date.month(), 1);
    /// assert_eq!(date.day(), 1);
    /// assert_eq!(date.year(), 2019);
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

        month.is_valid_day_in_month(day, year)
    }

    pub fn is_valid_year(year: i32) -> bool {
        year >= 0
    }

    /// Returns the day of the week for the date.
    /// 0 = Sunday, 1 = Monday, ..., 6 = Saturday
    ///
    /// This method is based on the Zeller's congruence algorithm.
    /// https://en.wikipedia.org/wiki/Zeller%27s_congruence
    ///
    /// This method is only valid for the Gregorian calendar.
    /// This method is only valid for dates after 1582-10-15.
    /// This method is only valid for dates before 9999-12-31.
    ///
    /// # Examples
    /// ```
    /// use utils::time::{Date, Weekday};
    ///
    /// let date = Date::from_str("2019-01-01").unwrap();
    /// let weekday = Date::get_day_of_week(date.year(), date.month(), date.day());
    ///
    /// assert_eq!(weekday, 3); // Tuesday
    /// assert_eq!(Weekday::from_u8(weekday).unwrap().as_str(), "Tuesday");
    /// ```
    pub fn get_day_of_week(year: i32, month: u8, day: u8) -> u8 {
        let mut month = month as i32;
        let mut year = year;

        if month < 3 {
            month += 12;
            year -= 1;
        }

        let century = year / 100;
        let year_of_century = year % 100;

        let weekday = (day as i32
            + (((month + 1) * 26) / 10)
            + year_of_century
            + (year_of_century / 4)
            + (century / 4)
            - (2 * century))
            % 7;

        ((weekday + 7) % 7) as u8
    }
}

impl Display for Date {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }
}

impl Format for Date {
    fn format(&self, format: &DateTimeFormat) -> Box<str> {
        match format {
            DateTimeFormat::ISO8601 => {
                format!("{:04}-{:02}-{:02}", self.year, self.month, self.day).into_boxed_str()
            }
            DateTimeFormat::RFC3339 => {
                format!("{:04}-{:02}-{:02}", self.year, self.month, self.day).into_boxed_str()
            }
            DateTimeFormat::RFC2822 => {
                format!("{:04}-{:02}-{:02}", self.year, self.month, self.day).into_boxed_str()
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Time {
    hour: u8,
    minute: u8,
    second: u8,
    millisecond: u16,
    offset: i32,
}

impl Time {
    pub fn new(
        hour: u8,
        minute: u8,
        second: u8,
        millisecond: u16,
        offset: i32,
    ) -> Result<Self, Error> {
        if !Time::is_valid(hour, minute, second, millisecond, offset) {
            return Err(Error::new("Invalid time", ErrorCode::Invalid));
        }

        let time = Self {
            hour,
            minute,
            second,
            millisecond,
            offset,
        };

        Ok(time)
    }

    /// Returns the current time in UTC.
    ///
    /// # Examples
    /// ```
    /// use utils::time::Time;
    ///
    /// let time = Time::now().unwrap();
    /// assert!(Time::is_valid(time.hour(), time.minute(), time.second(), time.millisecond(), time.offset()));
    /// ```
    pub fn now() -> Result<Self, Error> {
        let date = OffsetDateTime::now_utc();

        Time::from_date_offset(&date)
    }

    /// Returns the current time in the local offset.
    /// This method is only valid for timezones that are offset from UTC by whole minutes.
    ///
    /// # Examples
    /// ```
    /// use utils::time::Time;
    ///
    /// let time = Time::local().unwrap();
    /// assert!(Time::is_valid(time.hour(), time.minute(), time.second(), time.millisecond(), time.offset()));
    /// ```
    pub fn local() -> Result<Self, Error> {
        let date = match OffsetDateTime::now_local() {
            Ok(date) => date,
            Err(err) => return Err(Error::new(err.to_string().as_str(), ErrorCode::Invalid)),
        };

        Time::from_date_offset(&date)
    }

    pub(self) fn from_date_offset(date: &OffsetDateTime) -> Result<Self, Error> {
        let hour = date.hour() as u8;
        let minute = date.minute() as u8;
        let second = date.second() as u8;
        let millisecond = date.millisecond() as u16;
        let offset = date.offset().whole_seconds() as i32;

        Time::new(hour, minute, second, millisecond, offset)
    }

    pub fn hour(&self) -> u8 {
        self.hour
    }

    pub fn minute(&self) -> u8 {
        self.minute
    }

    pub fn second(&self) -> u8 {
        self.second
    }

    pub fn millisecond(&self) -> u16 {
        self.millisecond
    }

    pub fn offset(&self) -> i32 {
        self.offset
    }

    /// Checks if the time is valid.
    ///
    /// - This method is only valid for times after 00:00:00.000 and before 23:59:59.999.
    /// - This method is only valid for offsets between -12:00 and +14:00.
    ///
    /// # Examples
    /// ```
    /// use utils::time::Time;
    /// use utils::errors::ErrorCode;
    ///
    /// let hour = 23;
    /// let minute = 59;
    /// let second = 59;
    /// let millisecond = 999;
    /// let offset = 14 * 60 * 60;
    ///
    /// assert!(Time::is_valid(hour, minute, second, millisecond, offset));
    ///
    /// let hour = 24;
    /// assert!(!Time::is_valid(hour, minute, second, millisecond, offset));
    /// ```
    ///
    pub fn is_valid(hour: u8, minute: u8, second: u8, millisecond: u16, offset: i32) -> bool {
        Self::is_valid_hour(hour)
            && Self::is_valid_minute(minute)
            && Self::is_valid_second(second)
            && Self::is_valid_millisecond(millisecond)
            && Self::is_valid_offset(offset)
    }

    pub fn is_valid_hour(hour: u8) -> bool {
        hour <= 23
    }

    pub fn is_valid_minute(minute: u8) -> bool {
        minute <= 59
    }

    pub fn is_valid_second(second: u8) -> bool {
        second <= 59
    }

    pub fn is_valid_millisecond(millisecond: u16) -> bool {
        millisecond <= 999
    }

    pub fn is_valid_offset(offset: i32) -> bool {
        offset >= -43200 && offset <= 50400
    }
}

impl Display for Time {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:02}:{:02}:{:02}.{:03}{}",
            self.hour, self.minute, self.second, self.millisecond, self.offset
        )
    }
}

impl Format for Time {
    fn format(&self, format: &DateTimeFormat) -> Box<str> {
        match format {
            DateTimeFormat::ISO8601 => {
                let mut string = String::new();

                string.push_str(&format!("{:02}", self.hour));
                string.push(':');
                string.push_str(&format!("{:02}", self.minute));
                string.push(':');
                string.push_str(&format!("{:02}", self.second));
                string.push('.');
                string.push_str(&format!("{:03}", self.millisecond));
                string.push_str(&format!("{:+03}", self.offset / 3600));
                string.push_str(&format!("{:02}", (self.offset % 3600) / 60));

                string.into_boxed_str()
            }

            DateTimeFormat::RFC2822 => {
                let mut string = String::new();

                string.push_str(&format!("{:02}", self.hour));
                string.push(':');
                string.push_str(&format!("{:02}", self.minute));
                string.push(':');
                string.push_str(&format!("{:02}", self.second));
                string.push('.');
                string.push_str(&format!("{:03}", self.millisecond));
                string.push_str(&format!("{:+03}", self.offset / 3600));
                string.push_str(&format!("{:02}", (self.offset % 3600) / 60));

                string.into_boxed_str()
            }

            DateTimeFormat::RFC3339 => {
                let mut string = String::new();

                string.push_str(&format!("{:02}", self.hour));
                string.push(':');
                string.push_str(&format!("{:02}", self.minute));
                string.push(':');
                string.push_str(&format!("{:02}", self.second));
                string.push('.');
                string.push_str(&format!("{:03}", self.millisecond));
                string.push_str(&format!("{:+03}", self.offset / 3600));
                string.push_str(&format!("{:02}", (self.offset % 3600) / 60));

                string.into_boxed_str()
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct DateTime {
    date: Date,
    time: Time,
}

impl DateTime {
    pub fn new(date: Date, time: Time) -> Result<Self, Error> {
        if !DateTime::is_valid(&date, &time) {
            return Err(Error::new("Invalid date time", ErrorCode::Invalid));
        }

        let date_time = Self { date, time };

        Ok(date_time)
    }

    pub fn now() -> Result<Self, Error> {
        let date = Date::now()?;
        let time = Time::now()?;

        DateTime::new(date, time)
    }

    pub fn local() -> Result<Self, Error> {
        let date = Date::now()?;
        let time = Time::local()?;

        DateTime::new(date, time)
    }

    pub fn date(&self) -> &Date {
        &self.date
    }

    pub fn time(&self) -> &Time {
        &self.time
    }

    pub fn year(&self) -> i32 {
        self.date.year()
    }

    pub fn month(&self) -> u8 {
        self.date.month()
    }

    pub fn day(&self) -> u8 {
        self.date.day()
    }

    pub fn hour(&self) -> u8 {
        self.time.hour()
    }

    pub fn minute(&self) -> u8 {
        self.time.minute()
    }

    pub fn second(&self) -> u8 {
        self.time.second()
    }

    pub fn millisecond(&self) -> u16 {
        self.time.millisecond()
    }

    pub fn offset(&self) -> i32 {
        self.time.offset()
    }

    pub fn is_valid(date: &Date, time: &Time) -> bool {
        Date::is_valid(date.year(), date.month(), date.day())
            && Time::is_valid(
                time.hour(),
                time.minute(),
                time.second(),
                time.millisecond(),
                time.offset(),
            )
    }
}
impl Format for DateTime {
    /// Formats the date time using the given format.
    /// # Examples
    /// ```
    /// use utils::time::{DateTime, Date, Time, DateTimeFormat, Format};
    ///
    /// let date = Date::new(2020, 1, 1).unwrap();
    /// let time = Time::new(0, 0, 0, 0, 0).unwrap();
    ///
    /// let date_time = DateTime::new(date, time).unwrap();
    ///
    /// let iso8601 = date_time.format(&DateTimeFormat::ISO8601);
    /// assert_eq!(iso8601.as_ref(), "2020-01-01T00:00:00.000+0000");
    ///
    /// let rfc2822 = date_time.format(&DateTimeFormat::RFC2822);
    /// assert_eq!(rfc2822.as_ref(), "Wed, 01 Jan 2020 00:00:00 +0000");
    ///
    /// let rfc3339 = date_time.format(&DateTimeFormat::RFC3339);
    /// assert_eq!(rfc3339.as_ref(), "2020-01-01T00:00:00.000+0000");
    /// ```
    fn format(&self, format: &DateTimeFormat) -> Box<str> {
        match format {
            DateTimeFormat::ISO8601 => {
                let mut string = String::new();

                string.push_str(&self.date.format(format));
                string.push('T');
                string.push_str(&self.time.format(format));

                string.into_boxed_str()
            }

            DateTimeFormat::RFC2822 => {
                let mut string = String::new();

                string.push_str(&self.date.format(format));
                string.push(' ');
                string.push_str(&self.time.format(format));

                string.into_boxed_str()
            }

            DateTimeFormat::RFC3339 => {
                let mut string = String::new();

                string.push_str(&self.date.format(format));
                string.push('T');
                string.push_str(&self.time.format(format));

                string.into_boxed_str()
            }
        }
    }
}

pub enum Month {
    January = 1,
    February = 2,
    March = 3,
    April = 4,
    May = 5,
    June = 6,
    July = 7,
    August = 8,
    September = 9,
    October = 10,
    November = 11,
    December = 12,
}

impl From<Month> for u8 {
    fn from(month: Month) -> Self {
        match month {
            Month::January => 1,
            Month::February => 2,
            Month::March => 3,
            Month::April => 4,
            Month::May => 5,
            Month::June => 6,
            Month::July => 7,
            Month::August => 8,
            Month::September => 9,
            Month::October => 10,
            Month::November => 11,
            Month::December => 12,
        }
    }
}

impl Month {
    fn from_u8(month: u8) -> Result<Self, Error> {
        match month {
            1 => Ok(Self::January),
            2 => Ok(Self::February),
            3 => Ok(Self::March),
            4 => Ok(Self::April),
            5 => Ok(Self::May),
            6 => Ok(Self::June),
            7 => Ok(Self::July),
            8 => Ok(Self::August),
            9 => Ok(Self::September),
            10 => Ok(Self::October),
            11 => Ok(Self::November),
            12 => Ok(Self::December),
            _ => Err(Error::new("Invalid month provided", ErrorCode::Invalid)),
        }
    }

    pub fn as_str<'a>(&self) -> &'a str {
        match self {
            Self::January => "January",
            Self::February => "February",
            Self::March => "March",
            Self::April => "April",
            Self::May => "May",
            Self::June => "June",
            Self::July => "July",
            Self::August => "August",
            Self::September => "September",
            Self::October => "October",
            Self::November => "November",
            Self::December => "December",
        }
    }

    pub fn to_short_str<'a>(&self) -> &'a str {
        match self {
            Self::January => "Jan",
            Self::February => "Feb",
            Self::March => "Mar",
            Self::April => "Apr",
            Self::May => "May",
            Self::June => "Jun",
            Self::July => "Jul",
            Self::August => "Aug",
            Self::September => "Sep",
            Self::October => "Oct",
            Self::November => "Nov",
            Self::December => "Dec",
        }
    }

    pub fn get_days_from_month(month: u8, year: i32) -> Result<u8, Error> {
        let month = Self::from_u8(month)?;

        Ok(month.valid_days_in_month(year))
    }

    pub fn valid_days_in_month(&self, year: i32) -> u8 {
        match self {
            Self::January => 31,
            Self::February => {
                if year % 4 == 0 {
                    29
                } else {
                    28
                }
            }
            Self::March => 31,
            Self::April => 30,
            Self::May => 31,
            Self::June => 30,
            Self::July => 31,
            Self::August => 31,
            Self::September => 30,
            Self::October => 31,
            Self::November => 30,
            Self::December => 31,
        }
    }

    pub fn is_valid_day_in_month(&self, day: u8, year: i32) -> bool {
        day <= self.valid_days_in_month(year)
    }
}

pub enum Weekday {
    Sunday = 1,
    Monday = 2,
    Tuesday = 3,
    Wednesday = 4,
    Thursday = 5,
    Friday = 6,
    Saturday = 7,
}

impl From<Weekday> for u8 {
    fn from(day: Weekday) -> Self {
        match day {
            Weekday::Sunday => 1,
            Weekday::Monday => 2,
            Weekday::Tuesday => 3,
            Weekday::Wednesday => 4,
            Weekday::Thursday => 5,
            Weekday::Friday => 6,
            Weekday::Saturday => 7,
        }
    }
}

impl Weekday {
    pub fn from_u8(day: u8) -> Result<Self, Error> {
        match day {
            1 => Ok(Self::Sunday),
            2 => Ok(Self::Monday),
            3 => Ok(Self::Tuesday),
            4 => Ok(Self::Wednesday),
            5 => Ok(Self::Thursday),
            6 => Ok(Self::Friday),
            7 => Ok(Self::Saturday),
            _ => Err(Error::new("Invalid day provided", ErrorCode::Invalid)),
        }
    }

    pub fn as_str<'a>(&self) -> &'a str {
        match self {
            Self::Sunday => "Sunday",
            Self::Monday => "Monday",
            Self::Tuesday => "Tuesday",
            Self::Wednesday => "Wednesday",
            Self::Thursday => "Thursday",
            Self::Friday => "Friday",
            Self::Saturday => "Saturday",
        }
    }

    pub fn to_short_str<'a>(&self) -> &'a str {
        match self {
            Self::Sunday => "Sun",
            Self::Monday => "Mon",
            Self::Tuesday => "Tue",
            Self::Wednesday => "Wed",
            Self::Thursday => "Thu",
            Self::Friday => "Fri",
            Self::Saturday => "Sat",
        }
    }
}

pub enum TimeZone {
    UTC,
    Other(DateTime),
}

impl TimeZone {
    /// Returns the timezone as a str
    /// UTC, Local, or a Timezone offset
    ///
    /// # Examples
    /// ```
    /// use utils::time::{DateTime, Date, Time};
    /// use utils::time::TimeZone;
    ///
    /// let timezone = TimeZone::UTC.as_offset_str();
    /// assert_eq!(timezone.as_ref(), "+00:00");
    ///
    /// let date = Date::new(2020, 1, 1).unwrap();
    /// let time = Time::new(0, 0, 0, 0, 3600).unwrap();
    ///
    /// let date_time = DateTime::new(date, time).unwrap();
    ///
    /// let timezone = TimeZone::Other(date_time).as_offset_str();
    /// assert_eq!(timezone.as_ref(), "+01:00");
    /// ```
    pub fn as_offset_str(&self) -> Box<str> {
        match self {
            TimeZone::UTC => "+00:00".into(),
            TimeZone::Other(date_time) => {
                let offset = date_time.offset();
                match FixedOffset::east_opt(offset) {
                    Some(offset) => return offset.to_string().into(),
                    None => todo!(),
                };
            }
        }
    }

    /// Returns the name of the timezone
    ///
    /// # Examples
    /// ```
    /// use utils::time::{DateTime, Date, Time};
    /// use utils::time::TimeZone;
    ///
    /// let timezone = TimeZone::UTC.as_str();
    /// assert_eq!(timezone.as_ref(), "UTC");
    ///
    /// let date = Date::new(2020, 1, 1).unwrap();
    /// let time = Time::new(0, 0, 0, 0, 3600).unwrap();
    ///
    /// let date_time = DateTime::new(date, time).unwrap();
    ///
    /// let timezone = TimeZone::Other(date_time).as_str();
    /// assert_eq!(timezone.as_ref(), "America/Chicago");
    /// ```
    pub fn as_str(&self) -> Box<str> {
        match self {
            TimeZone::UTC => "UTC".into(),
            TimeZone::Other(date_time) => "Other".into(),
        }
    }
}
