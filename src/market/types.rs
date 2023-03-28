use std::marker::PhantomData;
use rkyv::{Archive, Deserialize, Serialize};
// use serde;
use derive_more::Display;
use crate::general::*;

// #[derive(Display, From, Copy, Clone, Ord, Archive, Deserialize, Serialize, Debug, PartialEq, serde::Deserialize)]
// #[derive(num_derive::Float, derive_more::Neg, num_derive::FromPrimitive, num_derive::ToPrimitive, num_derive::NumCast)]
// pub struct PriceCalc(pub f32);
pub type PriceCalc = f32;

// Encodes missing as NaN
// #[derive(Display, From, Copy, Clone, Archive, Deserialize, Serialize, Debug, PartialEq, serde::Deserialize)]
// pub struct Missing<T>(pub T);

// #[derive(Archive, Deserialize, Serialize, Debug, PartialEq)]
// pub struct PriceStore(pub f32);
pub type PriceStore = f32;

// #[derive(Archive, Deserialize, Serialize, Debug, PartialEq)]
// pub struct PriceExt(pub f32);
pub type PriceExt = f32;

pub type Tennies = i32; // Tenths of pennies
pub type StrikeType = Tennies;
// pub struct StrikeType(Tennies);

pub fn to_strike(p:PriceCalc) -> StrikeType {
    (p * 1000.0) as StrikeType
}
pub fn from_strike(s:StrikeType) -> PriceCalc {
    (s as PriceCalc) / 1000.0
}

pub trait Style: Switch {
    fn code() -> &'static str;
}
// #[derive(Display)]
pub struct Call;
#[derive(Display,Debug)]
pub struct Put;
impl Style for Call {
    fn code() -> &'static str { return "Call" }
}
impl Style for Put {
    fn code() -> &'static str { return "Put" }
}

pub trait Switch {
    fn switch<T>(a:T, b:T) -> T;
}
impl Switch for Call {
    fn switch<T>(a:T, _:T) -> T {
        return a;
    }
}
impl Switch for Put {
    fn switch<T>(_:T, b:T) -> T {
        return b;
    }
}

pub trait Side {}
#[derive(Display,Debug)]
pub struct Long;
#[derive(Display,Debug)]
pub struct Short;
impl Side for Long {}
impl Side for Short {}

impl std::fmt::Debug for Call {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Call")
    }
}
impl std::fmt::Display for Call {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Call")
    }
}

#[derive(Archive, Deserialize, Serialize, PartialEq)]
pub struct Opt<S:Style> {
    pub _style: PhantomData<*const S>,
    pub expir: ExpirDate,
    pub strike: PriceCalc,
}
impl<S:Style> Opt<S> {
    pub fn new(expir: ExpirDate, strike: PriceCalc) -> Self {
        Opt {
            _style: PhantomData,
            expir: expir,
            strike: strike,
        }
    }
}
impl<S:Style> std::fmt::Display for Opt<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", S::code(), self.expir, self.strike)
    }
}
impl<S:Style> std::fmt::Debug for Opt<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Copy, Clone, Archive, Deserialize, Serialize, Debug)]
pub struct BidAsk {
    pub bid: PriceCalc,
    pub ask: PriceCalc,
}

#[derive(Copy, Clone, Archive, Deserialize, Serialize, Debug)]
pub struct Quote {
    pub bisk: BidAsk,
    pub last: PriceCalc, // Missing as NaN
    pub size_bid: u32,
    pub size_ask: u32,
    pub meta: Meta,
}

#[derive(Copy, Clone, Archive, Deserialize, Serialize, Debug)]
pub struct Meta {
    pub delta: f32,
    pub gamma: f32,
    pub vega: f32,
    pub theta: f32,
    pub rho: f32,
    pub iv: f32, // Missing as NaN
    pub volume: f32,
}

#[derive(Archive, Deserialize, Serialize, Debug)]
pub struct OptQuote<S:Style> {
    pub opt: Opt<S>,
    // pub meta: Meta,
    pub quote: Quote,
}
