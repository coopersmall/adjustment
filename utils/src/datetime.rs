use ::time::OffsetDateTime;
use serde::{Deserialize, Serialize};

use crate::errors::{Error, ErrorCode};

pub mod date;
pub mod helpers;
pub mod primatives;
pub mod time;
pub mod timer;

pub use self::date::Date;
use self::primatives::{Day, Hour, Millisecond, Minute, Month, Second, Weekday, Year};
pub use self::time::{Offset, Time};

pub enum DateTimeFormat {
    ISO8601,
    RFC2822,
    RFC3339,
    PRETTY,
}

pub type DateFormatResult = Result<Box<str>, Error>;

pub trait Format {
    fn format(&self, format: &DateTimeFormat) -> DateFormatResult;
}

pub trait FormatNow {
    fn format_now(format: &DateTimeFormat) -> Box<str>;
}

pub trait FormatLocal {
    fn format_local(format: &DateTimeFormat) -> DateFormatResult;
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct DateTime {
    date: Date,
    time: Time,
}

impl DateTime {
    pub fn new(
        year: i32,
        month: u8,
        day: u8,
        hour: u8,
        minute: u8,
        second: u8,
        milliseconds: Option<u16>,
        offset: Option<i32>,
    ) -> Result<Self, Error> {
        let date = Date::new(year, month, day)?;
        let time = Time::new(hour, minute, second, milliseconds, offset)?;

        let date_time = Self { date, time };

        Ok(date_time)
    }

    pub fn now() -> Self {
        let now = OffsetDateTime::now_utc();
        let date = Date::from_offset_time(&now);
        let time = Time::from_offset_time(&now);

        Self { date, time }
    }

    pub fn local() -> Result<Self, Error> {
        let now = OffsetDateTime::now_local();
        let now = match now {
            Ok(now) => now,
            Err(error) => {
                return Err(
                    Error::new("Failed to get local time", ErrorCode::DateTimeCreation)
                        .with_cause(error),
                )
            }
        };

        let date = Date::from_offset_time(&now);
        let time = Time::from_offset_time(&now);

        let date_time = Self { date, time };

        Ok(date_time)
    }

    pub fn date(&self) -> &Date {
        &self.date
    }

    pub fn time(&self) -> &Time {
        &self.time
    }

    pub fn year(&self) -> &Year {
        self.date.year()
    }

    pub fn month(&self) -> &Month {
        self.date.month()
    }

    pub fn day(&self) -> &Day {
        self.date.day()
    }

    pub fn weekday(&self) -> &Weekday {
        self.date.weekday()
    }

    pub fn hour(&self) -> &Hour {
        self.time.hour()
    }

    pub fn minute(&self) -> &Minute {
        self.time.minute()
    }

    pub fn second(&self) -> &Second {
        self.time.second()
    }

    pub fn millisecond(&self) -> Option<&Millisecond> {
        self.time.millisecond()
    }

    pub fn offset(&self) -> Option<&Offset> {
        self.time.offset()
    }

    pub fn is_valid(
        year: i32,
        month: u8,
        day: u8,
        hour: u8,
        minute: u8,
        second: u8,
        milliseconds: u16,
        offset: i32,
    ) -> bool {
        Date::is_valid(year, month, day)
            && Time::is_valid(hour, minute, second, milliseconds, offset)
    }
}

impl Format for DateTime {
    /// Formats the date time using the given format.
    /// # Examples
    /// ```
    /// use utils::datetime::*;
    ///
    /// let date = Date::new(2020, 1, 1).unwrap();
    /// let time = Time::new(0, 0, 0, 0, 0).unwrap();
    ///
    /// let date_time = DateTime::new(date, time).unwrap();
    ///
    /// let iso8601 = date_time.format(&DateTimeFormat::ISO8601).unwrap();
    /// assert_eq!(iso8601.as_ref(), "2020-01-01T00:00:00.000+00:00");
    ///
    /// let pretty = date_time.format(&DateTimeFormat::PRETTY).unwrap();
    /// assert_eq!(pretty.as_ref(), "Wed, January 1st 2020 00:00:00 AM UTC");
    ///
    /// let rfc2822 = date_time.format(&DateTimeFormat::RFC2822).unwrap();
    /// assert_eq!(rfc2822.as_ref(), "Wed, 01 Jan 2020 00:00:00 +00:00");
    ///
    /// let rfc3339 = date_time.format(&DateTimeFormat::RFC3339).unwrap();
    /// assert_eq!(rfc3339.as_ref(), "2020-01-01T00:00:00.000+00:00");
    /// ```

    fn format(&self, format: &DateTimeFormat) -> DateFormatResult {
        let date = self.date.format(format)?;
        let time = self.time.format(format)?;

        Ok(Self::shared_format(format, date, time))
    }
}

impl FormatNow for DateTime {
    fn format_now(format: &DateTimeFormat) -> Box<str> {
        let date = Date::format_now(format);
        let time = Time::format_now(format);

        Self::shared_format(format, date, time)
    }
}

impl FormatLocal for DateTime {
    fn format_local(format: &DateTimeFormat) -> DateFormatResult {
        let date = Date::format_local(format)?;
        let time = Time::format_local(format)?;

        Ok(Self::shared_format(format, date, time))
    }
}

impl DateTime {
    fn shared_format(format: &DateTimeFormat, date: Box<str>, time: Box<str>) -> Box<str> {
        let separator = match format {
            DateTimeFormat::ISO8601 => "T",
            DateTimeFormat::PRETTY => " ",
            DateTimeFormat::RFC2822 => " ",
            DateTimeFormat::RFC3339 => "T",
        };

        let mut string = String::new();

        string.push_str(date.as_ref());
        string.push_str(separator);
        string.push_str(time.as_ref());

        string.into_boxed_str()
    }
}
