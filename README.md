# Karm Language
A simple language based on expressions and with a pure functional twist.
In my journey of understanding how compilers works, there is KARM. It's only a project but it's kind of like my baby. Just a little language, with a grammar and a syntax that I made myself.

Talking about syntax, here is the BNF of the language:
```html
<program> ::= <expr>*

<expr> ::= <fn> ';'

<fn> ::= 'fn' <id> ['::' (<id> ',')*] '->' <content_expr>

<content_expr> ::= (<if> | <binary> | <term>)

<if> ::= 'if' <binary> '?' <content_expr> ':' <content_expr>

<binary> ::= (<binary> | <term>) <op> ( <binary> | <term>)

<op> ::= '+' | '-' | '/' | '*'

<term> ::= (<literal> | <fn_call> | <var>)

<literal> ::= ('+w/' | '+d/')

<fn_call> ::= <id> '(' [<content_expr>*] ')'

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