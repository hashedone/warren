use rustyline::{error::ReadlineError, Editor};

use warren::Machine;

mod ast;
mod context;
mod parser;

use context::Context;

fn handle_query(query: ast::Term, ctx: &mut Context, machine: &mut Machine) {
    let (query, variables) = ctx.build_query(query);
    let query_result = machine.query(query);

    for (var, qref) in variables {
        if let Some(unification) = query_result.build_term(qref, ctx) {
            println!("{} := {:?}", var, unification);
        } else {
            println!("Invalid unification for {}", var);
        }
    }
}

fn handle_stmt(stmt: Option<ast::Statement>, ctx: &mut Context, machine: &mut Machine) {
    let stmt = if let Some(stmt) = stmt {
        stmt
    } else {
        println!("Invalid statement");
        return;
    };

    match stmt {
        ast::Statement::Query(q) => handle_query(q, ctx, machine),
    }
}

fn main() {
    let mut rl = Editor::<()>::new();
    let mut context = Context::default();
    let mut machine = Machine::new();

    rl.load_history("history").ok();

    loop {
        match rl.readline("> ") {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                let ast = parser::parse(line.as_str());
                handle_stmt(ast.ok(), &mut context, &mut machine);
            }
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => break,
            Err(err) => {
                println!("Error: {:?}", err);
            }
        }
    }

    rl.save_history("history").ok();
}
