# Karm Language
A simple language based on expressions and with a pure functional twist.
In my journey of understanding how compilers works, there is KARM. It's only a project but it's kind of like my baby. Just a little language, with a grammar and a syntax that I made myself.

# Summary
- [How to contribute](#contribute)
    - [Requirements](#requirements)
    - [Git Workflow](#git-workflow)
- [Technical specificities](#technical-specificities)
    - [Examples](#examples)


# Contribute

## Requirements
- Running on linux (required by `termion >= 3.0.0`)
- `rustc >= 1.75.0`
- `cargo >= 1.75.0`

## Git workflow
To submit your code, create a PR on the `dev` branch, which will be reviewed later on and merged on main if the feature is stable and legitimate.

Please document all your code, especially the structs fields and the functions by providing their specification. You can get inspirations from the actual code.

# Technical specificities
Here is the BNF of the language's grammar :
```html
<program> ::= <expr>*

<expr> ::= <lam> ';'

<lam> ::= 'fn' <id> ['::' (<id> ',')*] '->' <content-expr>

<content-expr> ::= (<if> | <lam-call> | <term>)

<if> ::= 'if' <lam-call> '?' <content-expr> ':' <content-expr>

<term> ::= (<literal> | <lam-call> | <var>)

<literal> ::= ('+w/' | '+d/')

<lam-call> ::= <id> '(' [<content-expr>*] ')'

<var> ::= <id>
``` 
## Examples

### Hello World!
```rust
fn main -> puts("Hello, World");
```
(The `puts()` function will be part of the standard library)

### Fibonacci
Basic implementation of the fibonacci sequence in Karm :
```rust
fn fib :: n -> if n <= 1 ? n : fib(n - 1) + fib(n - 2);
```
