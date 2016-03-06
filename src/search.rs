#![allow(missing_docs)]
// TODO: unfinished functionality

use std::fmt;
use std::borrow::Cow;
use std::convert::Into;

pub enum Term<'a> {
    Any,
    File,
    Base,
    LastMod,
    Tag(Cow<'a, str>),
}

pub struct Filter<'a> {
    typ: Term<'a>,
    what: Cow<'a, str>,
}

impl<'a> Filter<'a> {
    fn new<W>(typ: Term<'a>, what: W) -> Filter
        where W: 'a + Into<Cow<'a, str>>
    {
        Filter {
            typ: typ,
            what: what.into(),
        }
    }
}

pub struct Query<'a> {
    filters: Vec<Filter<'a>>,
    window: Option<(u32, u32)>,
    fuzzy: bool,
}

impl<'a> Query<'a> {
    pub fn new() -> Query<'a> {
        Query {
            filters: Vec::new(),
            window: None,
            fuzzy: false,
        }
    }

    pub fn filter<'b: 'a, V: 'b + Into<Cow<'b, str>>>(&mut self, term: Term<'b>, value: V) -> &mut Query<'a> {
        self.filters.push(Filter::new(term, value));
        self
    }

    pub fn limit(&mut self, limit: u32) -> &mut Query<'a> {
        self.window(0, limit)
    }

    pub fn window(&mut self, offset: u32, limit: u32) -> &mut Query<'a> {
        self.window = Some((offset, limit));
        self
    }

    pub fn fuzzy(&mut self) -> &mut Query<'a> {
        self.fuzzy = true;
        self
    }
}

impl<'a> fmt::Display for Term<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Term::Any => f.write_str("any"),
            Term::File => f.write_str("file"),
            Term::Base => f.write_str("base"),
            Term::LastMod => f.write_str("modified-since"),
            Term::Tag(ref tag) => f.write_str(&*tag),
        }
    }
}

impl<'a> fmt::Display for Filter<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.typ, self.what)
    }
}

impl<'a> fmt::Display for Query<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.fuzzy {
            write!(f, "find");
        } else {
            write!(f, "search");
        }

        for filter in &self.filters {
            try!(filter.fmt(f));
        }

        match self.window {
            Some((a, b)) => write!(f, " window {}:{}", a, b),
            None => Ok(()),
        }
    }
}
