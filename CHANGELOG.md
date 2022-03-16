# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).


## [Unreleased]

### Added

#### REPL

- Proper formatting for parse errors

### Changed

### Fixed


## [0.2.0] - 2022-03-13

### Added

#### Core

- Basic support for vectors and matrices
- Algebraic simplification during evaluation
- Infrastructure for evaluating functions
- Macro-based function dispatch system
- New built-in functions:
  - `and`
  - `det`
  - `factorial`
  - `is_prime`
  - `nth_prime`
  - `prime_pi`

#### REPL

- Persistent input history
- Multi-line editing mode
- Syntax highlighting for input and output
- Highlighting of matching brackets in input

### Fixed

#### Core

- Type inflation in parser
- Exponential blowup in parser

#### REPL

- Error on empty input


## [0.1.0] - 2021-11-28

Initial release with basic REPL and support for elementary arithmetic and logic.


[unreleased]: https://github.com/p-e-w/savage/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/p-e-w/savage/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/p-e-w/savage/releases/tag/v0.1.0
