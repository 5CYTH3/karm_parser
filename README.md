# Karm Language
A simple language based on expressions and with a pure functional twist.
In my journey of understanding how compilers works, there is KARM. It's only a project but it's kind of like my baby. Just a little language, with a grammar and a syntax that I made myself.

Talking about syntax, here is the BNF of the language:
```html
<expr> ::= (<fn> | <literal> | <binary>) ';'

<fn> ::= 'fn' <id> ['::' <id>] '->' <expr>

<binary> ::= (<literal> | <binary>) <op> (<literal> | <binary>)

<op> ::= '+' | '-' | '/' | '*'

<literal> ::= '+w/' | '+d/'
``` 
### Kind of a notepad
Just a reminder about the syntax I wanna give

```rust
fn addFive :: x -> x + 5;
fn sayHello -> puts "Hello";
```
tokens: Ident, "fn", ::, ->, +, -, /, *, ;.

With this example :
```rust
fn day -> 22 + 5;
```
We are supposed to get returned something like this:

```rust
Expr::Fn {
	id: "day"
	expr: Expr::Binary {
		op: Op::Plus,
		left: Expr::Literal("22"),
		right: Expr::Literal("5")
	}
}
```