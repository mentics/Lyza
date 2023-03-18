use chrono::prelude::*;
use std::marker::PhantomData;
use rkyv::{Archive, Deserialize, Serialize};
use serde;

use derive_more::{Display, From};

#[derive(Display, From, Copy, Clone, Archive, Deserialize, Serialize, Debug, PartialEq, serde::Deserialize)]
pub struct PriceCalc(pub f32);

// #[derive(Archive, Deserialize, Serialize, Debug, PartialEq)]
pub struct PriceStore(pub f32);

// #[derive(Archive, Deserialize, Serialize, Debug, PartialEq)]
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
    pub last: PriceCalc,
    pub size_bid: u32,
    pub size_ask: u32,
}

#[derive(Archive, Deserialize, Serialize, Debug, PartialEq)]
pub struct Meta {
    pub delta: f32,
    pub gamma: f32,
    pub vega: f32,
    pub theta: f32,
    pub rho: f32,
    pub iv: f32,
    pub volume: f32,
}

#[derive(Archive, Deserialize, Serialize, Debug, PartialEq)]
pub struct OptQuote<S:Style> {
    pub opt: Opt<S>,
    pub meta: Meta,
    pub quote: Quote,
}
