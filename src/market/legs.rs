use lazy_static::lazy_static;
use regex::Regex;
use std::marker::PhantomData;

use crate::general::*;
use crate::market::types::*;

pub struct Leg<D:Side,S:Style> {
    pub _side: PhantomData<*const D>,
    pub opt: Opt<S>,
    pub quantity: QuantityType,
}

impl<D:Side,S:Style> Leg<D,S> {
    pub fn new(opt:Opt<S>, qty:f32) -> Self {
        Leg {
            _side: PhantomData,
            opt: opt,
            quantity: qty,
        }
    }
}

// 1lp100.5@2023-03-22

impl<D:Side,S:Style> std::fmt::Display for Leg<D,S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.quantity != 1.0 {
            write!(f, "{}", self.quantity)?;
        }
        write!(f, "{}", D::code())?;
        write!(f, "{}", self.opt)
    }
}
impl<D:Side,S:Style> std::fmt::Debug for Leg<D,S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

lazy_static! {
    static ref RE_OPT: Regex = Regex::new(r"([0-9.]+)?([ls])([cp])([0-9.]+)@?(\d\d|\d{4}-\d\d-\d\d)?").unwrap();
}

type EnumStyleLeg<D> = StyleEnum<Leg<D,Call>,Leg<D,Put>>;
type EnumSideLeg = SideEnum<EnumStyleLeg<Long>, EnumStyleLeg<Short>>;

pub fn parse_leg(s: &str, xpirs:Option<Vec<ExpirDate>>) -> Result<EnumSideLeg, anyhow::Error> {
    let captures = RE_OPT.captures(s).unwrap_or_else(|| panic!("Could not parse Opt: {s}"));
    let qty:QuantityType = captures.get(1).map_or(Ok(1.0), |x| x.as_str().parse())?;
    let side_code = captures[2].chars().next().unwrap();
    let style_code = captures[3].chars().next().unwrap();
    let strike:StrikeType = captures[4].parse().unwrap();
    let str_xpir = captures.get(5).map_or("1", |x| x.as_str());
    let xpir = if str_xpir.len() <= 2 {
        match xpirs {
            Some(xs) => {
                let xpr:usize = str_xpir.parse().unwrap();
                xs[xpr]
            },
            None => return Err(anyhow::anyhow!("Xpir num but no xpirs {}", str_xpir)),
        }
    } else {
        ExpirDate::parse(str_xpir).unwrap()
    };

    let opt = make_opt(style_code, xpir, strike);

    return Ok(make_side(side_code, opt, qty));
}

pub fn make_side(side_code:char, opt:EnumStyleOpt, qty:QuantityType) -> EnumSideLeg {
    if side_code == Long::code() {
        return SideEnum::Long(make_style_leg(opt, qty));
    } else {
        return SideEnum::Short(make_style_leg(opt, qty));
    }
}

fn make_style_leg<D:Side>(opt:EnumStyleOpt, qty:QuantityType) -> EnumStyleLeg<D> {
    match opt {
        StyleEnum::Call(x) => StyleEnum::Call(Leg::new(x, qty)),
        StyleEnum::Put(x) => StyleEnum::Put(Leg::new(x, qty)),
    }
}