use std::collections::{HashMap, BTreeMap};
use crate::general::*;
use crate::market::types::*;

trait Chall<'a> {
    type R: 'a;
    fn at(&'a self, ts:Timestamp) -> Option<Self::R>; //&'a dyn Chat<'a>;
}

trait Chat<'a> {
    type R: 'a;
    fn expir(&'a self, xpir:ExpirDate) -> Option<Self::R>; //&'a dyn Chex<'a>;
}

trait Chex<'a> {
    fn quoteCall(&'a self, strike:StrikeType) -> Option<&'a Quote>;
    fn quotePut(&'a self, strike:StrikeType) -> Option<&'a Quote>;
}

pub struct ChainsAll {
    chats: HashMap<Timestamp,ChainAt>,
}
impl ChainsAll {
    pub fn new(calls: Vec<(Timestamp,OptQuote<Call>)>, puts: Vec<(Timestamp,OptQuote<Put>)>) -> ChainsAll {
        let chats: HashMap<Timestamp,ChainAt> = HashMap::new();
        return ChainsAll { chats: chats };
    }
}
impl<'a> Chall<'a> for ChainsAll {
    type R = &'a ChainAt;
    fn at(&'a self, ts:Timestamp) -> Option<Self::R> {
        self.chats.get(&ts)
    }
}

pub struct ChainAt {
    chexs: HashMap<ExpirDate,ChainStyles>,
}
impl<'a> Chat<'a> for ChainAt {
    type R = &'a ChainStyles;
    fn expir(&'a self, xpir:ExpirDate) -> Option<Self::R> {
        self.chexs.get(&xpir)
    }
}

pub struct ChainStyles {
    calls: BTreeMap<StrikeType,Quote>,
    puts: BTreeMap<StrikeType,Quote>,
}
impl<'a> Chex<'a> for ChainStyles {
    fn quoteCall(&'a self, strike:StrikeType) -> Option<&'a Quote> {
        self.calls.get(&strike)
    }

    fn quotePut(&'a self, strike:StrikeType) -> Option<&'a Quote> {
        self.puts.get(&strike)
    }
}

// let quote = chall.at(ts).expir(xpir).quote(opt);

