use std::collections::{HashMap, BTreeMap};
use crate::general::*;
use crate::market::types::*;

trait Chall {
    type R;
    fn at(&self, ts:Timestamp) -> Option<&Self::R>;
}

trait Chat {
    type R;
    fn expir(&self, xpir:ExpirDate) -> Option<&Self::R>;
}

trait Chex {
    fn quote<S:Style>(&self, strike:PriceCalc) -> Option<&Quote>;
}

pub struct ChainsAll {
    chats: HashMap<Timestamp,ChainAt>,
}
impl ChainsAll {
    pub fn new(calls: &Vec<(Timestamp,OptQuote<Call>)>, puts: &Vec<(Timestamp,OptQuote<Put>)>) -> ChainsAll {
        let chats: HashMap<Timestamp,ChainAt> = HashMap::new();
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
    // fn addPut(ts:Timestamp, oq:OptQuote<Call>) {
    //     let chat = chats.entry(ts).or_insert_with(|| ChainAt::new());
    //     chat.add(oq);
    // }
    pub fn lup<S:Style>(&self, ts:Timestamp, xpir:ExpirDate, strike:PriceCalc) -> Option<&Quote> {
        return self.chats.get(&ts)?.expir(xpir)?.quote::<S>(strike);
    }
}
impl Chall for ChainsAll {
    type R = ChainAt;
    fn at(&self, ts:Timestamp) -> Option<&Self::R> {
        self.chats.get(&ts)
    }
}

pub struct ChainAt {
    chexs: HashMap<ExpirDate,ChainStyles>,
}
impl ChainAt {
    pub fn new() -> ChainAt {
        let chexs: HashMap<ExpirDate,ChainStyles> = HashMap::new();
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
    fn expir(&self, xpir:ExpirDate) -> Option<&Self::R> {
        self.chexs.get(&xpir)
    }
}

pub struct ChainStyles {
    calls: BTreeMap<StrikeType,Quote>,
    puts: BTreeMap<StrikeType,Quote>,
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
    fn quote<S:Style>(&self, strike:PriceCalc) -> Option<&Quote> {
        let c = S::switch(&self.calls, &self.puts);
        // TODO: is that really the way to do it?
        return c.get(&to_strike(strike));
    }
    // fn quoteCall(&'a self, strike:StrikeType) -> Option<&'a Quote> {
    //     self.calls.get(&strike)
    // }

    // fn quotePut(&'a self, strike:StrikeType) -> Option<&'a Quote> {
    //     self.puts.get(&strike)
    // }
}

// let quote = chall.at(ts).expir(xpir).quote(opt);

