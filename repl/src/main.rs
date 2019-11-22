use rustyline::{error::ReadlineError, Editor};

use warren::Machine;

mod ast;
mod context;
mod parser;

use context::Context;

fn handle_query(
    query: ast::Term,
    ctx: &mut Context,
    machine: &mut Machine
) {
    let (query, variables) = ctx.build_query(query);
    let query_result = machine.query(query, &Default::default());

    for (var, qref) in variables {
        if let Some(unification) = query_result.build_term(qref, ctx) {
            println!("{} := {:?}", var, unification);
        } else {
            println!("Invalid unification for {}", var);
        }
    }
}

fn handle_fact(
    fact: ast::Term,
    ctx: &mut Context,
    _machine: &mut Machine
) {
    let _fact = ctx.build_fact(fact);
    unimplemented!()
}

fn handle_stmt(
    stmt: ast::Statement,
    ctx: &mut Context,
    machine: &mut Machine
) {
    match stmt {
        ast::Statement::Query(q) => handle_query(q, ctx, machine),
        ast::Statement::Fact(f) => handle_fact(f, ctx, machine),
    }
}

fn handle_assembly(
    stmt: ast::Statement,
    ctx: &mut Context,
) {
    let asm = match stmt {
        ast::Statement::Query(q) =>
            ctx.build_query(q).0.assembly(),
        ast::Statement::Fact(f) =>
            ctx.build_fact(f).assembly(),
    };

    println!("{}", asm);
}

fn handle_directive(
    d: Option<ast::Directive>,
    ctx: &mut Context,
    machine: &mut Machine
) {
    let d = if let Some(d) = d {
        d
    } else {
        println!("Invalid statement");
        return;
    };

    match d {
        ast::Directive::Statement(s) => handle_stmt(s, ctx, machine),
        ast::Directive::Assembly(s) => handle_assembly(s, ctx),
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
                handle_directive(ast.ok(), &mut context, &mut machine);
            }
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => break,
            Err(err) => {
                println!("Error: {:?}", err);
            }
        }
    }

    rl.save_history("history").ok();
}
