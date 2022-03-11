# Savage Computer Algebra System

Savage is a new computer algebra system written from scratch in pure Rust.
Its goals are correctness, simplicity, and usability, in that order.
The entire system compiles to a single, dependency-free executable just
2.5 MB in size. While that executable will of course grow as Savage matures,
the plan is to eventually deliver a useful computer algebra system in 5 MB
or less.

The name "Savage" is a reference/homage to [Sage](https://www.sagemath.org/),
the leading open-source computer algebra system. Since Sage already exists
and works very well, it would make no sense to attempt to create a clone of it.
Instead, Savage aims to be something of an antithesis to Sage: Where Sage is
a unified frontend to dozens of mathematics packages, Savage is a tightly-integrated,
monolithic system. Where Sage covers many areas of mathematics, including cutting-edge
research topics, Savage will focus on the "bread and butter" math employed by
engineers and other people who *use*, rather than develop, mathematical concepts.
Where Sage features amazingly sophisticated implementations of countless functions,
Savage has code that is savagely primitive, getting the job done naively but correctly,
without worrying whether the performance is still optimal when the input is
a million-digit number.

**Savage is in early development and is not yet ready to be used for serious work.**
It is, however, ready to play around with, and is happily accepting contributions
to move the project forward.


## Features

This is what Savage offers **today:**

* Arbitrary-precision integer, rational, and complex arithmetic
* Input, simplification, and evaluation of symbolic expressions
* First-class support for vectors and matrices, with coefficients being arbitrary expressions
* REPL with syntax and bracket highlighting, persistent history, and automatic multi-line input
* Macro-based system for defining functions with metadata and automatic type checking
* [Usable as a library](#savage-as-a-library) from any Rust program

The following features are **planned,** with some of the groundwork already done:

* User-defined variables and functions
* Built-in help system
* Many more functions from various areas of math
* More powerful expression simplification
* Jupyter kernel

By contrast, the following are considered **non-features** for Savage,
and there are no plans to add them either now or in the future:

* *Advanced/research-level mathematics:* As a rule of thumb, if it doesn't belong
  in a typical undergraduate course, it probably doesn't belong in Savage.
* *Physics/finance/machine learning/other areas adjacent to math:* The scope would
  grow without bounds and that is exactly what Savage aims to avoid.
* *Formal verification of implementations:* The required technologies aren't mature yet
  and Savage is not a research project.
* *Performance at the expense of simplicity:* Yes, I know that multiplication
  can be done faster using some fancy Fourier tricks. No, I won't implement that.
* *General-purpose programming:* Too complex, and not the focus of this project.
* *File/network I/O:* Savage performs computations, nothing more and nothing less.
  Functions have no side effects.
* *Modules/packages/extensions/plugins:* The world is complicated enough.
  Either something is built in, or Savage doesn't have it at all.
* *GUI:* Although it's possible to create a GUI frontend backed by the `savage_core`
  crate, there are no plans to do so within the Savage project itself.


## Installation

Building Savage from source requires [Rust](https://www.rust-lang.org/) **1.56 or later.**
Once a supported version of Rust is installed on your system, you only need to run

```
cargo install savage
```

to install the Savage REPL to your Cargo binary directory (usually `$HOME/.cargo/bin`).
Of course, you can also just clone this repository and `cargo run` the REPL from the
repository root.

In the future, there will be pre-built executables for major platforms
available with every Savage release.


## Tour

### Arithmetic

Arithmetic operations in Savage have no precision limits (other than the amount
of memory available in your system):

```
in: 1 + 1
out: 2

in: 1.1 ^ 100
out: 13780.612339822270184118337172089636776264331200038466433146477552154985209
5523076769401159497458526446001

in: 3 ^ 4 ^ 5
out: 373391848741020043532959754184866588225409776783734007750636931722079040617
26525122999368893880397722046876506543147515810872705459216085858135133698280918
73141917485942625809388070199519564042855718180410466812887974029255176680123406
17298396574731619152386723046235125934896058590588284654793540505936202376547807
44273058214452705898875625145281779341335214192074462302751872918543286237573706
39854853194764169262638199728870069070138992565242971985276987492741962768110607
02333710356481
```

Results are automatically printed in either fractional or decimal form,
depending on whether the input contained fractions or decimal numbers:

```
in: 6/5 * 3
out: 18/5

in: 1.2 * 3
out: 3.6
```

The variable `i` is predefined to represent the imaginary unit, allowing for
complex numbers to be entered using standard notation:

```
in: (1 + i) ^ 12
out: -64
```

### Linear algebra

Vectors and matrices are first-class citizens in Savage and support the standard
addition, subtraction, multiplication, and exponentiation operators. Coefficients
can be arbitrary expressions:

```
in: [a, b] - [a, c]
out: [0, b - c]

in: [a, b, c] * 3
out: [a * 3, b * 3, c * 3]

in: [[1, 2], [3, 4]] * [5, 6]
out: [17, 39]
```

Determinants are evaluated symbolically:

```
in: det([[a, 2], [3, a]])
out: a ^ 2 - 6
```

### Logic

The standard `&&`, `||`, `!`, and comparison operators are available. Savage
automatically evaluates many tautologies and contradictions, even in the presence
of undefined variables:

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

### Number theory

Verify that the Mersenne number *M<sub>31</sub>* is a prime number:

```
in: is_prime(2^31 - 1)
out: true
```

Compute the ten millionth prime number:

```
in: nth_prime(10^7)
out: 179424673
```

Compute the number of primes up to ten million:

```
in: prime_pi(10^7)
out: 664579
```

These functions for dealing with prime numbers are powered by the ultra-fast
[`primal`](https://crates.io/crates/primal) crate. Many more functions from
number theory will be added to Savage in the future.


## Savage as a library

All of Savage's actual computer algebra functionality is contained in the
[`savage_core`](https://crates.io/crates/savage_core) crate. That crate exposes
everything necessary to build software that leverages symbolic math capabilities.
Assuming `savage_core` has been added as a dependency to a crate's `Cargo.toml`,
it can be used like this:

```rust
use std::collections::HashMap;

use savage_core::{expression::Expression, helpers::*};

fn main() {
    // Expressions can be constructed by parsing a string literal...
    let lhs = "det([[a, 2], [3, a]])".parse::<Expression>().unwrap();
    // ... or directly from code using helper functions.
    let rhs = pow(var("a"), int(2)) - int(6);

    let mut context = HashMap::new();
    // The context can be used to set the values of variables during evaluation.
    // Change "b" to "a" to see this in action!
    context.insert("b".to_owned(), int(3));

    assert_eq!(lhs.evaluate(context), Ok(rhs));
}
```

Please note that at this point, the primary purpose of the `savage_core` crate is
to power the Savage REPL, so any use by third-party crates should be considered
somewhat experimental. Note also that like the rest of Savage, `savage_core` is
licensed under the terms of the [AGPL](LICENSE), which imposes conditions on any
dependent software that go beyond what is required by the more common permissive
licenses. Make sure you understand the AGPL and its implications before adding
`savage_core` as a dependency to your crate.


## Acknowledgments

Savage stands on the shoulders of the giant that is the Rust ecosystem. Among the
many third-party crates that Savage relies on, I want to highlight two that play
a particularly important role:

* [`num`](https://crates.io/crates/num) is the fundamental crate for all numeric
  computations in Savage. It provides the crucial `BigInt` type that enables
  standard arithmetic operations to be performed with arbitrary precision.
  `num`'s code is of high quality and extremely well tested.
* [`chumsky`](https://crates.io/crates/chumsky) is the magic behind Savage's expression
  parser. I have looked at every parser crate currently available and found Chumsky's
  API to be by far the most intuitive. Furthermore, Chumsky's author is highly
  responsive on the issue tracker, and has personally helped me understand and resolve
  two major issues that arose during the development of Savage's parser.


## License

Copyright &copy; 2021-2022  Philipp Emanuel Weidmann (<pew@worldwidemann.com>)

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU Affero General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.

**By contributing to this project, you agree to release your
contributions under the same license.**
