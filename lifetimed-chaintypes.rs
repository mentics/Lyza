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
    fn quote<S:Style>(&mut self, strike:StrikeType) -> Option<&Quote>;
    // fn quoteCall(&'a self, strike:StrikeType) -> Option<&'a Quote>;
    // fn quotePut(&'a self, strike:StrikeType) -> Option<&'a Quote>;
}

pub struct ChainsAll<'a> {
    chats: HashMap<Timestamp,ChainAt<'a>>,
}
impl<'a> ChainsAll<'a> {
    pub fn new(calls: &'a Vec<(Timestamp,OptQuote<Call>)>, puts: &'a Vec<(Timestamp,OptQuote<Put>)>) -> ChainsAll<'a> {
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
    fn add<S:Style>(&mut self, ts:Timestamp, oq:&'a OptQuote<S>) {
        let chat = self.chats.entry(ts).or_insert_with(|| ChainAt::new());
        chat.add(oq);
    }
    // fn addPut(ts:Timestamp, oq:OptQuote<Call>) {
    //     let chat = chats.entry(ts).or_insert_with(|| ChainAt::new());
    //     chat.add(oq);
    // }
}
impl<'a> Chall<'a> for ChainsAll<'a> {
    type R = &'a ChainAt<'a>;
    fn at(&'a self, ts:Timestamp) -> Option<Self::R> {
        self.chats.get(&ts)
    }
}

pub struct ChainAt<'a> {
    chexs: HashMap<ExpirDate,ChainStyles<'a>>,
}
impl<'a> ChainAt<'a> {
    pub fn new() -> ChainAt<'a> {
        let chexs: HashMap<ExpirDate,ChainStyles> = HashMap::new();
        let chat = ChainAt { chexs: chexs };
        return chat;
    }
    fn add<S:Style>(&mut self, oq:&'a OptQuote<S>) {
        let xpir = oq.opt.expir;
        let chex = self.chexs.entry(xpir).or_insert_with(|| ChainStyles::new());
        chex.add(oq);
    }
}
impl<'a> Chat<'a> for ChainAt<'a> {
    type R = &'a ChainStyles<'a>;
    fn expir(&'a self, xpir:ExpirDate) -> Option<Self::R> {
        self.chexs.get(&xpir)
    }
}

pub struct ChainStyles<'a> {
    calls: BTreeMap<StrikeType,&'a Quote>,
    puts: BTreeMap<StrikeType,&'a Quote>,
}
impl<'a> ChainStyles<'a> {
    fn new() -> ChainStyles<'a> {
        let calls = BTreeMap::new();
        let puts = BTreeMap::new();
        let chex = ChainStyles { calls: calls, puts: puts };
        return chex;
    }

    fn add<S:Style>(&mut self, oq: &'a OptQuote<S>) {
        let c = S::switch(&mut self.calls, &mut self.puts);
        c.insert(to_strike(oq.opt.strike), &oq.quote);
    }
}
impl<'a> Chex<'a> for ChainStyles<'a> {
    fn quote<S:Style>(&mut self, strike:StrikeType) -> Option<&'a Quote> {
        let c = S::switch(&mut self.calls, &mut self.puts);
        // TODO: is that really the way to do it?
        return c.get(&strike).map(|val| *val);
    }
    // fn quoteCall(&'a self, strike:StrikeType) -> Option<&'a Quote> {
    //     self.calls.get(&strike)
    // }

    // fn quotePut(&'a self, strike:StrikeType) -> Option<&'a Quote> {
    //     self.puts.get(&strike)
    // }
}

// let quote = chall.at(ts).expir(xpir).quote(opt);

