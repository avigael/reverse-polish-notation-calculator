/*
 * Reverse Polish Notation: rpn.rs
 */

use rand::prelude::*;
use std::io;

// Stacks will work with Items, which either either integers or booleans
#[derive(Eq, PartialEq, Ord, PartialOrd, Debug)]
pub enum Item {
    Int(i32),
    Bool(bool),
}

// List of possible errors
#[derive(Debug)]
pub enum Error {
    Empty,         // Tried to pop empty stack
    Extra,         // Stack ended with extra elements
    Type,          // Type mismatch
    Syntax,        // Syntax error, didn't recognize op
    IO(io::Error), // Some kind of IO error
    Quit,          // User quitting
}

// Base operations supported by calculator, see rpn.md
#[derive(Debug)]
pub enum Op {
    Add,
    Eq,
    Neg,
    Swap,
    Rand,
    Cond,
    Quit,
}

// A result type for our calculator: either a valid value, or a calculator Error
pub type Result<T> = std::result::Result<T, Error>;

// A type for Stacks
#[derive(Debug)]
pub struct Stack(Vec<Item>);

impl Stack {
    // Make a new Stack
    pub fn new() -> Self {
        Stack(vec![])
    }

    // Check if a Stack is empty
    pub fn empty(&self) -> bool {
        self.0.is_empty()
    }

    // Push an item onto a stack (should never error)
    pub fn push(&mut self, item: Item) -> Result<()> {
        Ok(self.0.push(item))
    }

    // Pop an item off the Stack; may result in Empty error
    pub fn pop(&mut self) -> Result<Item> {
        let res = self.0.pop();
        match res {
            None => Err(Error::Empty),
            Some(thing) => Ok(thing),
        }
    }

    // Main evaluation function: apply an operation to a Stack
    pub fn eval(&mut self, op: Op) -> Result<()> {
        match op {
            Op::Add => {
                let _x = Stack::pop(self)?;
                let _y = Stack::pop(self)?;
                match (_x, _y) {
                    (Item::Int(x), Item::Int(y)) => self.push(Item::Int(x + y)),
                    _ => Err(Error::Type),
                }
            }
            Op::Eq => {
                let _x = Stack::pop(self)?;
                let _y = Stack::pop(self)?;
                match (_x, _y) {
                    (Item::Int(x), Item::Int(y)) => self.push(Item::Bool(x == y)),
                    (Item::Bool(x), Item::Bool(y)) => self.push(Item::Bool(x == y)),
                    _ => Err(Error::Type),
                }
            }
            Op::Neg => {
                let _x = Stack::pop(self)?;
                match _x {
                    Item::Bool(x) => self.push(Item::Bool(!x)),
                    _ => Err(Error::Type),
                }
            }
            Op::Swap => {
                let _x = Stack::pop(self)?;
                let _y = Stack::pop(self)?;
                self.push(_x)?;
                self.push(_y)
            }
            Op::Rand => {
                let _x = Stack::pop(self)?;
                match _x {
                    Item::Int(x) => {
                        let num: i32 = thread_rng().gen_range(0, x);
                        self.push(Item::Int(num))
                    }
                    _ => Err(Error::Type),
                }
            }
            Op::Cond => {
                let _x = Stack::pop(self)?;
                let _y = Stack::pop(self)?;
                let _z = Stack::pop(self)?;
                match (_x, _y, _z) {
                    (Item::Int(x), Item::Int(y), Item::Bool(z)) => {
                        if z {
                            self.push(Item::Int(y))
                        } else {
                            self.push(Item::Int(x))
                        }
                    }
                    _ => Err(Error::Type),
                }
            }
            Op::Quit => Err(Error::Quit),
        }
    }
}
