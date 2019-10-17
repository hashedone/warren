use crate::Operation;
use std::borrow::Cow;
use std::cmp::max;

#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum OpCode {
    PutStructure, // Op Ident Arity XReg
    SetVariable,  // Op XReg
    SetValue,     // Op XReg
}

impl PartialEq<usize> for OpCode {
    fn eq(&self, other: &usize) -> bool {
        *self as usize == *other
    }
}

impl PartialEq<OpCode> for usize {
    fn eq(&self, other: &OpCode) -> bool {
        *self == *other as usize
    }
}

pub struct Program<'a> {
    program: Cow<'a, [usize]>,
    xregs: usize, // X registers to alocate
}

impl Default for Program<'static> {
    fn default() -> Self {
        Self {
            program: Cow::Owned(Default::default()),
            xregs: 0,
        }
    }
}

impl<'a> Program<'a> {
    // Builds `PutStructure` from given program index
    fn put_structure(&self, index: usize) -> Option<Operation> {
        if self.program.len() > index + 3 {
            let ident = self.program[index + 1];
            let arity = self.program[index + 2];
            let xreg = self.program[index + 3];
            Some(Operation::PutStructure(ident, arity, xreg))
        } else {
            None
        }
    }

    // Builds `SetVariable` from given program index
    fn set_variable(&self, index: usize) -> Option<Operation> {
        if self.program.len() > index + 1 {
            let xreg = self.program[index + 1];
            Some(Operation::SetVariable(xreg))
        } else {
            None
        }
    }

    // Builds `SetValue` from given program index
    fn set_value(&self, index: usize) -> Option<Operation> {
        if self.program.len() > index + 1 {
            let xreg = self.program[index + 1];
            Some(Operation::SetValue(xreg))
        } else {
            None
        }
    }

    /// Gives operation from given program index
    pub fn operation(&self, index: usize) -> Option<Operation> {
        match self.program.get(index)? {
            op if *op == OpCode::PutStructure => self.put_structure(index),
            op if *op == OpCode::SetVariable => self.set_variable(index),
            op if *op == OpCode::SetValue => self.set_value(index),
            _ => None,
        }
    }

    /// Gives minimal number of X registers which should be
    /// allocated to run this program
    ///
    /// This is highest index of registers used in program + 1
    pub fn x_registers(&self) -> usize {
        self.xregs
    }
}

pub struct ProgramBuilder {
    program: Vec<usize>,
    xregs: usize, // X registers to allocate
}

impl Default for ProgramBuilder {
    fn default() -> Self {
        Self {
            program: vec![],
            xregs: 0,
        }
    }
}

impl ProgramBuilder {
    pub fn put_structure(&mut self, ident: usize, arity: usize, xreg: usize) -> &mut Self {
        self.xregs = max(self.xregs, xreg + 1);

        self.program.push(OpCode::PutStructure as usize);
        self.program.push(ident);
        self.program.push(arity);
        self.program.push(xreg);
        self
    }

    pub fn set_variable(&mut self, xreg: usize) -> &mut Self {
        self.xregs = max(self.xregs, xreg + 1);

        self.program.push(OpCode::SetVariable as usize);
        self.program.push(xreg);
        self
    }

    pub fn set_value(&mut self, xreg: usize) -> &mut Self {
        self.xregs = max(self.xregs, xreg + 1);

        self.program.push(OpCode::SetValue as usize);
        self.program.push(xreg);
        self
    }

    pub fn build(self) -> Program<'static> {
        Program {
            program: self.program.into(),
            xregs: self.xregs,
        }
    }
}
