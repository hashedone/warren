use crate::program::ProgramBuilder;
use crate::Program;
use std::borrow::Borrow;

/// Reference to statement part for building complex (structure)
/// statements
#[derive(Clone, Copy)]
pub struct StatementRef(pub(crate) usize);

/// Statement to be added to machine state
pub struct Statement<'a> {
    pub(crate) program: Program<'a>,
}

#[derive(Clone)]
enum RegisterAllocation {
    Var,
    Struct(usize, Vec<usize>),
}

/// Builder for structured statement
pub struct StatementBuilder {
    registers: Vec<RegisterAllocation>,
}

impl Default for StatementBuilder {
    fn default() -> Self {
        Self {
            // First register is reserved for top-level
            // structure
            registers: vec![RegisterAllocation::Var]
        }
    }
}

impl StatementBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn variable(&mut self) -> StatementRef {
        self.registers.push(RegisterAllocation::Var);
        StatementRef(self.registers.len() - 1)
    }

    pub fn structure(
        &mut self,
        ident: usize,
        subterms: impl IntoIterator<Item = StatementRef>,
    ) -> StatementRef {
        self.registers.push(RegisterAllocation::Struct(
            ident,
            subterms.into_iter().map(|StatementRef(r)| r).collect()
        ));
        StatementRef(self.registers.len() - 1)
    }

    pub fn constant(&mut self, ident: usize) -> StatementRef {
        self.structure(ident, std::iter::empty())
    }

    pub fn build(
        mut self,
        StatementRef(r): StatementRef
    ) -> Statement<'static> {
        self.registers.swap(0, r);

        unimplemented!()
    }
}
