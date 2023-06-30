use chrono::{Datelike, NaiveDate};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use super::{DateFormatResult, DateTimeFormat, Format, FormatLocal, FormatNow};
use crate::errors::{Error, ErrorCode};
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Date {
    year: i32,
    month: Month,
    day: u8,
    weekday: Weekday,
}

impl Date {
    pub fn new(year: i32, month: u8, day: u8) -> Result<Self, Error> {
        if !Date::is_valid(year, month, day) {
            return Err(Error::new("Invalid date provided", ErrorCode::Invalid));
        }

        let month = match Month::from_u8(month) {
            Ok(month) => month,
            Err(err) => {
                return Err(Error::new("Invalid month provided", ErrorCode::Invalid).with_cause(err))
            }
        };

        let weekday = Date::get_weekday(year, month.as_u8(), day);
        let weekday = match Weekday::from_u8(weekday) {
            Ok(weekday) => weekday,
            Err(err) => {
                return Err(Error::new("Failed to get weekday", ErrorCode::Invalid).with_cause(err))
            }
        };

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
        let month = Month::dangerously_from_u8(month);

        let weekday = Date::get_weekday(year, month.as_u8(), day);
        let weekday = Weekday::dangerously_from_u8(weekday);

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
        let month = Month::dangerously_from_u8(month);

        let weekday = Date::get_weekday(year, month.as_u8(), day);
        let weekday = Weekday::dangerously_from_u8(weekday);

        let date = Self {
            year,
            month,
            day,
            weekday,
        };

        Ok(date)
    }

    pub fn year(&self) -> i32 {
        self.year
    }

    pub fn month(&self) -> u8 {
        self.month.as_u8()
    }

    pub fn day(&self) -> u8 {
        self.day
    }

    pub fn weekday(&self) -> u8 {
        self.weekday.as_u8()
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
                    pretty_format_day(self.day),
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

    fn get_weekday(year: i32, month: u8, day: u8) -> u8 {
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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
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

    pub fn as_long<'a>(&self) -> &'a str {
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

    pub fn as_short<'a>(&self) -> &'a str {
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

    pub fn as_u8(&self) -> u8 {
        match self {
            Self::January => 1,
            Self::February => 2,
            Self::March => 3,
            Self::April => 4,
            Self::May => 5,
            Self::June => 6,
            Self::July => 7,
            Self::August => 8,
            Self::September => 9,
            Self::October => 10,
            Self::November => 11,
            Self::December => 12,
        }
    }

    pub fn get_days_from_month(month: u8, year: i32) -> Result<u8, Error> {
        let month = Self::from_u8(month)?;

        Ok(month.valid_days_in_month(year))
    }

    pub fn is_valid_day_in_month(&self, day: u8, year: i32) -> bool {
        day <= self.valid_days_in_month(year)
    }

    pub fn has_31_days(&self) -> bool {
        match self {
            Self::January => true,
            Self::March => true,
            Self::May => true,
            Self::July => true,
            Self::August => true,
            Self::October => true,
            Self::December => true,
            _ => false,
        }
    }

    pub fn has_30_days(&self) -> bool {
        match self {
            Self::April => true,
            Self::June => true,
            Self::September => true,
            Self::November => true,
            _ => false,
        }
    }

    pub fn has_28_days(&self) -> bool {
        match self {
            Self::February => true,
            _ => false,
        }
    }
}

impl Month {
    fn dangerously_from_u8(month: u8) -> Self {
        match month {
            1 => Self::January,
            2 => Self::February,
            3 => Self::March,
            4 => Self::April,
            5 => Self::May,
            6 => Self::June,
            7 => Self::July,
            8 => Self::August,
            9 => Self::September,
            10 => Self::October,
            11 => Self::November,
            12 => Self::December,
            _ => panic!("Invalid month provided"),
        }
    }

    fn valid_days_in_month(&self, year: i32) -> u8 {
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

impl Display for Month {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_long())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Weekday {
    Sunday = 1,
    Monday = 2,
    Tuesday = 3,
    Wednesday = 4,
    Thursday = 5,
    Friday = 6,
    Saturday = 7,
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

    pub(super) fn dangerously_from_u8(day: u8) -> Self {
        match day {
            1 => Self::Sunday,
            2 => Self::Monday,
            3 => Self::Tuesday,
            4 => Self::Wednesday,
            5 => Self::Thursday,
            6 => Self::Friday,
            7 => Self::Saturday,
            _ => panic!("Invalid day provided"),
        }
    }

    pub fn as_long<'a>(&self) -> &'a str {
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

    pub fn as_short<'a>(&self) -> &'a str {
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

    pub fn as_u8(&self) -> u8 {
        match self {
            Self::Sunday => 1,
            Self::Monday => 2,
            Self::Tuesday => 3,
            Self::Wednesday => 4,
            Self::Thursday => 5,
            Self::Friday => 6,
            Self::Saturday => 7,
        }
    }
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

impl Display for Weekday {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_long())
    }
}

pub(self) fn pretty_format_day<'a>(day: u8) -> &'a str {
    match day {
        1 => "1st",
        2 => "2nd",
        3 => "3rd",
        4 => "4th",
        5 => "5th",
        6 => "6th",
        7 => "7th",
        8 => "8th",
        9 => "9th",
        10 => "10th",
        11 => "11th",
        12 => "12th",
        13 => "13th",
        14 => "14th",
        15 => "15th",
        16 => "16th",
        17 => "17th",
        18 => "18th",
        19 => "19th",
        20 => "20th",
        21 => "21st",
        22 => "22nd",
        23 => "23rd",
        24 => "24th",
        25 => "25th",
        26 => "26th",
        27 => "27th",
        28 => "28th",
        29 => "29th",
        30 => "30th",
        31 => "31st",
        _ => "th",
    }
}
