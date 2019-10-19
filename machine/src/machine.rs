use crate::query::{Query, QueryResult};
use crate::storage::{Cell, Storage};
use crate::{Operation, Program};

enum UnificationState {
    Read,
    Write,
}

pub struct Machine {
    pub(crate) storage: Storage,
    preg: usize,                         // Instruction pointer register
    sreg: usize,                         // S register
    unification_state: UnificationState, // Read/Write state for unification
}

impl Default for Machine {
    fn default() -> Self {
        Self {
            storage: Storage::new(),
            preg: 0,
            sreg: 0,
            unification_state: UnificationState::Read,
        }
    }
}

impl Machine {
    pub fn new() -> Self {
        Default::default()
    }

    fn run(&mut self, program: &Program) {
        self.preg = 0;
        while let Some(op) = program.operation(self.preg) {
            self.perform_op(op);
            self.preg += op.advance();
        }
    }

    pub fn query(&mut self, query: Query) -> QueryResult {
        self.storage.reset(query.program.x_registers());

        self.run(&query.program);
        if query.top_level != 0 {
            // 0 register should contain top level structure
            self.storage[0] = self.storage[query.top_level];
        }

        let regs = query.program.x_registers();
        QueryResult {
            machine: self,
            regs: self.storage.registers()[0..regs].to_vec(),
        }
    }

    fn perform_op(&mut self, op: Operation) -> bool {
        match op {
            Operation::PutStructure(ident, arity, xreg) => self.put_structure(ident, arity, xreg),
            Operation::SetVariable(xreg) => self.set_variable(xreg),
            Operation::SetValue(xreg) => self.set_value(xreg),
            Operation::GetStructure(ident, arity, xreg) => self.get_structure(ident, arity, xreg),
            Operation::UnifyVariable(xreg) => self.unify_variable(xreg),
            Operation::UnifyValue(xreg) => self.unify_value(xreg),
        }
    }

    fn put_structure(&mut self, ident: usize, arity: usize, xreg: usize) -> bool {
        let cell = self.storage.push_struct(ident, arity);
        self.storage[xreg] = cell;
        true
    }

    fn set_variable(&mut self, xreg: usize) -> bool {
        let cell = self.storage.push_var();
        self.storage[xreg] = cell;
        true
    }

    fn set_value(&mut self, xreg: usize) -> bool {
        self.storage.push_cell(self.storage[xreg]);
        true
    }

    fn get_structure(&mut self, ident: usize, arity: usize, xreg: usize) -> bool {
        let item = if let Some(item) = self.storage.deref(xreg) {
            item
        } else {
            return false;
        };

        match item {
            Cell::Ref(r) => {
                let idx = self.storage.len();
                self.storage.push_struct(ident, arity);
                self.storage.bind(r, idx);
                self.unification_state = UnificationState::Write;
                true
            }
            Cell::Struct(a) => {
                if Cell::Funct(ident, arity) == self.storage[a] {
                    self.sreg = a + 1;
                    self.unification_state = UnificationState::Read;
                    true
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    fn unify_variable(&mut self, xreg: usize) -> bool {
        match self.unification_state {
            UnificationState::Read => {
                self.storage[xreg] = self.storage[self.sreg];
            }
            UnificationState::Write => {
                self.storage[xreg] = self.storage.push_var();
            }
        }
        self.sreg += 1;
        true
    }

    fn unify_value(&mut self, xreg: usize) -> bool {
        match self.unification_state {
            UnificationState::Read => {
                self.storage.unify(xreg, self.sreg);
            }
            UnificationState::Write => {
                self.storage.push_cell(self.storage[xreg]);
            }
        }
        self.sreg += 1;
        true
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

        let query = builder.build(p);

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
