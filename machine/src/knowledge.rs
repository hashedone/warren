use crate::Program;
use crate::statement::Statement;
use derivative::Derivative;

#[derive(Derivative)]
#[derivative(Default)]
pub struct Knowledge<'a> {
    programs: Vec<Program<'a>>,
}

impl<'a> Knowledge<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add(&mut self, fact: Statement<'a>) -> &mut Self {
        self.programs.push(fact.program);
        self
    }

    pub(crate) fn x_registers(&self) -> usize {
        self.programs
            .iter()
            .map(|p| p.x_registers())
            .max()
            .unwrap_or(0)
    }

    pub(crate) fn programs(&self) -> impl Iterator<Item=&Program> {
        self.programs.iter()
    }
}
