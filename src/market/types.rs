use std::marker::PhantomData;
use speedy::{Readable, Writable, Context, Writer, Reader};
use crate::general::*;
use crate::macros::enum_type;

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

enum_type! {
    Style:Switch {
        fn code() -> char;
    }
    Call {
        fn code() -> char { 'c' }
    }
    Put {
        fn code() -> char { 'p' }
    }
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
#[derive(Readable,Writable)]
pub struct Long;
#[derive(Readable,Writable)]
pub struct Short;
impl Side for Long {}
impl Side for Short {}

// #[derive(Readable, Writable)]
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

impl<C:Context> Writable<C> for Opt<Call> {
    #[inline]
    fn write_to<W:?Sized+Writer<C>>(&self, writer:&mut W) -> Result<(),C::Error> {
        // Call::code().write_to(writer)?;
        self.expir.write_to(writer)?;
        self.strike.write_to(writer)?;
        return Ok(());
    }

    #[inline]
    fn bytes_needed(&self) -> Result<usize, C::Error> {
        // TODO
        Ok(1 + 4 + 4)
        // Ok(1 + ExpirDate::minimum_bytes_needed() + PriceCalc::minimum_bytes_needed())
    }
}

impl<C:Context> Writable<C> for Opt<Put> {
    #[inline]
    fn write_to<W:?Sized+Writer<C>>(&self, writer:&mut W) -> Result<(),C::Error> {
        // Put::code().write_to(writer)?;
        self.expir.write_to(writer)?;
        self.strike.write_to(writer)?;
        return Ok(());
    }

    #[inline]
    fn bytes_needed(&self) -> Result<usize, C::Error> {
        // TODO
        Ok(4 + 4)
        // Ok(1 + ExpirDate::minimum_bytes_needed() + PriceCalc::minimum_bytes_needed())
    }
}

// TODO
// impl<C:Context, S:Style+Writable<C>> Writable<C> for Opt<S> {
//     #[inline]
//     fn write_to<W:?Sized+Writer<C>>(&self, writer:&mut W) -> Result<(),C::Error> {
//         S::code().write_to(writer);
//         self.expir.write_to(writer)?;
//         self.strike.write_to(writer)?;
//         return Ok(());
//     }

//     #[inline]
//     fn bytes_needed(&self) -> Result<usize, C::Error> {
//         // TODO
//         Ok(1 + 4 + 4)
//         // Ok(1 + ExpirDate::minimum_bytes_needed() + PriceCalc::minimum_bytes_needed())
//     }
// }

impl<'a,C:Context> Readable<'a,C> for Opt<Call> {
    #[inline]
    fn read_from<R:Reader<'a,C>>(reader:&mut R) -> Result<Self,C::Error> {
        // let code = reader.read_value()?;
        let xpir = reader.read_value()?;
        let strike = reader.read_value()?;
        Ok(Opt::new(xpir, strike))
    }

    #[inline]
    fn minimum_bytes_needed() -> usize {
        // TODO
        4 + 4
    }
}

impl<'a,C:Context> Readable<'a,C> for Opt<Put> {
    #[inline]
    fn read_from<R:Reader<'a,C>>(reader:&mut R) -> Result<Self,C::Error> {
        // let code = reader.read_value()?;
        let xpir = reader.read_value()?;
        let strike = reader.read_value()?;
        Ok(Opt::new(xpir, strike))
    }

    #[inline]
    fn minimum_bytes_needed() -> usize {
        // TODO
        4 + 4
    }
}

// impl<'a,C:Context, S:Style+Readable<'a,C>> Readable<'a,C> for Opt<S> {
//     #[inline]
//     fn read_from<R:Reader<'a,C>>(reader:&mut R) -> Result<Self,C::Error> {
//         // let code = reader.read_value()?;
//         let xpir = reader.read_value()?;
//         let strike = reader.read_value()?;
//         Ok(Opt::new(xpir, strike))
//     }

//     #[inline]
//     fn minimum_bytes_needed() -> usize {
//         // TODO
//         1 + 4 + 4
//     }
// }

#[derive(Copy, Clone, Readable, Writable, Debug)]
pub struct BidAsk {
    pub bid: PriceCalc,
    pub ask: PriceCalc,
}

#[derive(Copy, Clone, Readable, Writable, Debug)]
pub struct Quote {
    pub bisk: BidAsk,
    pub last: PriceCalc, // Missing as NaN
    pub size_bid: u32,
    pub size_ask: u32,
    pub meta: Meta,
}

#[derive(Copy, Clone, Readable, Writable, Debug)]
pub struct Meta {
    pub delta: f32,
    pub gamma: f32,
    pub vega: f32,
    pub theta: f32,
    pub rho: f32,
    pub iv: f32, // Missing as NaN
    pub volume: f32,
}

#[derive(Readable, Writable, Debug)]
pub struct OptQuote<S:Style> {
    pub opt: Opt<S>,
    // pub meta: Meta,
    pub quote: Quote,
}
