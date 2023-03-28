use std::collections::BTreeMap;
use rkyv::{Archive, Deserialize, Serialize};

use crate::general::*;
use crate::market::types::*;

pub trait Chall {
    type R:Chat;
    fn at(&self, ts:&Timestamp) -> Option<&Self::R>;
    fn run_all(&self, f:fn(&Timestamp, &Self::R) -> ()) {
        self.run(|_| true, f)
    }
    fn run(&self, pred:fn(&Timestamp) -> bool, f:fn(&Timestamp, &Self::R) -> ());
}

pub trait Chat {
    type R:Chex;
    fn expir(&self, xpir:&ExpirDate) -> Option<&Self::R>;
}

pub trait Chex {
    fn quote<S:Style>(&self, strike:&PriceCalc) -> Option<&Quote>;
}

#[derive(Archive, Deserialize, Serialize)]
pub struct ChainsAll {
    pub chats: BTreeMap<Timestamp,ChainAt>,
}
impl ChainsAll {
    pub fn new(calls: &Vec<(Timestamp,OptQuote<Call>)>, puts: &Vec<(Timestamp,OptQuote<Put>)>) -> ChainsAll {
        let chats: BTreeMap<Timestamp,ChainAt> = BTreeMap::new();
        let mut chall = ChainsAll { chats: chats };
        for (ts, call) in calls {
            chall.add(*ts, call);
        }
        for (ts, put) in puts {
            chall.add(*ts, put);
        }
        return chall;
    }
    fn add<S:Style>(&mut self, ts:Timestamp, oq:&OptQuote<S>) {
        let chat = self.chats.entry(ts).or_insert_with(|| ChainAt::new());
        chat.add(oq);
    }
    pub fn lup<S:Style>(&self, ts:&Timestamp, xpir:&ExpirDate, strike:&PriceCalc) -> Option<&Quote> {
        return self.chats.get(&ts)?.expir(xpir)?.quote::<S>(strike);
    }
}

impl Chall for ChainsAll {
    type R = ChainAt;
    fn at(&self, ts:&Timestamp) -> Option<&Self::R> {
        self.chats.get(&ts)
    }

    fn run<'a>(&'a self, pred:Pred1<&'a Timestamp>, f:fn(&'a Timestamp, &'a Self::R) -> ()) {
        let mut i = 0;
        for (ts, val) in self.chats.iter() {
            if pred(ts) {
                f(ts, val);
            }
            i += 1;
            if i > 10 {
                break;
            }
        }
    }
}

#[derive(Archive, Deserialize, Serialize)]
pub struct ChainAt {
    pub chexs: BTreeMap<ExpirDate,ChainStyles>,
}
impl ChainAt {
    pub fn new() -> ChainAt {
        let chexs: BTreeMap<ExpirDate,ChainStyles> = BTreeMap::new();
        let chat = ChainAt { chexs: chexs };
        return chat;
    }
    fn add<S:Style>(&mut self, oq:&OptQuote<S>) {
        let xpir = oq.opt.expir;
        let chex = self.chexs.entry(xpir).or_insert_with(|| ChainStyles::new());
        chex.add(oq);
    }
}
impl Chat for ChainAt {
    type R = ChainStyles;
    fn expir(&self, xpir:&ExpirDate) -> Option<&Self::R> {
        self.chexs.get(&xpir)
    }
}

#[derive(Archive, Deserialize, Serialize, Debug)]
pub struct ChainStyles {
    pub calls: BTreeMap<StrikeType,Quote>,
    pub puts: BTreeMap<StrikeType,Quote>,
}
impl ChainStyles {
    fn new() -> ChainStyles {
        let calls = BTreeMap::new();
        let puts = BTreeMap::new();
        let chex = ChainStyles { calls: calls, puts: puts };
        return chex;
    }

    fn add<S:Style>(&mut self, oq: &OptQuote<S>) {
        let c = S::switch(&mut self.calls, &mut self.puts);
        c.insert(to_strike(oq.opt.strike), oq.quote);
    }
}
impl Chex for ChainStyles {
    fn quote<S:Style>(&self, strike:&PriceCalc) -> Option<&Quote> {
        let c = S::switch(&self.calls, &self.puts);
        // TODO: is that really the way to do it?
        return c.get(&to_strike(*strike));
    }
    // fn quoteCall(&'a self, strike:StrikeType) -> Option<&'a Quote> {
    //     self.calls.get(&strike)
    // }

    // fn quotePut(&'a self, strike:StrikeType) -> Option<&'a Quote> {
    //     self.puts.get(&strike)
    // }
}

// let quote = chall.at(ts).expir(xpir).quote(opt);

