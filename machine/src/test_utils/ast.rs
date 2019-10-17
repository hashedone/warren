use crate::term_builder::TermBuilder;

#[derive(Clone, PartialEq, Eq)]
pub enum Term {
    Var(usize),
    Const(usize),
    Struct(usize, Vec<Term>),
}

pub struct Builder;

impl TermBuilder for Builder {
    type Term = Term;

    fn variable(&self, id: usize) -> Term {
        Term::Var(id)
    }

    fn constant(&self, ident: usize) -> Term {
        Term::Const(ident)
    }

    fn structure(
        &self,
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
