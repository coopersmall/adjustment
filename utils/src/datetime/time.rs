use chrono::{FixedOffset, Offset as ChronoOffset, TimeZone};
use chrono_tz::TZ_VARIANTS;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use std::fmt::{Display, Formatter};

use super::{DateFormatResult, DateTimeFormat, Format, FormatLocal, FormatNow};
use crate::errors::{Error, ErrorCode};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Time {
    hour: u8,
    minute: u8,
    seconds: u8,
    milliseconds: u16,
    offset: Offset,
}

impl Time {
    pub fn new(
        hour: u8,
        minute: u8,
        seconds: u8,
        milliseconds: u16,
        offset: i32,
    ) -> Result<Self, Error> {
        if !Time::is_valid(hour, minute, seconds, milliseconds, offset) {
            return Err(Error::new("Invalid datetime", ErrorCode::Invalid));
        }

        let offset = match Offset::from_seconds(offset) {
            Ok(offset) => offset,
            Err(err) => return Err(Error::new(err.to_string().as_str(), ErrorCode::Invalid)),
        };

        let datetime = Self {
            hour,
            minute,
            seconds,
            milliseconds,
            offset,
        };

        Ok(datetime)
    }

    /// Returns the current datetime in UTC.
    ///
    /// # Examples
    /// ```
    /// use utils::datetime::time::Time;
    ///
    /// let time = Time::now();
    /// assert!(Time::is_valid(time.hour(), time.minute(), time.seconds(), time.milliseconds(), time.offset()));
    /// ```

    pub fn now() -> Self {
        let date = OffsetDateTime::now_utc();

        let hour = date.hour() as u8;
        let minute = date.minute() as u8;
        let seconds = date.second() as u8;
        let milliseconds = date.millisecond() as u16;
        let offset = date.offset().whole_seconds() as i32;
        let offset = Offset::dangerously_from_seconds(offset);

        Self {
            hour,
            minute,
            seconds,
            milliseconds,
            offset,
        }
    }

    /// Returns the current datetime in the local offset.
    /// This method is only valid for timezones that are offset from UTC by whole minutes.
    ///
    /// # Examples
    /// ```
    /// use utils::datetime::time::Time;
    ///
    /// let datetime = Time::local().unwrap();
    /// assert!(Time::is_valid(datetime.hour(), datetime.minute(), datetime.seconds(), datetime.milliseconds(), datetime.offset() as i32));
    /// ```

    pub fn local() -> Result<Self, Error> {
        let date = match OffsetDateTime::now_local() {
            Ok(date) => date,
            Err(err) => return Err(Error::new(err.to_string().as_str(), ErrorCode::Invalid)),
        };

        let hour = date.hour() as u8;
        let minute = date.minute() as u8;
        let seconds = date.second() as u8;
        let milliseconds = date.millisecond() as u16;
        let offset = date.offset().whole_seconds() as i32;

        Time::new(hour, minute, seconds, milliseconds, offset)
    }

    pub fn hour(&self) -> u8 {
        self.hour
    }

    pub fn minute(&self) -> u8 {
        self.minute
    }

    pub fn seconds(&self) -> u8 {
        self.seconds
    }

    pub fn milliseconds(&self) -> u16 {
        self.milliseconds
    }

    pub fn offset(&self) -> i32 {
        self.offset.as_seconds()
    }

    /// Checks if the datetime is valid.
    ///
    /// - This method is only valid for times after 00:00:00.000 and before 23:59:59.999.
    /// - This method is only valid for offsets between -12:00 and +14:00.
    ///
    /// # Examples
    /// ```
    /// use utils::datetime::time::Time;
    /// use utils::errors::ErrorCode;
    ///
    /// let hour = 23;
    /// let minute = 59;
    /// let seconds = 59;
    /// let milliseconds = 999;
    /// let offset = 14 * 60 * 60;
    ///
    /// assert!(Time::is_valid(hour, minute, seconds, milliseconds, offset));
    ///
    /// let hour = 24;
    /// assert!(!Time::is_valid(hour, minute, seconds, milliseconds, offset));
    /// ```
    ///

    pub fn is_valid(hour: u8, minute: u8, seconds: u8, milliseconds: u16, offset: i32) -> bool {
        Self::is_valid_hour(hour)
            && Self::is_valid_minute(minute)
            && Self::is_valid_second(seconds)
            && Self::is_valid_millisecond(milliseconds)
            && Self::is_valid_offset(offset)
    }

    pub fn is_valid_hour(hour: u8) -> bool {
        hour <= 23
    }

    pub fn is_valid_minute(minute: u8) -> bool {
        minute <= 59
    }

    pub fn is_valid_second(seconds: u8) -> bool {
        seconds <= 59
    }

    pub fn is_valid_millisecond(milliseconds: u16) -> bool {
        milliseconds <= 999
    }

    pub fn is_valid_offset(offset: i32) -> bool {
        offset >= -43200 && offset <= 50400
    }
}

impl Time {
    fn shared_format(&self, format: &DateTimeFormat, offset: Box<str>) -> Box<str> {
        let mut string = String::new();

        match format {
            DateTimeFormat::ISO8601 => {
                string.push_str(&format!("{:02}", self.hour));
                string.push(':');
                string.push_str(&format!("{:02}", self.minute));
                string.push(':');
                string.push_str(&format!("{:02}", self.seconds));
                string.push('.');
                string.push_str(&format!("{:03}", self.milliseconds));
                string.push_str(&offset);

                string.into_boxed_str()
            }

            DateTimeFormat::PRETTY => {
                string.push_str(&format!("{:02}", pretty_format_hour(self.hour)));
                string.push(':');
                string.push_str(&format!("{:02}", self.minute));
                string.push(':');
                string.push_str(&format!("{:02}", self.seconds));
                string.push(' ');
                let meridiem = if self.hour < 12 { "AM" } else { "PM" };
                string.push_str(meridiem);
                string.push_str(" ");
                string.push_str(&offset);

                string.into_boxed_str()
            }

            DateTimeFormat::RFC2822 => {
                let mut string = String::new();

                string.push_str(&format!("{:02}", self.hour));
                string.push(':');
                string.push_str(&format!("{:02}", self.minute));
                string.push(':');
                string.push_str(&format!("{:02}", self.seconds));
                string.push(' ');
                string.push_str(&offset);

                string.into_boxed_str()
            }

            DateTimeFormat::RFC3339 => {
                let mut string = String::new();

                string.push_str(&format!("{:02}", self.hour));
                string.push(':');
                string.push_str(&format!("{:02}", self.minute));
                string.push(':');
                string.push_str(&format!("{:02}", self.seconds));
                string.push('.');
                string.push_str(&format!("{:03}", self.milliseconds));
                string.push_str(&offset);

                string.into_boxed_str()
            }
        }
    }
}

impl Format for Time {
    fn format(&self, format: &DateTimeFormat) -> DateFormatResult {
        let offset = self.offset.format(format)?;
        Ok(Time::shared_format(self, format, offset))
    }
}

impl FormatNow for Time {
    fn format_now(format: &DateTimeFormat) -> Box<str> {
        let now = Time::now();
        let offset = now.offset.format_now(format);

        now.shared_format(format, offset)
    }
}

impl FormatLocal for Time {
    fn format_local(format: &DateTimeFormat) -> DateFormatResult {
        let local = Time::local()?;
        let offset = local.offset.format(format)?;

        Ok(local.shared_format(format, offset))
    }
}

impl Display for Time {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let offset = Offset::shared_format(&self.offset, &DateTimeFormat::ISO8601);
        write!(
            f,
            "{}",
            Time::shared_format(self, &DateTimeFormat::ISO8601, offset)
        )
    }
}

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Offset(i32);

impl Offset {
    /// Returns the offset in from_seconds
    ///
    /// # Examples
    /// ```
    /// use utils::datetime::time::Offset;
    ///
    /// let offset = Offset::from_seconds(0);
    /// assert!(offset.is_ok());
    ///
    /// let offset = Offset::from_seconds(3600);
    /// assert!(offset.is_ok());
    /// ```

    pub fn from_seconds(from_seconds: i32) -> Result<Self, Error> {
        if !Offset::is_valid_seconds(&from_seconds) {
            return Err(Error::new("Invalid offset provided", ErrorCode::Invalid));
        }
        Ok(Self(from_seconds))
    }

    pub fn as_seconds(&self) -> i32 {
        self.0
    }

    /// Checks if the offset is valid
    /// The offset must be between -43200 and 50400
    ///
    /// # Examples
    /// ```
    /// use utils::datetime::time::Offset;
    ///
    /// let seconds = 3600;
    /// assert!(Offset::is_valid_seconds(&seconds));
    ///
    /// let seconds = -43201;
    /// assert!(!Offset::is_valid_seconds(&seconds));
    /// ```

    pub fn is_valid_seconds(from_seconds: &i32) -> bool {
        *from_seconds >= -43200 && *from_seconds <= 50400
    }

    /// Returns the timezone abbreviation for the given offset
    /// If the timezone abbreviation cannot be found, None is returned
    ///
    /// # Examples
    /// ```
    /// use utils::datetime::time::Offset;
    ///
    /// let offset = Offset::from_seconds(0).unwrap();
    /// assert_eq!(offset.get_timezone_abbreviation(), Some("UTC".into()));
    ///
    /// let offset = Offset::from_seconds(18000).unwrap();
    /// assert_eq!(offset.get_timezone_abbreviation(), Some("EST".into()));
    /// ```

    pub fn get_timezone_abbreviation(&self) -> Option<Box<str>> {
        if self.0 < -43200 || self.0 > 50400 {
            return None;
        }

        if self.0 == 0 {
            return Some("UTC".into());
        }

        if let Some(tz) = find_common_tz_from_seconds(self.0) {
            return Some(tz.into());
        }

        let offset = match FixedOffset::west_opt(self.0) {
            Some(offset) => offset,
            None => return None,
        };

        let now_utc = chrono::Utc::now().naive_utc();

        TZ_VARIANTS
            .iter()
            .find(|tz| tz.offset_from_utc_datetime(&now_utc).fix() == offset)
            .map(|tz| tz.name().into())
    }
}

impl Offset {
    fn dangerously_from_seconds(from_seconds: i32) -> Self {
        Self(from_seconds)
    }

    fn format_now(&self, format: &DateTimeFormat) -> Box<str> {
        Offset::shared_format(self, format)
    }

    fn shared_format(offset: &Offset, format: &DateTimeFormat) -> Box<str> {
        let mut string = String::new();

        match format {
            DateTimeFormat::ISO8601 => {
                string.push_str(if offset.0 < 0 { "-" } else { "+" });

                string.push_str(&format!("{:02}", offset.0.abs() / 3600));
                string.push(':');
                string.push_str(&format!("{:02}", (offset.0.abs() % 3600) / 60));

                string.into_boxed_str()
            }

            DateTimeFormat::PRETTY => match offset.get_timezone_abbreviation() {
                Some(tz) => tz,
                None => return string.into_boxed_str(),
            },

            DateTimeFormat::RFC2822 => {
                string.push_str(if offset.0 < 0 { "-" } else { "+" });

                string.push_str(&format!("{:02}", offset.0.abs() / 3600));
                string.push(':');
                string.push_str(&format!("{:02}", (offset.0.abs() % 3600) / 60));

                string.into_boxed_str()
            }

            DateTimeFormat::RFC3339 => {
                string.push_str(if offset.0 < 0 { "-" } else { "+" });

                string.push_str(&format!("{:02}", offset.0.abs() / 3600));
                string.push(':');
                string.push_str(&format!("{:02}", (offset.0.abs() % 3600) / 60));

                string.into_boxed_str()
            }
        }
    }
}

impl Format for Offset {
    /// Returns the name of the timezone
    ///
    /// # Examples
    /// ```
    /// use utils::datetime::{Format, DateTimeFormat};
    /// use utils::datetime::time::Offset;
    ///
    /// let format = DateTimeFormat::ISO8601;
    ///
    /// let offset = Offset::from_seconds(0).unwrap().format(&format).unwrap();
    /// assert_eq!(offset.as_ref(), "+00:00");
    ///
    /// let offset = Offset::from_seconds(50400).unwrap().format(&format).unwrap();
    /// assert_eq!(offset.as_ref(), "+14:00");
    ///
    /// let offset = Offset::from_seconds(-43200).unwrap().format(&format).unwrap();
    /// assert_eq!(offset.as_ref(), "-12:00");
    /// ```

    fn format(&self, f: &DateTimeFormat) -> DateFormatResult {
        Ok(Offset::shared_format(self, f))
    }
}

impl Display for Offset {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            Offset::shared_format(self, &DateTimeFormat::ISO8601)
        )
    }
}

pub(self) fn find_common_tz_from_seconds(seconds: i32) -> Option<&'static str> {
    match seconds {
        0 => Some("UTC"),
        18000 => Some("EST"),
        21600 => Some("CST"),
        25200 => Some("MST"),
        28800 => Some("PST"),
        -32400 => Some("AKST"),
        -36000 => Some("HST"),
        _ => None,
    }
}

pub fn pretty_format_hour(hour: u8) -> u8 {
    if hour > 12 {
        return hour - 12;
    }
    hour
}
