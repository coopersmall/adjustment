use serde::{Deserialize, Serialize};
use std::{
    fmt::{Display, Formatter},
    ops::{Add, AddAssign, Sub, SubAssign},
};

use crate::errors::{Error, ErrorCode};

use super::Date;

const UNIX_EPOCH_YEAR: u8 = 1970;
const MONTHS_PER_YEAR: u8 = 12;
const SECOND_PER_DAY: u16 = 86400;
const SECONDS_PER_HOUR: u8 = 3600;
const SECONDS_PER_MINUTE: u8 = 60;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Serialize, Deserialize)]
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
    pub fn from_u8(month: u8) -> Result<Self, Error> {
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

    pub fn is_valid_day(&self, day: &u8, year: &i32) -> bool {
        day <= &self.valid_days_in_month(*year)
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

    pub fn unix(&self, year: &Year) -> u32 {
        let mut unix = 0 as u32;

        Self::January
            .into_iter()
            .filter(|month| month < self)
            .for_each(|month| {
                let days = month.day_count(year);
                unix += days as u32 * SECOND_PER_DAY as u32;
            });

        unix
    }

    pub fn day_count(&self, year: &Year) -> u8 {
        Month::valid_days_in_month(&self, year.as_i32())
    }

    pub fn last_day(&self, year: &Year) -> Day {
        let days = Month::valid_days_in_month(&self, year.as_i32());
        Day::dangerously_from_u8(days)
    }
}

impl Month {
    pub fn is_last_day(&self, year: &Year, day: &Day) -> bool {
        day == &self.valid_days_in_month(year.as_i32())
    }
}

impl Month {
    pub(super) fn dangerously_from_u8(month: u8) -> Self {
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

impl Iterator for Month {
    type Item = Month;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::January => {
                *self = Self::February;
                Some(Self::January)
            }
            Self::February => {
                *self = Self::March;
                Some(Self::February)
            }
            Self::March => {
                *self = Self::April;
                Some(Self::March)
            }
            Self::April => {
                *self = Self::May;
                Some(Self::April)
            }
            Self::May => {
                *self = Self::June;
                Some(Self::May)
            }
            Self::June => {
                *self = Self::July;
                Some(Self::June)
            }
            Self::July => {
                *self = Self::August;
                Some(Self::July)
            }
            Self::August => {
                *self = Self::September;
                Some(Self::August)
            }
            Self::September => {
                *self = Self::October;
                Some(Self::September)
            }
            Self::October => {
                *self = Self::November;
                Some(Self::October)
            }
            Self::November => {
                *self = Self::December;
                Some(Self::November)
            }
            Self::December => None,
        }
    }
}

impl DoubleEndedIterator for Month {
    fn next_back(&mut self) -> Option<Self::Item> {
        match self {
            Self::January => None,
            Self::February => {
                *self = Self::January;
                Some(Self::February)
            }
            Self::March => {
                *self = Self::February;
                Some(Self::March)
            }
            Self::April => {
                *self = Self::March;
                Some(Self::April)
            }
            Self::May => {
                *self = Self::April;
                Some(Self::May)
            }
            Self::June => {
                *self = Self::May;
                Some(Self::June)
            }
            Self::July => {
                *self = Self::June;
                Some(Self::July)
            }
            Self::August => {
                *self = Self::July;
                Some(Self::August)
            }
            Self::September => {
                *self = Self::August;
                Some(Self::September)
            }
            Self::October => {
                *self = Self::September;
                Some(Self::October)
            }
            Self::November => {
                *self = Self::October;
                Some(Self::November)
            }
            Self::December => {
                *self = Self::November;
                Some(Self::December)
            }
        }
    }
}

impl Display for Month {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            _ => write!(f, "{}", self.as_u8()),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialOrd, Ord, Serialize, Deserialize)]
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

    pub fn from_values(year: i32, month: u8, day: u8) -> Result<Self, Error> {
        if month < 3 {
            month += 12;
        }

        let century = year / 100;
        let year_of_century = year % 100;

        let weekday = (day as i32
            + (((month as i32 + 1) * 26) / 10)
            + year_of_century
            + (year_of_century / 4)
            + (century / 4)
            - (2 * century))
            % 7;

        let weekday = ((weekday + 7) % 7) as u8;

        Self::from_u8(weekday)
    }

    pub(super) fn dangerously_from_values(year: i32, month: u8, day: u8) -> Self {
        if month < 3 {
            month += 12;
        }

        let century = year / 100;
        let year_of_century = year % 100;

        let weekday = (day as i32
            + (((month as i32 + 1) * 26) / 10)
            + year_of_century
            + (year_of_century / 4)
            + (century / 4)
            - (2 * century))
            % 7;

        let weekday = ((weekday + 7) % 7) as u8;

        Self::dangerously_from_u8(weekday)
    }

    pub fn from_date(date: &Date) -> Result<Self, Error> {
        Self::from_values(
            date.year().as_i32(),
            date.month().as_u8(),
            date.day().as_u8(),
        )
    }

    pub(super) fn dangerously_from_date(date: &Date) -> Self {
        Self::dangerously_from_values(
            date.year().as_i32(),
            date.month().as_u8(),
            date.day().as_u8(),
        )
    }
    pub(self) fn dangerously_from_u8(day: u8) -> Self {
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

impl Iterator for Weekday {
    type Item = Weekday;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Sunday => {
                *self = Self::Monday;
                Some(Self::Sunday)
            }
            Self::Monday => {
                *self = Self::Tuesday;
                Some(Self::Monday)
            }
            Self::Tuesday => {
                *self = Self::Wednesday;
                Some(Self::Tuesday)
            }
            Self::Wednesday => {
                *self = Self::Thursday;
                Some(Self::Wednesday)
            }
            Self::Thursday => {
                *self = Self::Friday;
                Some(Self::Thursday)
            }
            Self::Friday => {
                *self = Self::Saturday;
                Some(Self::Friday)
            }
            Self::Saturday => {
                *self = Self::Sunday;
                Some(Self::Saturday)
            }
        }
    }
}

impl DoubleEndedIterator for Weekday {
    fn next_back(&mut self) -> Option<Self::Item> {
        match self {
            Self::Sunday => {
                *self = Self::Saturday;
                Some(Self::Sunday)
            }
            Self::Monday => {
                *self = Self::Sunday;
                Some(Self::Monday)
            }
            Self::Tuesday => {
                *self = Self::Monday;
                Some(Self::Tuesday)
            }
            Self::Wednesday => {
                *self = Self::Tuesday;
                Some(Self::Wednesday)
            }
            Self::Thursday => {
                *self = Self::Wednesday;
                Some(Self::Thursday)
            }
            Self::Friday => {
                *self = Self::Thursday;
                Some(Self::Friday)
            }
            Self::Saturday => {
                *self = Self::Friday;
                Some(Self::Saturday)
            }
        }
    }
}

impl Display for Weekday {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_long())
    }
}

impl PartialEq<Weekday> for Weekday {
    fn eq(&self, other: &Weekday) -> bool {
        self.as_u8() == other.as_u8()
    }
}

impl PartialEq<u8> for Weekday {
    fn eq(&self, other: &u8) -> bool {
        self.as_u8() == *other
    }
}

pub enum DaysInMonth {
    ThirtyOne = 31,
    Thirty = 30,
    TwentyNine = 29,
    TwentyEight = 28,
}

impl DaysInMonth {
    pub fn from_month(month: Month, year: i32) -> Self {
        match month {
            Month::January => Self::ThirtyOne,
            Month::February => {
                if year % 4 == 0 {
                    Self::TwentyNine
                } else {
                    Self::TwentyEight
                }
            }
            Month::March => Self::ThirtyOne,
            Month::April => Self::Thirty,
            Month::May => Self::ThirtyOne,
            Month::June => Self::Thirty,
            Month::July => Self::ThirtyOne,
            Month::August => Self::ThirtyOne,
            Month::September => Self::Thirty,
            Month::October => Self::ThirtyOne,
            Month::November => Self::Thirty,
            Month::December => Self::ThirtyOne,
        }
    }

    pub fn to_months(&self) -> Box<[Month]> {
        match self {
            Self::ThirtyOne => Box::new([
                Month::January,
                Month::March,
                Month::May,
                Month::July,
                Month::August,
                Month::October,
                Month::December,
            ]),
            Self::Thirty => {
                Box::new([Month::April, Month::June, Month::September, Month::November])
            }
            Self::TwentyNine => Box::new([Month::February]),
            Self::TwentyEight => Box::new([Month::February]),
        }
    }

    pub fn as_u8(&self) -> u8 {
        match self {
            Self::ThirtyOne => 31,
            Self::Thirty => 30,
            Self::TwentyNine => 29,
            Self::TwentyEight => 28,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, Hash, Ord, Serialize, Deserialize)]
pub struct Day(u8);

impl Day {
    pub fn first() -> Self {
        Self(1)
    }

    pub fn last(month: Month, year: i32) -> Self {
        DaysInMonth::from_month(month, year).into()
    }

    pub fn from_u8(day: u8) -> Result<Self, Error> {
        if day > 31 {
            return Err(Error::new("Invalid day provided", ErrorCode::Invalid));
        }

        Ok(Self(day))
    }

    pub fn as_u8(&self) -> u8 {
        self.0
    }

    pub fn as_str(&self) -> &str {
        self.0.to_string().as_str()
    }

    pub fn pretty_format(&self) -> &str {
        match self.0 {
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

    pub fn is_valid(&self, month: Month, year: Year) -> bool {
        let last_day = month.last_day(&year);

        self <= &last_day
    }

    pub fn unix(&self, year: &Year, month: &Month) -> u32 {
        let mut unix = 0;

        let days_in_month = month.last_day(year);

        Day::first()
            .into_iter()
            .filter(|day| day.is_valid(*month, *year) && day < self)
            .for_each(|_| {
                unix += 86400;
            });

        unix
    }
}

impl Iterator for Day {
    type Item = Day;

    fn next(&mut self) -> Option<Self::Item> {
        match self.0 {
            1 => {
                *self = Self(2);
                Some(Self(1))
            }
            2 => {
                *self = Self(3);
                Some(Self(2))
            }
            3 => {
                *self = Self(4);
                Some(Self(3))
            }
            4 => {
                *self = Self(5);
                Some(Self(4))
            }
            5 => {
                *self = Self(6);
                Some(Self(5))
            }
            6 => {
                *self = Self(7);
                Some(Self(6))
            }
            7 => {
                *self = Self(8);
                Some(Self(7))
            }
            8 => {
                *self = Self(9);
                Some(Self(8))
            }
            9 => {
                *self = Self(10);
                Some(Self(9))
            }
            10 => {
                *self = Self(11);
                Some(Self(10))
            }
            11 => {
                *self = Self(12);
                Some(Self(11))
            }
            12 => {
                *self = Self(13);
                Some(Self(12))
            }
            13 => {
                *self = Self(14);
                Some(Self(13))
            }
            14 => {
                *self = Self(15);
                Some(Self(14))
            }
            15 => {
                *self = Self(16);
                Some(Self(15))
            }
            16 => {
                *self = Self(17);
                Some(Self(16))
            }
            17 => {
                *self = Self(18);
                Some(Self(17))
            }
            18 => {
                *self = Self(19);
                Some(Self(18))
            }
            19 => {
                *self = Self(20);
                Some(Self(19))
            }
            20 => {
                *self = Self(21);
                Some(Self(20))
            }
            21 => {
                *self = Self(22);
                Some(Self(21))
            }
            22 => {
                *self = Self(23);
                Some(Self(22))
            }
            23 => {
                *self = Self(24);
                Some(Self(23))
            }
            24 => {
                *self = Self(25);
                Some(Self(24))
            }
            25 => {
                *self = Self(26);
                Some(Self(25))
            }
            26 => {
                *self = Self(27);
                Some(Self(26))
            }
            27 => {
                *self = Self(28);
                Some(Self(27))
            }
            28 => {
                *self = Self(29);
                Some(Self(28))
            }
            29 => {
                *self = Self(30);
                Some(Self(29))
            }
            30 => {
                *self = Self(31);
                Some(Self(30))
            }
            31 => {
                *self = Self(1);
                Some(Self(31))
            }
            _ => None,
        }
    }
}

impl DoubleEndedIterator for Day {
    fn next_back(&mut self) -> Option<Self::Item> {
        match self.0 {
            1 => {
                *self = Self(31);
                Some(Self(1))
            }
            2 => {
                *self = Self(1);
                Some(Self(2))
            }
            3 => {
                *self = Self(2);
                Some(Self(3))
            }
            4 => {
                *self = Self(3);
                Some(Self(4))
            }
            5 => {
                *self = Self(4);
                Some(Self(5))
            }
            6 => {
                *self = Self(5);
                Some(Self(6))
            }
            7 => {
                *self = Self(6);
                Some(Self(7))
            }
            8 => {
                *self = Self(7);
                Some(Self(8))
            }
            9 => {
                *self = Self(8);
                Some(Self(9))
            }
            10 => {
                *self = Self(9);
                Some(Self(10))
            }
            11 => {
                *self = Self(10);
                Some(Self(11))
            }
            12 => {
                *self = Self(11);
                Some(Self(12))
            }
            13 => {
                *self = Self(12);
                Some(Self(13))
            }
            14 => {
                *self = Self(13);
                Some(Self(14))
            }
            15 => {
                *self = Self(14);
                Some(Self(15))
            }
            16 => {
                *self = Self(15);
                Some(Self(16))
            }
            17 => {
                *self = Self(16);
                Some(Self(17))
            }
            18 => {
                *self = Self(17);
                Some(Self(18))
            }
            19 => {
                *self = Self(18);
                Some(Self(19))
            }
            20 => {
                *self = Self(19);
                Some(Self(20))
            }
            21 => {
                *self = Self(20);
                Some(Self(21))
            }
            22 => {
                *self = Self(21);
                Some(Self(22))
            }
            23 => {
                *self = Self(22);
                Some(Self(23))
            }
            24 => {
                *self = Self(23);
                Some(Self(24))
            }
            25 => {
                *self = Self(24);
                Some(Self(25))
            }
            26 => {
                *self = Self(25);
                Some(Self(26))
            }
            27 => {
                *self = Self(26);
                Some(Self(27))
            }
            28 => {
                *self = Self(27);
                Some(Self(28))
            }
            29 => {
                *self = Self(28);
                Some(Self(29))
            }
            30 => {
                *self = Self(29);
                Some(Self(30))
            }
            31 => {
                *self = Self(30);
                Some(Self(31))
            }
            _ => None,
        }
    }
}

impl Day {
    pub(super) fn dangerously_from_u8(day: u8) -> Self {
        Self(day)
    }
}

impl Display for Day {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_u8())
    }
}

impl PartialEq<Day> for Day {
    fn eq(&self, other: &Day) -> bool {
        self.as_u8() == other.as_u8()
    }
}

impl PartialEq<u8> for Day {
    fn eq(&self, other: &u8) -> bool {
        self.as_u8() == *other
    }
}

impl PartialOrd<Day> for Day {
    fn partial_cmp(&self, other: &Day) -> Option<std::cmp::Ordering> {
        self.as_u8().partial_cmp(&other.as_u8())
    }
}

impl PartialOrd<u8> for Day {
    fn partial_cmp(&self, other: &u8) -> Option<std::cmp::Ordering> {
        self.as_u8().partial_cmp(other)
    }
}

impl From<DaysInMonth> for Day {
    fn from(days_in_month: DaysInMonth) -> Self {
        Self(days_in_month.as_u8())
    }
}

impl From<Day> for u8 {
    fn from(day: Day) -> Self {
        day.as_u8()
    }
}

pub enum DaysInYear {
    Leap = 366,
    Regular = 365,
}

impl DaysInYear {
    pub fn from_year(year: &Year) -> Self {
        if year.is_leap_year() {
            Self::Leap
        } else {
            Self::Regular
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, Hash, Ord, Serialize, Deserialize)]
pub struct Year(i32);

impl Year {
    pub fn from_i32(year: i32) -> Result<Self, Error> {
        if year < 0 {
            return Err(Error::new("Invalid year provided", ErrorCode::Invalid));
        }

        Ok(Self(year))
    }

    pub fn as_i32(&self) -> i32 {
        self.0
    }

    pub fn as_str(&self) -> &str {
        self.0.to_string().as_str()
    }

    pub fn is_leap_year(&self) -> bool {
        self.0 % 4 == 0
    }

    pub fn next_leap_year(&self) -> Self {
        let mut year = self.0 + 1;

        while year % 4 != 0 {
            year += 1;
        }

        Self(year)
    }

    pub fn is_next_leap_year(&self) -> bool {
        let mut year = self.0 + 1;

        while year % 4 != 0 {
            year += 1;
        }

        year == self.0
    }

    pub fn unix(&self) -> u32 {
        let mut unix = 0;

        for year in UNIX_EPOCH_YEAR as i32..self.0 {
            let year = Self(year);

            unix += DaysInYear::from_year(&year) as u32 * SECOND_PER_DAY as u32;
        }

        unix
    }
}

impl Iterator for Year {
    type Item = Year;

    fn next(&mut self) -> Option<Self::Item> {
        let year = self.0 + 1;

        self.0 = year;

        Some(Self(year))
    }
}

impl DoubleEndedIterator for Year {
    fn next_back(&mut self) -> Option<Self::Item> {
        let year = self.0 - 1;

        self.0 = year;

        Some(Self(year))
    }
}

impl Year {
    pub(super) fn dangerously_from_i32(year: i32) -> Self {
        Self(year)
    }
}

impl Display for Year {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_i32())
    }
}

impl PartialEq<Year> for Year {
    fn eq(&self, other: &Year) -> bool {
        self.as_i32() == other.as_i32()
    }
}

impl PartialEq<i32> for Year {
    fn eq(&self, other: &i32) -> bool {
        self.as_i32() == *other
    }
}

impl PartialOrd<Year> for Year {
    fn partial_cmp(&self, other: &Year) -> Option<std::cmp::Ordering> {
        self.as_i32().partial_cmp(&other.as_i32())
    }
}

impl PartialOrd<i32> for Year {
    fn partial_cmp(&self, other: &i32) -> Option<std::cmp::Ordering> {
        self.as_i32().partial_cmp(other)
    }
}

impl From<Year> for i32 {
    fn from(year: Year) -> Self {
        year.as_i32()
    }
}

#[derive(Debug, Clone, Copy, Eq, Hash, Ord, Serialize, Deserialize)]
pub struct Hour(u8);

impl Hour {
    pub fn from_u8(hour: u8) -> Result<Self, Error> {
        if hour > 23 {
            return Err(Error::new("Invalid hour provided", ErrorCode::Invalid));
        }

        Ok(Self(hour))
    }

    pub(super) fn dangerously_from_u8(hour: u8) -> Self {
        Self(hour)
    }

    pub fn as_u8(&self) -> u8 {
        self.0
    }

    pub fn as_str(&self) -> &str {
        self.0.to_string().as_str()
    }

    pub fn unix(&self) -> u32 {
        self.0 as u32 * SECONDS_PER_HOUR as u32
    }

    pub fn is_noon(&self) -> bool {
        self.0 == 12
    }

    pub fn is_midnight(&self) -> bool {
        self.0 == 0
    }

    pub fn is_morning(&self) -> bool {
        self.0 >= 6 && self.0 < 12
    }

    pub fn is_afternoon(&self) -> bool {
        self.0 >= 12 && self.0 < 18
    }

    pub fn is_evening(&self) -> bool {
        self.0 >= 18 && self.0 < 24
    }

    pub fn is_night(&self) -> bool {
        self.0 >= 0 && self.0 < 6
    }

    pub fn pretty_format(&self) -> u8 {
        let mut hour = self.0;
        if hour > 12 {
            return hour - 12;
        }
        hour
    }
}

impl PartialEq<Hour> for Hour {
    fn eq(&self, other: &Hour) -> bool {
        self.as_u8() == other.as_u8()
    }
}

impl PartialEq<u8> for Hour {
    fn eq(&self, other: &u8) -> bool {
        self.as_u8() == *other
    }
}

impl PartialOrd<Hour> for Hour {
    fn partial_cmp(&self, other: &Hour) -> Option<std::cmp::Ordering> {
        self.as_u8().partial_cmp(&other.as_u8())
    }
}

impl PartialOrd<u8> for Hour {
    fn partial_cmp(&self, other: &u8) -> Option<std::cmp::Ordering> {
        self.as_u8().partial_cmp(other)
    }
}

impl Display for Hour {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_u8())
    }
}

impl Iterator for Hour {
    type Item = Hour;

    fn next(&mut self) -> Option<Self::Item> {
        match self.0 {
            23 => {
                self.0 = 0;

                Some(Self(23))
            }
            _ => {
                let hour = self.0 + 1;

                self.0 = hour;

                Some(Self(hour))
            }
        }
    }
}

impl DoubleEndedIterator for Hour {
    fn next_back(&mut self) -> Option<Self::Item> {
        match self.0 {
            0 => {
                self.0 = 23;

                Some(Self(0))
            }
            _ => {
                let hour = self.0 - 1;

                self.0 = hour;

                Some(Self(hour))
            }
        }
    }
}

impl Add<u8> for Hour {
    type Output = Self;

    fn add(self, rhs: u8) -> Self::Output {
        let hour = self.0 + rhs;

        match hour {
            0..=23 => Self(hour),
            _ => Self(hour - 24),
        }
    }
}

impl AddAssign<u8> for Hour {
    fn add_assign(&mut self, rhs: u8) {
        self.0 += rhs;

        if self.0 > 23 {
            self.0 -= 24;
        }
    }
}

impl Sub<u8> for Hour {
    type Output = Self;

    fn sub(self, rhs: u8) -> Self::Output {
        let hour = self.0 - rhs;

        match hour {
            0..=23 => Self(hour),
            _ => Self(hour + 24),
        }
    }
}

impl SubAssign<u8> for Hour {
    fn sub_assign(&mut self, rhs: u8) {
        self.0 -= rhs;

        if self.0 > 23 {
            self.0 += 24;
        }
    }
}

impl Add<Hour> for Hour {
    type Output = Self;

    fn add(self, rhs: Hour) -> Self::Output {
        let hour = self.0 + rhs.0;

        match hour {
            0..=23 => Self(hour),
            _ => Self(hour - 24),
        }
    }
}

impl AddAssign<Hour> for Hour {
    fn add_assign(&mut self, rhs: Hour) {
        self.0 += rhs.0;

        if self.0 > 23 {
            self.0 -= 24;
        }
    }
}

impl Sub<Hour> for Hour {
    type Output = Self;

    fn sub(self, rhs: Hour) -> Self::Output {
        let hour = self.0 - rhs.0;

        match hour {
            0..=23 => Self(hour),
            _ => Self(hour + 24),
        }
    }
}

impl SubAssign<Hour> for Hour {
    fn sub_assign(&mut self, rhs: Hour) {
        self.0 -= rhs.0;

        if self.0 > 23 {
            self.0 += 24;
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, Hash, Ord, Serialize, Deserialize)]
pub struct Minute(u8);

impl Minute {
    pub fn from_u8(minute: u8) -> Result<Self, Error> {
        if minute > 59 {
            return Err(Error::new("Invalid minute provided", ErrorCode::Invalid));
        }

        Ok(Self(minute))
    }

    pub(super) fn dangerously_from_u8(minute: u8) -> Self {
        Self(minute)
    }

    pub fn as_u8(&self) -> u8 {
        self.0
    }

    pub fn as_str(&self) -> &str {
        self.0.to_string().as_str()
    }

    pub fn unix(&self) -> u32 {
        self.0 as u32 * SECONDS_PER_MINUTE as u32
    }

    pub fn is_top_of_hour(&self) -> bool {
        self.0 == 0
    }

    pub fn is_bottom_of_hour(&self) -> bool {
        self.0 == 59
    }

    pub fn is_half_hour(&self) -> bool {
        self.0 == 30
    }

    pub fn is_before_15_minutes(&self) -> bool {
        self.0 < 15
    }

    pub fn is_before_30_minutes(&self) -> bool {
        self.0 < 30
    }

    pub fn is_before_45_minutes(&self) -> bool {
        self.0 < 45
    }
}

impl PartialEq<Minute> for Minute {
    fn eq(&self, other: &Minute) -> bool {
        self.as_u8() == other.as_u8()
    }
}

impl PartialEq<u8> for Minute {
    fn eq(&self, other: &u8) -> bool {
        self.as_u8() == *other
    }
}

impl PartialOrd<Minute> for Minute {
    fn partial_cmp(&self, other: &Minute) -> Option<std::cmp::Ordering> {
        self.as_u8().partial_cmp(&other.as_u8())
    }
}

impl PartialOrd<u8> for Minute {
    fn partial_cmp(&self, other: &u8) -> Option<std::cmp::Ordering> {
        self.as_u8().partial_cmp(other)
    }
}

impl Display for Minute {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_u8())
    }
}

impl Iterator for Minute {
    type Item = Minute;

    fn next(&mut self) -> Option<Self::Item> {
        match self.0 {
            59 => {
                self.0 = 0;

                Some(Self(59))
            }
            _ => {
                let minute = self.0 + 1;

                self.0 = minute;

                Some(Self(minute))
            }
        }
    }
}

impl DoubleEndedIterator for Minute {
    fn next_back(&mut self) -> Option<Self::Item> {
        match self.0 {
            0 => {
                self.0 = 59;

                Some(Self(0))
            }
            _ => {
                let minute = self.0 - 1;

                self.0 = minute;

                Some(Self(minute))
            }
        }
    }
}

impl Add<u8> for Minute {
    type Output = Self;

    fn add(self, rhs: u8) -> Self::Output {
        let minute = self.0 + rhs;

        match minute {
            0..=59 => Self::dangerously_from_u8(minute),
            _ => Self::dangerously_from_u8(minute - 60),
        }
    }
}

impl AddAssign<u8> for Minute {
    fn add_assign(&mut self, rhs: u8) {
        self.0 += rhs;

        if self.0 > 59 {
            self.0 -= 60;
        }
    }
}

impl Sub<u8> for Minute {
    type Output = Self;

    fn sub(self, rhs: u8) -> Self::Output {
        let minute = self.0 - rhs;

        match minute {
            0..=59 => Self::dangerously_from_u8(minute),
            _ => Self::dangerously_from_u8(minute + 60),
        }
    }
}

impl SubAssign<u8> for Minute {
    fn sub_assign(&mut self, rhs: u8) {
        self.0 -= rhs;

        if self.0 > 59 {
            self.0 += 60;
        }
    }
}

impl Add<Minute> for Minute {
    type Output = Self;

    fn add(self, rhs: Minute) -> Self::Output {
        let minute = self.0 + rhs.as_u8();

        match minute {
            0..=59 => Self::dangerously_from_u8(minute),
            _ => Self::dangerously_from_u8(minute - 60),
        }
    }
}

impl AddAssign<Minute> for Minute {
    fn add_assign(&mut self, rhs: Minute) {
        self.0 += rhs.as_u8();

        if self.0 > 59 {
            self.0 -= 60;
        }
    }
}

impl Sub<Minute> for Minute {
    type Output = Self;

    fn sub(self, rhs: Minute) -> Self::Output {
        let minute = self.0 - rhs.as_u8();

        match minute {
            0..=59 => Self::dangerously_from_u8(minute),
            _ => Self::dangerously_from_u8(minute + 60),
        }
    }
}

impl SubAssign<Minute> for Minute {
    fn sub_assign(&mut self, rhs: Minute) {
        self.0 -= rhs.as_u8();

        if self.0 > 59 {
            self.0 += 60;
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, Hash, Ord, Serialize, Deserialize)]
pub struct Second(u8);

impl Second {
    pub fn from_u8(second: u8) -> Result<Self, Error> {
        if second > 59 {
            return Err(Error::new("Invalid second provided", ErrorCode::Invalid));
        }

        Ok(Self(second))
    }

    pub(super) fn dangerously_from_u8(second: u8) -> Self {
        Self(second)
    }

    pub fn as_u8(&self) -> u8 {
        self.0
    }

    pub fn as_str(&self) -> &str {
        self.0.to_string().as_str()
    }

    pub fn unix(&self) -> u32 {
        self.0 as u32
    }

    pub fn is_top_of_minute(&self) -> bool {
        self.0 == 0
    }

    pub fn is_bottom_of_minute(&self) -> bool {
        self.0 == 59
    }

    pub fn is_before_15_seconds(&self) -> bool {
        self.0 < 15
    }

    pub fn is_before_30_seconds(&self) -> bool {
        self.0 < 30
    }

    pub fn is_before_45_seconds(&self) -> bool {
        self.0 < 45
    }
}

impl PartialEq<Second> for Second {
    fn eq(&self, other: &Second) -> bool {
        self.as_u8() == other.as_u8()
    }
}

impl PartialEq<u8> for Second {
    fn eq(&self, other: &u8) -> bool {
        self.as_u8() == *other
    }
}

impl PartialOrd<Second> for Second {
    fn partial_cmp(&self, other: &Second) -> Option<std::cmp::Ordering> {
        self.as_u8().partial_cmp(&other.as_u8())
    }
}

impl PartialOrd<u8> for Second {
    fn partial_cmp(&self, other: &u8) -> Option<std::cmp::Ordering> {
        self.as_u8().partial_cmp(other)
    }
}

impl Display for Second {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_u8())
    }
}

impl Iterator for Second {
    type Item = Second;

    fn next(&mut self) -> Option<Self::Item> {
        match self.0 {
            59 => {
                self.0 = 0;

                Some(Self(59))
            }
            _ => {
                let second = self.0 + 1;

                self.0 = second;

                Some(Self(second))
            }
        }
    }
}

impl DoubleEndedIterator for Second {
    fn next_back(&mut self) -> Option<Self::Item> {
        match self.0 {
            0 => {
                self.0 = 59;

                Some(Self(0))
            }
            _ => {
                let second = self.0 - 1;

                self.0 = second;

                Some(Self(second))
            }
        }
    }
}

impl Add<u8> for Second {
    type Output = Second;

    fn add(self, rhs: u8) -> Self::Output {
        let second = self.0 + rhs;

        if second > 59 {
            Self(second - 60)
        } else {
            Self(second)
        }
    }
}

impl AddAssign<u8> for Second {
    fn add_assign(&mut self, rhs: u8) {
        self.0 += rhs;

        if self.0 > 59 {
            self.0 -= 60;
        }
    }
}

impl Sub<u8> for Second {
    type Output = Second;

    fn sub(self, rhs: u8) -> Self::Output {
        if self.0 < rhs {
            Self(60 - rhs + self.0)
        } else {
            Self(self.0 - rhs)
        }
    }
}

impl SubAssign<u8> for Second {
    fn sub_assign(&mut self, rhs: u8) {
        self.0 -= rhs;

        if self.0 > 59 {
            self.0 += 60;
        }
    }
}

impl Add<Second> for Second {
    type Output = Second;

    fn add(self, rhs: Second) -> Self::Output {
        let second = self.0 + rhs.0;

        if second > 59 {
            Self(second - 60)
        } else {
            Self(second)
        }
    }
}

impl AddAssign<Second> for Second {
    fn add_assign(&mut self, rhs: Second) {
        self.0 += rhs.0;

        if self.0 > 59 {
            self.0 -= 60;
        }
    }
}

impl Sub<Second> for Second {
    type Output = Second;

    fn sub(self, rhs: Second) -> Self::Output {
        if self.0 < rhs.0 {
            Self(60 - rhs.0 + self.0)
        } else {
            Self(self.0 - rhs.0)
        }
    }
}

impl SubAssign<Second> for Second {
    fn sub_assign(&mut self, rhs: Second) {
        if self.0 < rhs.0 {
            self.0 = 60 - rhs.0 + self.0;
        } else {
            self.0 -= rhs.0;
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, Hash, Ord, Serialize, Deserialize)]
pub struct Millisecond(u16);

impl Millisecond {
    pub fn from_u16(millisecond: u16) -> Result<Self, Error> {
        if millisecond > 999 {
            return Err(Error::new(
                "Invalid millisecond provided",
                ErrorCode::Invalid,
            ));
        }

        Ok(Self(millisecond))
    }

    pub(super) fn dangerously_from_u16(millisecond: u16) -> Self {
        Self(millisecond)
    }

    pub fn as_u16(&self) -> u16 {
        self.0
    }

    pub fn as_str(&self) -> &str {
        self.0.to_string().as_str()
    }

    pub fn unix(&self) -> u32 {
        self.0 as u32
    }

    pub fn is_top_of_second(&self) -> bool {
        self.0 == 0
    }

    pub fn is_bottom_of_second(&self) -> bool {
        self.0 == 999
    }

    pub fn is_before_250_milliseconds(&self) -> bool {
        self.0 < 250
    }

    pub fn is_before_500_milliseconds(&self) -> bool {
        self.0 < 500
    }

    pub fn is_before_750_milliseconds(&self) -> bool {
        self.0 < 750
    }
}

impl PartialEq<Millisecond> for Millisecond {
    fn eq(&self, other: &Millisecond) -> bool {
        self.as_u16() == other.as_u16()
    }
}

impl PartialEq<u16> for Millisecond {
    fn eq(&self, other: &u16) -> bool {
        self.as_u16() == *other
    }
}

impl PartialOrd<Millisecond> for Millisecond {
    fn partial_cmp(&self, other: &Millisecond) -> Option<std::cmp::Ordering> {
        self.as_u16().partial_cmp(&other.as_u16())
    }
}

impl PartialOrd<u16> for Millisecond {
    fn partial_cmp(&self, other: &u16) -> Option<std::cmp::Ordering> {
        self.as_u16().partial_cmp(other)
    }
}

impl Display for Millisecond {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_u16())
    }
}

impl Iterator for Millisecond {
    type Item = Millisecond;

    fn next(&mut self) -> Option<Self::Item> {
        match self.0 {
            999 => {
                self.0 = 0;

                Some(Self(999))
            }
            _ => {
                let millisecond = self.0 + 1;

                self.0 = millisecond;

                Some(Self(millisecond))
            }
        }
    }
}

impl DoubleEndedIterator for Millisecond {
    fn next_back(&mut self) -> Option<Self::Item> {
        match self.0 {
            0 => {
                self.0 = 999;

                Some(Self(0))
            }
            _ => {
                let millisecond = self.0 - 1;

                self.0 = millisecond;

                Some(Self(millisecond))
            }
        }
    }
}

impl Add<u16> for Millisecond {
    type Output = Millisecond;

    fn add(self, rhs: u16) -> Self::Output {
        let millisecond = self.0 + rhs;

        if millisecond > 999 {
            Self(millisecond - 1000)
        } else {
            Self(millisecond)
        }
    }
}

impl AddAssign<u16> for Millisecond {
    fn add_assign(&mut self, rhs: u16) {
        self.0 += rhs;

        if self.0 > 999 {
            self.0 -= 1000;
        }
    }
}

impl Sub<u16> for Millisecond {
    type Output = Millisecond;

    fn sub(self, rhs: u16) -> Self::Output {
        let millisecond = self.0 - rhs;

        if millisecond < 0 {
            Self(millisecond + 1000)
        } else {
            Self(millisecond)
        }
    }
}

impl SubAssign<u16> for Millisecond {
    fn sub_assign(&mut self, rhs: u16) {
        self.0 -= rhs;

        if self.0 < 0 {
            self.0 += 1000;
        }
    }
}

impl Add<Millisecond> for Millisecond {
    type Output = Millisecond;

    fn add(self, rhs: Millisecond) -> Self::Output {
        let millisecond = self.0 + rhs.0;

        if millisecond > 999 {
            Self(millisecond - 1000)
        } else {
            Self(millisecond)
        }
    }
}

impl AddAssign<Millisecond> for Millisecond {
    fn add_assign(&mut self, rhs: Millisecond) {
        self.0 += rhs.0;

        if self.0 > 999 {
            self.0 -= 1000;
        }
    }
}

impl Sub<Millisecond> for Millisecond {
    type Output = Millisecond;

    fn sub(self, rhs: Millisecond) -> Self::Output {
        let millisecond = self.0 - rhs.0;

        if millisecond < 0 {
            Self(millisecond + 1000)
        } else {
            Self(millisecond)
        }
    }
}

impl SubAssign<Millisecond> for Millisecond {
    fn sub_assign(&mut self, rhs: Millisecond) {
        self.0 -= rhs.0;

        if self.0 < 0 {
            self.0 += 1000;
        }
    }
}
