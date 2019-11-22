use crate::query::{Query, QueryResult};
use crate::storage::{Cell, Storage};
use crate::{Operation, Program};
use crate::Knowledge;

enum UnificationState {
    Read,
    Write,
}

pub struct Machine {
    storage: Storage,
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

    #[cfg(test)]
    pub(crate) fn with_storage(storage: Storage) -> Self {
        Self {
            storage,
            ..Default::default()
        }
    }

    fn run(&mut self, program: &Program) {
        self.preg = 0;
        while let Some(op) = program.operation(self.preg) {
            self.perform_op(op);
        }
    }

    pub(crate) fn storage(&self) -> &Storage {
        &self.storage
    }

    pub fn query<'a>(
        &'a mut self,
        query: Query,
        knowledge: &Knowledge
    ) -> QueryResult {
        let regs = std::cmp::max(
            query.program.x_registers(),
            knowledge.x_registers()
        );

        self.storage.reset(regs);

        self.run(&query.program);
        if query.top_level != 0 {
            // 0 register should contain top level structure
            self.storage[0] = self.storage[query.top_level];
        }

        for fact in knowledge.programs().take(1) {
            self.run(fact);
        }

        let regs = query.program.x_registers();
        QueryResult {
            machine: self,
            regs: self.storage.registers()[0..regs].to_vec(),
        }
    }

    pub(crate) fn perform_op(&mut self, op: Operation) -> bool {
        let res = match op {
            Operation::PutStructure(ident, arity, xreg) => self.put_structure(ident, arity, xreg),
            Operation::SetVariable(xreg) => self.set_variable(xreg),
            Operation::SetValue(xreg) => self.set_value(xreg),
            Operation::GetStructure(ident, arity, xreg) => self.get_structure(ident, arity, xreg),
            Operation::UnifyVariable(xreg) => self.unify_variable(xreg),
            Operation::UnifyValue(xreg) => self.unify_value(xreg),
        };

        self.preg += op.advance();
        res
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
    use crate::statement::StatementBuilder;
    use crate::knowledge::Knowledge;
    use crate::test_utils::ast::{Builder as TermBuilder, Term};

    #[test]
    fn l0_query() {
        // p(Z, h(Z, W), f(W))
        // p/3 := 2
        // h/2 := 1
        // f/1 := 0
        let mut builder = QueryBuilder::new();
        let w = builder.variable();
        let z = builder.variable();
        let h = builder.structure(1, vec![z, w]);
        let f = builder.structure(0, vec![w]);
        let p = builder.structure(2, vec![z, h, f]);

        let query = builder.build(p);

        let mut machine = Machine::new();
        machine.query(query, &Default::default());
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

    #[test]
    fn l0_program() {
        // p/3 := 2
        // h/2 := 1
        // f/1 := 0
        // a/0 := 3

        let fact = {
            // p(f(X), h(Y, f(a)), Y)
            let mut builder = StatementBuilder::new();
            let x = builder.variable();
            let f0 = builder.structure(0, vec![x]);
            let y = builder.variable();
            let a = builder.constant(3);
            let f1 = builder.structure(0, vec![a]);
            let h = builder.structure(2, vec![y, f1]);
            let p = builder.structure(3, vec![f0, h, y]);

            builder.build(p)
        };

        let (query, p) = {
            // p(Z, h(Z, W), f(W))
            let mut builder = QueryBuilder::new();
            let w = builder.variable();
            let z = builder.variable();
            let h = builder.structure(1, vec![z, w]);
            let f = builder.structure(0, vec![w]);
            let p = builder.structure(2, vec![z, h, f]);

            (builder.build(p), p)
        };

        let mut machine = Machine::new();
        machine.query(query, Knowledge::new().add(fact));
        let term = machine
            .build_term(machine.storage[p.0], &mut TermBuilder)
            .unwrap();

        // ???
        // _2(?24, _1(?24, ?23), _0(?23))
        let expected_term = Term::Const(0);

        assert_eq!(expected_term, term);
    }
}
