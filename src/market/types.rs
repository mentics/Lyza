use lazy_static::lazy_static;
use regex::Regex;
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

pub type Tennies = u32; // Tenths of pennies
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

enum_type! {
    Side {
        fn code() -> char;
    }
    Long {
        fn code() -> char { 'l' }
    }
    Short {
        fn code() -> char { 's' }
    }
}

pub struct Opt<S:Style> {
    pub _style: PhantomData<*const S>,
    pub expir: ExpirDate,
    pub strike: StrikeType,
}
impl<S:Style> Opt<S> {
    pub fn new(expir: ExpirDate, strike: StrikeType) -> Self {
        Opt {
            _style: PhantomData,
            expir: expir,
            strike: strike,
        }
    }
}

// p100.5@2023-03-22
impl<S:Style> std::fmt::Display for Opt<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}@{}", S::code(), self.strike, self.expir)
    }
}
impl<S:Style> std::fmt::Debug for Opt<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

lazy_static! {
    static ref RE_OPT: Regex = Regex::new(r"([cp])([0-9.]+)@?(\d\d|\d{4}-\d\d-\d\d)?").unwrap();
}

pub fn parse_opt(s: &str, xpirs:Option<Vec<ExpirDate>>) -> Result<StyleEnum<Opt<Call>,Opt<Put>>, anyhow::Error> {
    let captures = RE_OPT.captures(s).unwrap_or_else(|| panic!("Could not parse Opt: {s}"));
    let style_code = captures[1].chars().next().unwrap();
    println!("caps: {:?}", captures);
    let strike:StrikeType = to_strike(captures[2].parse()?);
    println!("strike: {strike}");
    let str_xpir = captures.get(3).map_or("1", |x| x.as_str());
    println!("Checking {str_xpir}");
    let xpir = if str_xpir.len() <= 2 {
        println!("Matching {str_xpir}");
        match xpirs {
            Some(xs) => {
                let xpr:usize = str_xpir.parse().unwrap();
                xs[xpr-1]
            },
            None => return Err(anyhow::anyhow!("Xpir num but no xpirs {}", str_xpir)),
        }
    } else {
        println!("Parsing {str_xpir}");
        ExpirDate::parse(str_xpir).unwrap()
    };

    return Ok(make_opt(style_code, xpir, strike));
}

pub type EnumStyleOpt = StyleEnum<Opt<Call>,Opt<Put>>;

pub fn make_opt(style_code:char, xpir:ExpirDate, strike:StrikeType) -> EnumStyleOpt {
    if style_code == Call::code() {
        return StyleEnum::Call(Opt::new(xpir, strike))
    } else {
        return StyleEnum::Put(Opt::new(xpir, strike))
    }
}

impl<C:Context,S:Style> Writable<C> for Opt<S> {
    #[inline]
    fn write_to<W:?Sized+Writer<C>>(&self, writer:&mut W) -> Result<(),C::Error> {
        // S::code().write_to(writer)?;
        self.expir.write_to(writer)?;
        self.strike.write_to(writer)?;
        return Ok(());
    }

    #[inline]
    fn bytes_needed(&self) -> Result<usize, C::Error> {
        Ok(4 + 4) // Ok(ExpirDate::minimum_bytes_needed() + PriceCalc::minimum_bytes_needed())
    }
}

// impl<C:Context> Writable<C> for Opt<Call> {
//     #[inline]
//     fn write_to<W:?Sized+Writer<C>>(&self, writer:&mut W) -> Result<(),C::Error> {
//         // Call::code().write_to(writer)?;
//         self.expir.write_to(writer)?;
//         self.strike.write_to(writer)?;
//         return Ok(());
//     }

//     #[inline]
//     fn bytes_needed(&self) -> Result<usize, C::Error> {
//         Ok(ExpirDate::minimum_bytes_needed() + PriceCalc::minimum_bytes_needed())
//     }
// }

// impl<C:Context> Writable<C> for Opt<Put> {
//     #[inline]
//     fn write_to<W:?Sized+Writer<C>>(&self, writer:&mut W) -> Result<(),C::Error> {
//         // Put::code().write_to(writer)?;
//         self.expir.write_to(writer)?;
//         self.strike.write_to(writer)?;
//         return Ok(());
//     }

//     #[inline]
//     fn bytes_needed(&self) -> Result<usize, C::Error> {
//         Ok(ExpirDate::minimum_bytes_needed() + PriceCalc::minimum_bytes_needed())
//     }
// }

impl<'a,C:Context,S:Style> Readable<'a,C> for Opt<S> {
    #[inline]
    fn read_from<R:Reader<'a,C>>(reader:&mut R) -> Result<Self,C::Error> {
        // let code = reader.read_value()?;
        let xpir = reader.read_value()?;
        let strike = reader.read_value()?;
        Ok(Opt::new(xpir, strike))
    }

    #[inline]
    fn minimum_bytes_needed() -> usize {
        4 + 4 // ExpirDate::minimum_bytes_needed() + PriceCalc::minimum_bytes_needed()
    }
}

// impl<'a,C:Context> Readable<'a,C> for Opt<Call> {
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
//         4 + 4
//     }
// }

// impl<'a,C:Context> Readable<'a,C> for Opt<Put> {
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
//         4 + 4
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
}

#[derive(Copy, Clone, Readable, Writable, Debug)]
pub struct QuoteOption {
    pub quote: Quote,
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
    pub quote: QuoteOption,
}
