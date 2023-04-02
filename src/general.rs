// use std::ops::Deref;
use std::fmt;
use chrono::{prelude::{NaiveDate}, NaiveTime, NaiveDateTime};
use speedy::{Readable, Writable};
use std::mem::transmute;

type NaiveDateInternal = i32;

pub type Pred1<T> = fn(T) -> bool;

pub type QuantityType = f32;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Readable, Writable)]
pub struct Timestamp(pub i64);

impl Timestamp {
    pub fn from_naivedate(nd:NaiveDate) -> Self {
        Self(NaiveDateTime::new(nd, NaiveTime::default()).timestamp_millis())
    }
    pub fn from_ymd(year: i32, month: u32, day: u32) -> Self {
        Self(NaiveDateTime::new(NaiveDate::from_ymd_opt(year, month, day).unwrap(), NaiveTime::default()).timestamp_millis())
    }
}

impl fmt::Display for Timestamp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ndt = NaiveDateTime::from_timestamp_millis(self.0)
                .expect(format!("Could not get ndt for {}", self.0).as_str()); // TODO: optimize
        write!(f, "{}", ndt)
    }
}
impl fmt::Debug for Timestamp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}


// impl Deref for Timestamp {
//     type Target = NaiveDateTime;
//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Readable, Writable)]
pub struct ExpirDate(pub i32);
impl ExpirDate {
    pub fn from_millis(millis:i64) -> Self {
        Self::from_naive(NaiveDateTime::from_timestamp_millis(millis).unwrap().date())
    }

    pub fn from_naive(d:NaiveDate) -> Self {
        ExpirDate(unsafe { transmute::<NaiveDate,NaiveDateInternal>(d) })
    }

    pub fn parse(s:&str) -> Result<Self,chrono::ParseError> {
        let nd = NaiveDate::parse_from_str(s, "%Y-%m-%d")?;
        return Ok(Self::from_naive(nd));
    }

    pub fn to_naive(&self) -> &NaiveDate {
        unsafe { &transmute::<&NaiveDateInternal,&NaiveDate>(&self.0) }
    }
}
impl fmt::Display for ExpirDate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_naive())
    }
}
impl fmt::Debug for ExpirDate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

// impl<'a, C:Context> Readable<'a,C> for ExpirDate {
//     #[inline]
//     fn read_from< R:Reader<'a, C>>(reader:&mut R) -> Result<Self, C::Error> {
//         Ok(ExpirDate(reader.read_value()?))
//     }

//     #[inline]
//     fn minimum_bytes_needed() -> usize {
//         4
//     }
// }

pub static TS_ZERO: Timestamp = Timestamp(0); // NaiveDateTime::MIN;
