use serde::{Deserialize, Serialize};

use crate::errors::{Error, ErrorCode};

pub mod date;
pub mod time;

pub use self::date::{Date, Month, Weekday};
pub use self::time::{Offset, Time};

pub enum DateTimeFormat {
    ISO8601,
    RFC2822,
    RFC3339,
    PRETTY,
}

pub(self) type DateFormatResult = Result<Box<str>, Error>;

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
    pub fn new(date: Date, time: Time) -> Result<Self, Error> {
        if !DateTime::is_valid(&date, &time) {
            return Err(Error::new("Invalid date time", ErrorCode::Invalid));
        }

        let date_time = Self { date, time };

        Ok(date_time)
    }

    pub fn now() -> Result<Self, Error> {
        let date = Date::now();
        let time = Time::now();

        DateTime::new(date, time)
    }

    pub fn local() -> Result<Self, Error> {
        let date = Date::local()?;
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
        self.time.seconds()
    }

    pub fn millisecond(&self) -> u16 {
        self.time.milliseconds()
    }

    pub fn offset(&self) -> i32 {
        self.time.offset()
    }

    pub fn is_valid(date: &Date, time: &Time) -> bool {
        Date::is_valid(date.year(), date.month(), date.day())
            && Time::is_valid(
                time.hour(),
                time.minute(),
                time.seconds(),
                time.milliseconds(),
                time.offset(),
            )
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
