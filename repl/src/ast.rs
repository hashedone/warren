#[derive(Clone)]
pub enum Term {
    Var(String),
    Const(String),
    Struct(String, Vec<Term>),
}

impl std::fmt::Debug for Term {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Var(id) => write!(f, "?{}", id),
            Self::Const(id) => write!(f, "{}", id),
            Self::Struct(id, subterms) => {
                let subterms: Vec<_> = subterms.into_iter().map(|st| format!("{:?}", st)).collect();
                let subterms = subterms.join(", ");
                write!(f, "{}({})", id, subterms)
            }
        }
    }
}

#[derive(Debug)]
pub enum Statement {
    Query(Term),
}
