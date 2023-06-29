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

const ISO_8601_DATETIME: &str = "[year]-[month]-[day]T[hour]:[minute]:[second]";
const ISO_8601_DATE: &str = "[year]-[month]-[day]";
const ISO_8601_TIME: &str = "[hour]:[minute]:[second]";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DateTime {}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Date {
    month: u8,
    day: u8,
    year: i32,
}

impl Date {
    pub fn now() -> Result<Self, Error> {
        let date = OffsetDateTime::now_utc();

        let date = Self {
            month: date.month().into(),
            day: date.day(),
            year: date.year(),
        };

        if !date.is_valid() {
            return Err(Error::new("Invalid date provided", ErrorCode::Invalid));
        }

        Ok(date)
    }

    pub fn new(month: u8, day: u8, year: i32) -> Result<Self, Error> {
        let date = Self { month, day, year };

        if !date.is_valid() {
            return Err(Error::new("Invalid date provided", ErrorCode::Invalid));
        }

        Ok(date)
    }

    pub(self) fn from_date_offset(date: &OffsetDateTime) -> Self {
        Self {
            month: date.month().into(),
            day: date.day(),
            year: date.year(),
        }
    }

    pub fn month(&self) -> u8 {
        self.month
    }

    pub fn day(&self) -> u8 {
        self.day
    }

    pub fn year(&self) -> i32 {
        self.year
    }

    pub fn is_valid(&self) -> bool {
        Self::is_valid_month(self.month)
            && Self::is_valid_day(self.month, self.day, self.year)
            && Self::is_valid_year(self.year)
    }

    pub fn is_valid_month(month: u8) -> bool {
        month >= 1 && month <= 12
    }

    pub fn is_valid_day(month: u8, day: u8, year: i32) -> bool {
        if !Self::is_valid_month(month) {
            return false;
        }

        let month = match Months::from_u8(month) {
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
    /// use time::Date;
    ///
    /// let date = Date::from_str("2019-01-01").unwrap();
    /// assert_eq!(date.get_day_of_week(), 2);
    /// ```
    pub fn get_day_of_week(&self) -> i32 {
        let mut year = self.year;
        let mut month = self.month as i32;

        if month < 3 {
            month += 12;
            year -= 1;
        }

        let century = year / 100;
        let year_of_century = year % 100;

        let day_of_week = (self.day as i32
            + (((month + 1) * 26) / 10)
            + year_of_century
            + (year_of_century / 4)
            + (century / 4)
            - (2 * century))
            % 7;

        (day_of_week + 7) % 7
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Time {
    hour: u8,
    minute: u8,
    second: u8,
    millisecond: u16,
}

impl Time {
    pub fn now() -> Result<Self, Error> {
        let date = OffsetDateTime::now_utc();

        Ok(Self {
            hour: date.hour(),
            minute: date.minute(),
            second: date.second(),
            millisecond: date.millisecond(),
        })
    }

    pub fn new(hour: u8, minute: u8, second: u8, millisecond: u16) -> Result<Self, Error> {
        let time = Self {
            hour,
            minute,
            second,
            millisecond,
        };

        if !time.is_valid() {
            return Err(Error::new("Invalid time provided", ErrorCode::Invalid));
        }

        Ok(time)
    }

    pub(self) fn from_date_offset(date: &OffsetDateTime) -> Self {
        Self {
            hour: date.hour(),
            minute: date.minute(),
            second: date.second(),
            millisecond: date.millisecond(),
        }
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

    pub fn is_valid(&self) -> bool {
        let hour = self.hour();
        let minute = self.minute();
        let second = self.second();
        let millisecond = self.millisecond();

        if hour > 23 {
            return false;
        }

        if minute > 59 {
            return false;
        }

        if second > 59 {
            return false;
        }

        if millisecond > 999 {
            return false;
        }

        true
    }
}

pub enum DateTimeFormat {
    Date,
    Time,
    DateTime,
}

pub trait DateTimeExt {
    fn format(&self, format: DateTimeFormat) -> String;
}

pub enum Months {
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

impl From<Months> for u8 {
    fn from(month: Months) -> Self {
        match month {
            Months::January => 1,
            Months::February => 2,
            Months::March => 3,
            Months::April => 4,
            Months::May => 5,
            Months::June => 6,
            Months::July => 7,
            Months::August => 8,
            Months::September => 9,
            Months::October => 10,
            Months::November => 11,
            Months::December => 12,
        }
    }
}

impl Months {
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

    pub fn to_str<'a>(&self) -> &'a str {
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

pub enum Days {
    Sunday = 0,
    Monday = 1,
    Tuesday = 2,
    Wednesday = 3,
    Thursday = 4,
    Friday = 5,
    Saturday = 6,
}

impl From<u8> for Days {
    fn from(day: u8) -> Self {
        match day {
            0 => Self::Sunday,
            1 => Self::Monday,
            2 => Self::Tuesday,
            3 => Self::Wednesday,
            4 => Self::Thursday,
            5 => Self::Friday,
            6 => Self::Saturday,
            _ => panic!("Invalid day provided"),
        }
    }
}

impl From<Days> for u8 {
    fn from(day: Days) -> Self {
        match day {
            Days::Sunday => 0,
            Days::Monday => 1,
            Days::Tuesday => 2,
            Days::Wednesday => 3,
            Days::Thursday => 4,
            Days::Friday => 5,
            Days::Saturday => 6,
        }
    }
}

impl Days {
    pub fn to_str<'a>(&self) -> &'a str {
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
}
