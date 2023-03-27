// use std::ops::Deref;
use std::fmt;
use chrono::prelude::{NaiveDate};
use rkyv::{Archive, Deserialize, Serialize};
use std::mem::transmute;

type NaiveDateInternal = i32;

pub type Pred1<T> = fn(T) -> bool;

#[derive(Archive, Deserialize, Serialize, Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
#[archive_attr(derive(Eq, PartialEq, Ord, PartialOrd))]
pub struct Timestamp(pub i64);

impl fmt::Display for Timestamp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let millis = chrono::naive::NaiveDateTime::from_timestamp_millis(self.0).unwrap();
        write!(f, "{}", millis)
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

#[derive(Archive, Deserialize, Serialize, Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
#[archive_attr(derive(Eq, PartialEq, Ord, PartialOrd))]
pub struct ExpirDate(pub i32);
impl ExpirDate {
    pub fn from_millis(millis:i64) -> Self {
        Self::from_naive(chrono::naive::NaiveDateTime::from_timestamp_millis(millis).unwrap().date())
    }

    pub fn from_naive(d:NaiveDate) -> Self {
        ExpirDate(unsafe { transmute::<NaiveDate,NaiveDateInternal>(d) })
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

// impl Deref for ExpirDate {
//     type Target = NaiveDate;
//     fn deref(&self) -> &Self::Target {
//         unsafe { &transmute::<&NaiveDateInternal,&NaiveDate>(&self.0) }
//     }
// }

// impl<T> AsRef<T> for ExpirDate
// where
//     T: ?Sized,
//     <ExpirDate as Deref>::Target: AsRef<T>,
// {
//     fn as_ref(&self) -> &T {
//         self.deref().as_ref()
//     }
// }

pub static TS_ZERO: Timestamp = Timestamp(0); // NaiveDateTime::MIN;
