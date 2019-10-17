use crate::{Cell, Program, Operation};
use crate::storage::StorageMut;

pub struct Machine {
    pub(crate) heap: Vec<Cell>,    // Heap
    pub(crate) xregs: Vec<Cell>,   // X Registers
    pub(crate) preg: usize,        // Instruction pointer register
}

impl Default for Machine {
    fn default() -> Self {
        Self {
            heap: vec![],
            xregs: vec![],
            preg: 0,
        }
    }
}

impl Machine {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn run(&mut self, program: &Program) {
        if self.xregs.len() < program.x_registers() {
            self.xregs.resize_with(
                program.x_registers(),
                Default::default
            );
        }

        self.preg = 0;
        while let Some(op) = program.operation(self.preg) {
            self.perform_op(op);
            self.preg += op.advance();
        }
    }

    fn perform_op(&mut self, op: Operation) {
        match op {
            Operation::PutStructure(ident, arity, xreg) =>
                self.put_structure(ident, arity, xreg),
            Operation::SetVariable(xreg) => self.set_variable(xreg),
            Operation::SetValue(xreg) => self.set_value(xreg),
        }
    }

    fn put_structure(&mut self, ident: usize, arity: usize, xreg: usize) {
        self.xregs[xreg] = self.heap.push_struct(ident, arity);
    }

    fn set_variable(&mut self, xreg: usize) {
        self.xregs[xreg] = self.heap.push_var();
    }

    fn set_value(&mut self, xreg: usize) {
        self.heap.push(self.xregs[xreg]);
    }
}

#[cfg(test)]
mod tests {
    use super::Machine;
    use crate::{program::ProgramBuilder, Cell};

    #[test]
    fn l0_query() {
        let program = {
            let mut builder = ProgramBuilder::new();
            builder
                .put_structure(0, 2, 2)
                .set_variable(1)
                .set_variable(4)
                .put_structure(1, 1, 3)
                .set_value(4)
                .put_structure(2, 3, 0)
                .set_value(1)
                .set_value(2)
                .set_value(3);
            builder.build()
        };

        let heap = vec![
            Cell::Struct(1),
            Cell::Funct(0, 2),
            Cell::Ref(2),
            Cell::Ref(3),
            Cell::Struct(5),
            Cell::Funct(1, 1),
            Cell::Ref(3),
            Cell::Struct(8),
            Cell::Funct(2, 3),
            Cell::Ref(2),
            Cell::Struct(1),
            Cell::Struct(5),
        ];

        let regs = vec![
            Cell::Struct(8),
            Cell::Ref(2),
            Cell::Struct(1),
            Cell::Struct(5),
            Cell::Ref(3),
        ];

        let mut machine = Machine::new();
        machine.run(&program);

        assert_eq!(machine.heap, heap);
        assert_eq!(machine.xregs, regs);
    }
}
