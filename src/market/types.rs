use chrono::prelude::*;
use std::marker::PhantomData;
use rkyv::{Archive, Deserialize, Serialize};

#[derive(Archive, Deserialize, Serialize, Debug, PartialEq)]
pub struct PriceCalc(pub f32);
#[derive(Archive, Deserialize, Serialize, Debug, PartialEq)]
pub struct PriceStore(pub f32);
#[derive(Archive, Deserialize, Serialize, Debug, PartialEq)]
pub struct PriceExt(pub f32);

pub trait Style {}
pub struct Call;
pub struct Put;
impl Style for Call {}
impl Style for Put {}

pub trait Side {}
pub struct Long;
pub struct Short;
impl Side for Long {}
impl Side for Short {}

#[derive(Archive, Deserialize, Serialize, Debug, PartialEq)]
pub struct Opt<S:Style> {
    pub _style: PhantomData<*const S>,
    pub expir: NaiveDate,
    pub strike: PriceCalc,
}
impl<S:Style> Opt<S> {
    pub fn new(expir: NaiveDate, strike: PriceCalc) -> Self {
        Opt {
            _style: PhantomData,
            expir: expir,
            strike: strike,
        }
    }
}
#[derive(Archive, Deserialize, Serialize, Debug, PartialEq)]
pub struct Quote {
    pub bid: PriceCalc,
    pub ask: PriceCalc,
}