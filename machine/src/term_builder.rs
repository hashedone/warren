use crate::storage::Storage;
use crate::{Cell, Machine};

pub trait TermBuilder {
    type Term;

    fn variable(&mut self, id: usize) -> Self::Term;
    fn structure(&mut self, ident: usize, subterms: impl Iterator<Item = Self::Term>)
        -> Self::Term;
    fn constant(&mut self, ident: usize) -> Self::Term {
        self.structure(ident, std::iter::empty())
    }
}

impl Machine {
    pub(crate) fn build_term<Builder: TermBuilder>(
        &self,
        cell: Cell,
        builder: &mut Builder,
    ) -> Option<Builder::Term> {
        match cell {
            Cell::Ref(idx) => {
                let target = self.heap.deref(idx)?;

                if let Cell::Ref(idx) = target {
                    Some(builder.variable(idx))
                } else {
                    self.build_term(target, builder)
                }
            }
            Cell::Struct(idx) => {
                if let Cell::Funct(ident, arity) = self.heap.get(idx)? {
                    if *arity == 0 {
                        Some(builder.constant(*ident))
                    } else {
                        let subterms: Option<Vec<_>> =
                            self.heap[idx + 1..=idx + arity]
                                .iter()
                                .map(|cell| self.build_term(*cell, builder))
                                .collect();
                        let subterms = subterms?;

                        Some(builder.structure(*ident, subterms.into_iter()))
                    }
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::ast::{Builder, Term};
    use crate::{Cell, Machine};

    #[test]
    fn single_const() {
        let heap = vec![Cell::Struct(1), Cell::Funct(0, 0)];

        let regs = vec![heap[0]];

        let machine = Machine {
            heap,
            xregs: regs,
            ..Default::default()
        };

        let term = machine.build_term(Cell::Struct(1), &mut Builder).unwrap();
        let expected = Term::Const(0);

        assert_eq!(expected, term);
    }

    #[test]
    fn single_var() {
        let heap = vec![Cell::Ref(0)];

        let regs = vec![heap[0]];

        let machine = Machine {
            heap,
            xregs: regs,
            ..Default::default()
        };

        let term = machine.build_term(Cell::Ref(0), &mut Builder).unwrap();
        let expected = Term::Var(0);

        assert_eq!(expected, term);
    }

    #[test]
    fn sample_term() {
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

        let regs = vec![heap[7]];

        let machine = Machine {
            heap,
            xregs: regs,
            ..Default::default()
        };

        let term = machine.build_term(Cell::Struct(8), &mut Builder).unwrap();

        let expected = Term::Struct(
            2,
            vec![
                Term::Var(2),
                Term::Struct(0, vec![Term::Var(2), Term::Var(3)]),
                Term::Struct(1, vec![Term::Var(3)]),
            ],
        );

        assert_eq!(expected, term);
    }
}
