# Reverse Polish Notation Calculator

This project is written in Rust. To run this project please [install](https://www.rust-lang.org/tools/install "install") Rust on your machine.

This is a small [Reverse Polish Notation](https://en.wikipedia.org/wiki/Reverse_Polish_notation "Reverse Polish Notation") calculator written in Rust.

The rough idea is that a RPN calculator is a *stack language*: we push things onto a stack as we parse them, and when we parse operations we pop off the required number of arguments, apply the operation, and push the result back on to the stack.

## Inputs

* Add (`+`): Add two numbers together. Sample input: `3 5 +` should lead to 8.
* Eq (`=`): Check if two numbers or two booleans are equal. Sample input: `3 5
  =` should lead to false.
* Neg (`~`): Negate a boolean. Sample input: `false ~` should lead to true.
* Swap (`<->`): Swap the top two elements of the stack. Sample input: `0 1 <->
  [...]` should lead to `1 0 [...]`.
* Rand (`#`): Produce a random integer from 0 to the top element of the stack
  (which must be non-negative).  Sample input: `5 # [...]` should lead to a
  uniform integer in {0, 1, 2, 3, 4}, followed by `[...]`.
* Cond (`?`): If-then-else. Looks at the top three elements of the stack, and
  does an if-then-else. Sample input: `true 1 2 ?` should lead to 1, and `false
  1 2 ?` should lead to 2.
* Quit (`quit`): Quit the calculator. (You can just return `Error::Quit`.)

**Compilation instructions**: To run this file with cargo, execute the following command:
```
$ cargo run
```
