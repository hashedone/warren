use crate::ast::Term;
use bimap::BiMap;
use std::collections::HashMap;
use warren::query::{Query, QueryBuilder, QueryRef};
use warren::statement::{Statement, StatementBuilder, StatementRef};
use warren::TermBuilder;

pub struct Context {
    terms_mapping: BiMap<String, usize>,
}

impl Default for Context {
    fn default() -> Self {
        Self {
            terms_mapping: Default::default(),
        }
    }
}

impl Context {
    fn get_id(&mut self, id: String) -> usize {
        self.terms_mapping
            .get_by_left(&id)
            .cloned()
            .unwrap_or_else(|| {
                let ident = self.terms_mapping.len();
                self.terms_mapping.insert(id, ident);
                ident
            })
    }

    fn build_query_ref(
        &mut self,
        term: Term,
        builder: &mut QueryBuilder,
        variables: &mut HashMap<String, QueryRef>,
    ) -> QueryRef {
        match term {
            Term::Var(v) => *variables
                .entry(v)
                .or_insert_with(|| builder.variable()),
            Term::Const(id) => {
                let id = self.get_id(id);
                builder.constant(id)
            }
            Term::Struct(id, st) => {
                let id = self.get_id(id);
                let subterms: Vec<_> = st
                    .into_iter()
                    .map(|st| self.build_query_ref(st, builder, variables))
                    .collect();
                builder.structure(id, subterms.into_iter())
            }
        }
    }

    pub fn build_query(&mut self, term: Term) ->
        (Query, HashMap<String, QueryRef>)
    {
        let mut builder = Default::default();
        let mut variables = Default::default();
        let term = self.build_query_ref(term, &mut builder, &mut variables);

        (builder.build(term), variables)
    }

    fn build_fact_ref(
        &mut self,
        term: Term,
        builder: &mut StatementBuilder,
        variables: &mut HashMap<String, StatementRef>,
    ) -> StatementRef {
        match term {
            Term::Var(v) => *variables
                .entry(v)
                .or_insert_with(|| builder.variable()),
            Term::Const(id) => {
                let id = self.get_id(id);
                builder.constant(id)
            },
            Term::Struct(id, st) => {
                let id = self.get_id(id);
                let subterms: Vec<_> = st
                    .into_iter()
                    .map(|st| self.build_fact_ref(st, builder, variables))
                    .collect();
                builder.structure(id, subterms.into_iter())
            }
        }
    }

    pub fn build_fact(&mut self, term: Term) -> Statement
    {
        let mut builder = Default::default();
        let term = self.build_fact_ref(
            term,
            &mut builder,
            &mut Default::default()
        );

        builder.build(term)
    }
}

impl TermBuilder for Context {
    type Term = Term;

    fn variable(&mut self, id: usize) -> Term {
        Term::Var(format!("{}", id))
    }

    fn structure(&mut self, ident: usize, subterms: impl Iterator<Item = Term>) -> Term {
        let id = self
            .terms_mapping
            .get_by_right(&ident)
            .cloned()
            .unwrap_or_else(|| format!("_{}", ident));
        Term::Struct(id, subterms.collect())
    }

    fn constant(&mut self, ident: usize) -> Term {
        let id = self
            .terms_mapping
            .get_by_right(&ident)
            .cloned()
            .unwrap_or_else(|| format!("_{}", ident));
        Term::Const(id)
    }
}
