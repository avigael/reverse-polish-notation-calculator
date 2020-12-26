/*
 * Reverse Polish Notation: parser.rs
 */

extern crate rand;

use std::io::{self, Write};

use super::rpn;

pub fn rpn_repl() -> rpn::Result<()> {
    let mut stack = rpn::Stack::new();
    let mut input = String::new();

    // Read-eval-print loop
    loop {
        // Clear the input buffer
        input.clear();

        // Prompt the user
        print!("> ");
        io::stdout().flush().map_err(rpn::Error::IO)?;

        // Read a line and evaluate it
        io::stdin().read_line(&mut input).map_err(rpn::Error::IO)?;
        evaluate_line(&mut stack, &input)?;

        // A successful run should end with a stack with a exactly one item: the result
        let res = stack.pop()?;
        if stack.empty() {
            println!("Reply> {:?}", res);
        } else {
            return Err(rpn::Error::Extra);
        }
    }
}

fn evaluate_line(stack: &mut rpn::Stack, buf: &String) -> rpn::Result<()> {
    // Trim whitespace and split; this gives an iterator of tokens.
    let tokens = buf.trim().split_whitespace();

    // Main loop processing the tokens.
    for token in tokens {
        match token {
            "true" => stack.push(rpn::Item::Bool(true))?,
            "false" => stack.push(rpn::Item::Bool(false))?,
            "+" => stack.eval(rpn::Op::Add)?,
            "=" => stack.eval(rpn::Op::Eq)?,
            "~" => stack.eval(rpn::Op::Neg)?,
            "<->" => stack.eval(rpn::Op::Swap)?,
            "#" => stack.eval(rpn::Op::Rand)?,
            "?" => stack.eval(rpn::Op::Cond)?,
            "quit" => stack.eval(rpn::Op::Quit)?,
            _ => {
                let ptok = token.parse::<i32>();
                match ptok {
                    Ok(res) => stack.push(rpn::Item::Int(res))?,
                    _ => return Err(rpn::Error::Syntax),
                }
            }
        };
    }
    Ok(())
}
