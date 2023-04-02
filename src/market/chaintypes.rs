use std::collections::{BTreeMap, btree_map};
use std::ops::Range;
use speedy::{Readable, Writable};

use crate::general::{*, self};
use crate::market::types::*;

pub trait Chall {
    type R:Chat;
    fn tss_all(&self) -> btree_map::Keys<'_, general::Timestamp, ChainAt>;
    fn at(&self, ts:&Timestamp) -> Option<&Self::R>;
    fn run_all(&self, f:fn(&Timestamp, &Self::R) -> ()) {
        self.run(|_| true, f)
    }
    fn run(&self, pred:fn(&Timestamp) -> bool, f:fn(&Timestamp, &Self::R) -> ());
    fn run_range(&self, rang:Range<Timestamp>, f:fn(&Timestamp, &Self::R) -> ());
}

pub trait Chat {
    type R:Chex;
    fn expir(&self, xpir:&ExpirDate) -> Option<&Self::R>;
    fn under(&self) -> PriceCalc;
}

pub trait Chex {
    fn quote<S:Style>(&self, strike:&PriceCalc) -> Option<&QuoteOption>;
}

#[derive(Readable, Writable)]
pub struct ChainsAll {
    pub chats: BTreeMap<Timestamp,ChainAt>,
}
impl ChainsAll {
    pub fn new(unders: &Vec<(Timestamp,PriceCalc)>, calls: &Vec<(Timestamp,OptQuote<Call>)>, puts: &Vec<(Timestamp,OptQuote<Put>)>) -> ChainsAll {
        let chats: BTreeMap<Timestamp,ChainAt> = BTreeMap::new();
        let mut chall = ChainsAll { chats: chats };
        for (ts, under) in unders {
            chall.chats.insert(*ts, ChainAt::new(*under));
        }

        let mut ts_prev = calls.first().unwrap().0;
        let mut chat = chall.chats.get_mut(&ts_prev).unwrap();
        for (ts, oq) in calls {
            if *ts != ts_prev {
                ts_prev = *ts;
                chat = chall.chats.get_mut(&ts_prev).unwrap();
            }
            chat.add(oq);
        }

        ts_prev = puts.first().unwrap().0;
        chat = chall.chats.get_mut(&ts_prev).unwrap();
        for (ts, oq) in puts {
            if *ts != ts_prev {
                ts_prev = *ts;
                chat = chall.chats.get_mut(&ts_prev).unwrap();
            }
            chat.add(oq);
        }

        return chall;
    }
    // fn add<S:Style>(&mut self, ts:Timestamp, oq:&OptQuote<S>) {
    //     let chat = self.chats.entry(ts).or_insert_with(|| ChainAt::new());
    //     chat.add(oq);
    // }
    pub fn lup<S:Style>(&self, ts:&Timestamp, xpir:&ExpirDate, strike:&PriceCalc) -> Option<&QuoteOption> {
        return self.chats.get(&ts)?.expir(xpir)?.quote::<S>(strike);
    }
}

impl Chall for ChainsAll {
    type R = ChainAt;

    fn tss_all(&self) -> btree_map::Keys<'_, general::Timestamp, ChainAt> {
        self.chats.keys()
    }

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

    fn run_range(&self, rang:Range<Timestamp>, f:fn(&Timestamp, &Self::R) -> ()) {
        self.chats.range(rang).for_each(|(ts,val)| f(ts, val));
    }
}

#[derive(Readable, Writable)]
pub struct ChainAt {
    pub under: PriceCalc,
    pub chexs: BTreeMap<ExpirDate,ChainStyles>,
}
impl ChainAt {
    pub fn new(under: PriceCalc) -> ChainAt {
        let chexs: BTreeMap<ExpirDate,ChainStyles> = BTreeMap::new();
        let chat = ChainAt { under: under, chexs: chexs };
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
    fn under(&self) -> PriceCalc {
        self.under
    }
    fn expir(&self, xpir:&ExpirDate) -> Option<&Self::R> {
        self.chexs.get(&xpir)
    }
}

#[derive(Readable, Writable)]
pub struct ChainStyles {
    pub calls: BTreeMap<StrikeType,QuoteOption>,
    pub puts: BTreeMap<StrikeType,QuoteOption>,
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
        c.insert(oq.opt.strike, oq.quote);
    }
}
impl Chex for ChainStyles {
    fn quote<S:Style>(&self, strike:&PriceCalc) -> Option<&QuoteOption> {
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

