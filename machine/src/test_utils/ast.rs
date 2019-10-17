use crate::term_builder::TermBuilder;
use std::collections::{HashMap, HashSet};

#[derive(Clone)]
pub enum Term {
    Var(usize),
    Const(usize),
    Struct(usize, Vec<Term>),
}

pub struct Builder;

impl TermBuilder for Builder {
    type Term = Term;

    fn variable(&mut self, id: usize) -> Term {
        Term::Var(id)
    }

    fn constant(&mut self, ident: usize) -> Term {
        Term::Const(ident)
    }

    fn structure(
        &mut self,
        ident: usize,
        subterms: impl Iterator<Item=Term>,
    ) -> Term {
        Term::Struct(ident, subterms.collect())
    }
}

impl std::fmt::Debug for Term {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Var(id) => write!(f, "?{}", id),
            Self::Const(ident) => write!(f, "_{}", ident),
            Self::Struct(ident, subterms) => {
                let subterms: Vec<_> = subterms.into_iter()
                    .map(|st| format!("{:?}", st))
                    .collect();
                let subterms = subterms.join(", ");
                write!(f, "_{}({})", ident, subterms)
            }
        }
    }
}

impl Term {
    fn same(
        &self,
        other: &Term,
        mapping: &mut HashMap<usize, usize>
    ) -> bool {
        match (self, other) {
            (Self::Var(s), Self::Var(o)) => {
                mapping.entry(*s).or_insert(*o) == o
            },
            (Self::Const(s), Self::Const(o)) if s == o => true,
            (Self::Struct(s, ss), Self::Struct(o, so)) if s == o => {
                ss.iter().zip(so.iter())
                    .all(|(s, o)| s.same(o, mapping))
            },
            _ => false,
        }
    }
}

impl PartialEq for Term {
    // Not very eficient, but its for testing purposes
    // Should NOT be used for any benchmarking
    fn eq(&self, other: &Term) -> bool {
        let mut mapping = Default::default();
        let same = self.same(other, &mut mapping);
        let mappings = mapping.len();
        let mapping: HashSet<_> = mapping.into_iter()
            .map(|(_, o)| o)
            .collect();
        same && mapping.len() == mappings
    }
}

impl Eq for Term {}
