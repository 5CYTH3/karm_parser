# Karm Language
A simple language inspired by StandardML.
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
> [!CAUTION]
> This section, especially the grammar, is subject to changes.
Here is the BNF of the language's grammar :
```ebnf
program = { expr };

expr = let | call | if-expr | question | '(' expr ')';

let = 'let' id '=' ( type | fun ) [ 'in' expr ];

type = 'type' ( product | sum );

fun = [ 'infix' ] 'fun' [{ id [ ',' id ] }] '|->' expr;

call = id [ '(' { id ',' } ')' ];

if-expr = 'if' expr 'then' expr 'else' expr;

question = '?' expr;

term = id | literal;

literal = numbers | strings | booleans | chars;
``` 
## Examples

### Hello World!
```ocaml
let main = print("Hello, World");
```
(The `print` function will be part of the standard library)

### Fibonacci
Basic implementation of the fibonacci sequence in Karm :
```ocaml
let fib n = if n <= 1 then n else fib(n - 1) + fib(n - 2);
```

### Usage for question exprs
```ocaml
let show x =
    if (? x: Str) then print(x)
    else if (? x: Int) then printf("{i}", x)
    else if (? x: Bool) then printf("{b}", x);
```

