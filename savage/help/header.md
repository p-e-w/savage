# Savage Computer Algebra System

Savage is a new computer algebra system written from scratch in pure Rust. Its goals are correctness, simplicity, and usability, in that order.

This is Savage's documentation, which may be viewed at any time by entering `?` in the REPL (**r**ead-**e**val-**p**rint **l**oop, i.e. the Savage command interpreter). You can also directly view the documentation for a specific built-in function by entering `?` followed by the name of the function, e.g. `? det` for the determinant function.

For more information, visit **https://github.com/p-e-w/savage**. Note that Savage is in early development. If you encounter bugs or other problems, please don't hesitate to file an issue.


## Basic usage

Type mathematical expressions in the REPL using standard notation and press *Enter* to evaluate them:

```
in: 1 + 1
out: 2
```

Savage supports integer (`123`), fractional (`1/2`), decimal (`1.23`), and complex (`1 + 2*i`) number literals, as well as the sum (`+`), difference (`-`), product (`*`), quotient (`/`), remainder (`%`), and power (`^`) binary operators, and the negation (`-`) prefix operator:

```
in: 6/5 * 3
out: 18/5

in: 1.2 * 3
out: 3.6

in: (1 + i) ^ 12
out: -64
```

It also supports boolean (`true`/`false`) literals, and the conjunction ("and", `&&`), disjunction ("or", `||`), and logical negation ("not", `!`) operators. The standard comparison operators (`==`, `!=`, `<`, `<=`, `>`, `>=`) are available as well:

```
in: a && true
out: a

in: a || true
out: true

in: a || !a
out: true

in: a < a
out: false
```


## Vectors and matrices

A comma-separated list of expressions surrounded by square brackets (e.g. `[1, 2, 3]`) represents a column vector with the expressions as elements. A vector of vectors, all of which have the same size (e.g. `[[1, 2], [3, 4]]`), is interpreted as a column-major matrix whose rows are the constitutent vectors.

Vectors and matrices support the standard arithmetic operations:

```
in: [a, b] - [a, c]
out: [0, b - c]

in: [a, b, c] * 3
out: [a * 3, b * 3, c * 3]

in: [[1, 2], [3, 4]] * [5, 6]
out: [17, 39]
```

Individual elements of vectors and matrices can be accessed using the index notation familiar from many programming languages:

```
in: v = [1, 2, 3]
in: v[1]
out: 2

in: m = [[1, 2], [3, 4]]
in: m[1, 0]
out: 3
```


## Variables and functions

Symbolic names like `a`, `b`, and `c` are by default interpreted as expression placeholders. They can, however, be assigned specific values, at which point they become defined variables:

```
in: a = 1
in: b = 2
in: a + b
out: 3
```

Functions can also be defined with a syntax that is essentially identical to that used in standard mathematical notation:

```
in: sum(x, y) = x + y
in: sum(1, 2)
out: 3
```


## Built-in functions

The following named functions are available in the REPL without needing to be explicitly loaded or manually defined:
