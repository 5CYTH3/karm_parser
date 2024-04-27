# Roadmap of what needs to be done
All the steps of the Karm compilation :
- [x] Efficient lexer
- [x] Parser (maybe to refactor to use less ExprTypes)
- [ ] Typechecker
    - [ ] Hindley-Milner type system and type inference algorithm
    - [ ] Dependent Typing supports
- [ ] IR generation
- [ ] IR optimization
- [ ] Code generation

# Features
- [ ] Collections (arrays, lists, matrices)
- [ ] Algebraic data types (sum, product)
- [ ] Unary operators
- [ ] Infix and prefix functions
- [ ] Reverse application operator
- [ ] Pattern matching

# Concepts
I would really like to include some Homotopy Type Theory inside the project.

# By-subject ressources
### General
- [The Dragon Book](https://www.amazon.com/dp/0321486811)
- [Engineering A Compiler](https://www.amazon.com/dp/012088478X)
- [Programming Languages: Theory and Practice](http://people.cs.uchicago.edu/~blume/classes/aut2008/proglang/text/offline.pdf)
- [The Implementation of Functional Programming Languages](https://www.microsoft.com/en-us/research/wp-content/uploads/1987/01/slpj-book-1987-small.pdf)

### Parsing
- [Parsing Techniques: A Practical Guide](https://www.amazon.com/dp/038720248X)
    - [1st ed PDF](https://dickgrune.com/Books/PTAPG_1st_Edition/)

### Typechecker
- [Types and Programming Languages](https://www.amazon.com/dp/0262162091)

### Code Optimization
- [Building and Optimizing Compiler](https://www.amazon.com/dp/155558179X)
- [Optimizing Compilers for Modern Architecture](https://www.amazon.com/dp/1558602860/)
- [The Garbage Collection Handbook: The Art of Automatic Memory Management](https://www.amazon.com/dp/1420082795)

### Code generation
- [Linking/Loading](https://www.amazon.com/dp/1558604960)

BETE D'IDEE : ENFAIT POUR FAIRE LA REPR DE TYPES, JE PEUX JUSTE UTILISER UN COUPLE (Varname, TYPEJUDGEMENT)
