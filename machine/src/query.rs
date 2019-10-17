use crate::Program;
use crate::program::ProgramBuilder;
use std::borrow::Borrow;

/// Reference to query part for building complex (structure)
/// queries, and later for extracting unification result
#[derive(Clone, Copy)]
pub struct QueryRef(pub(crate) usize);

/// Query to be executed
pub struct Query<'a> {
    pub(crate) program: Program<'a>,
}

/// Builder for structured query
pub struct QueryBuilder {
    program: ProgramBuilder,
    next_register: usize,
}

impl Default for QueryBuilder {
    fn default() -> Self {
        Self {
            program: Default::default(),
            next_register: 0,
        }
    }
}

impl QueryBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    fn next_register(&mut self) -> usize {
        self.next_register += 1;
        self.next_register - 1
    }

    pub fn variable(&mut self) -> QueryRef {
        let register = self.next_register();
        self.program.set_variable(register);
        QueryRef(register)
    }

    pub fn structure(
        &mut self,
        ident: usize,
        subterms: impl ExactSizeIterator<Item=impl Borrow<QueryRef>>,
    ) -> QueryRef {
        let register = self.next_register();
        self.program.put_structure(ident, subterms.len(), register);
        for subterm in subterms {
            let QueryRef(reg) = subterm.borrow();
            self.program.set_value(*reg);
        }
        QueryRef(register)
    }

    pub fn constant(
        &mut self,
        ident: usize,
    ) -> QueryRef {
        self.structure(ident, std::iter::empty::<QueryRef>())
    }

    pub fn build(self) -> Query<'static> {
        Query {
            program: self.program.build(),
        }
    }
}
