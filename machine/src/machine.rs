use crate::query::{Query, QueryResult};
use crate::storage::Storage;
use crate::{Operation, Program};

pub struct Machine {
    pub(crate) storage: Storage,
    pub(crate) preg: usize,      // Instruction pointer register
}

impl Default for Machine {
    fn default() -> Self {
        Self {
            storage: Storage::new(),
            preg: 0,
        }
    }
}

impl Machine {
    pub fn new() -> Self {
        Default::default()
    }

    fn run(&mut self, program: &Program) {
        self.storage.reset(program.x_registers());

        self.preg = 0;
        while let Some(op) = program.operation(self.preg) {
            self.perform_op(op);
            self.preg += op.advance();
        }
    }

    pub fn query(&mut self, query: Query) -> QueryResult {
        self.run(&query.program);

        let regs = query.program.x_registers();
        QueryResult {
            machine: self,
            regs: self.storage.registers()[0..regs].to_vec(),
        }
    }

    fn perform_op(&mut self, op: Operation) {
        match op {
            Operation::PutStructure(ident, arity, xreg) => self.put_structure(ident, arity, xreg),
            Operation::SetVariable(xreg) => self.set_variable(xreg),
            Operation::SetValue(xreg) => self.set_value(xreg),
        }
    }

    fn put_structure(&mut self, ident: usize, arity: usize, xreg: usize) {
        let cell = self.storage.push_struct(ident, arity);
        self.storage[xreg] = cell;
    }

    fn set_variable(&mut self, xreg: usize) {
        let cell = self.storage.push_var();
        self.storage[xreg] = cell;
    }

    fn set_value(&mut self, xreg: usize) {
        self.storage.push_cell(self.storage[xreg]);
    }
}

#[cfg(test)]
mod tests {
    use super::Machine;
    use crate::query::QueryBuilder;
    use crate::test_utils::ast::{Builder as TermBuilder, Term};

    #[test]
    fn l0_query() {
        let mut builder = QueryBuilder::new();
        let w = builder.variable();
        let z = builder.variable();
        let h = builder.structure(1, [z, w].iter());
        let f = builder.structure(0, [w].iter());
        let p = builder.structure(2, [z, h, f].iter());

        let query = builder.build();

        let mut machine = Machine::new();
        machine.query(query);
        let term = machine
            .build_term(machine.storage[p.0], &mut TermBuilder)
            .unwrap();

        // _2(?0, _1(?0, ?1), _0(?1))
        let expected_term = Term::Struct(
            2,
            vec![
                Term::Var(0),
                Term::Struct(1, vec![Term::Var(0), Term::Var(1)]),
                Term::Struct(0, vec![Term::Var(1)]),
            ],
        );

        assert_eq!(expected_term, term);
    }
}
