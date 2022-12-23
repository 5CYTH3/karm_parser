Just a reminder about the syntax I wanna give

```km
func main {
    print("Hello World!");
}
```

When a function has axioms, then you are forced to call it with parthesis. Although, if your function has no axioms, you can call it without.

```km
Axioms n :: Integer;
func fib {
    if n <= 1 {
        ret n;
    }
    ret fib(n-1) + fib(n-2);
}

-----

func no_param {
    ret 99;
}

func main {
    no_param;
}
```

There is an operator to deep copy and one to assign values. The deep-copy one (<=>) allows user to clone the data from the first variable to the second one, while the assignment operator (:=) will link the two objects, which mean that whenever a change is made on the first object, the second one will be updated.

```
func main {
    b :: Integer := 5;
    a :: Integer := b; // a = 5
    c :: Integer <=> b; // c = 5
    b := 87; // then a = 87 but c = 5
}
```
