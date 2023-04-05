// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2021-2022  Philipp Emanuel Weidmann <pew@worldwidemann.com>

use permutohedron::heap_recursive;
use savage_macros::function;

use crate::{expression::Expression, functions::Matrix, functions::SquareMatrix, helpers::*};

#[function(
    name = "det",
    description = "determinant of a square matrix",
    examples = r#"[
        ("det([[1, 2], [3, 4]])", "-2"),
        ("det([[a, b], [c, d]])", "a * d - b * c"),
        ("det([])", "1"),
    ]"#,
    categories = r#"[
        "linear algebra",
    ]"#
)]
fn determinant(matrix: SquareMatrix) -> Expression {
    if matrix.is_empty() {
        return int(1);
    }

    let mut indices = (0..matrix.nrows()).collect::<Vec<usize>>();

    let mut products = Vec::new();

    heap_recursive(indices.as_mut_slice(), |permutation| {
        products.push(
            (0..matrix.nrows())
                .map(|i| matrix[(i, permutation[i])].clone())
                .reduce(|a, b| a * b)
                .unwrap(),
        );
    });

    // The first permutation generated by Heap's algorithm is the identity,
    // which has positive sign...
    let mut positive = true;

    products
        .into_iter()
        .reduce(|a, b| {
            // ... and every following permutation differs from its predecessor
            // by exactly one transposition, which flips the sign.
            positive = !positive;

            if positive {
                a + b
            } else {
                a - b
            }
        })
        .unwrap()
}

#[function(
    name = "transpose",
    description = "transpose of a matrix",
    examples = r#"[
        ("transpose([[1, 2], [3, 4]])", "[[1, 3], [2, 4]]"),
    ]"#,
    categories = r#"[
        "linear algebra",
    ]"#
)]
fn transpose(matrix: Matrix) -> Expression {
    Expression::Matrix(matrix.transpose())
}

#[function(
    name = "trace",
    description = "trace of a square matrix",
    examples = r#"[
        ("trace([[1, 2], [3, a]])", "1 + a"),
    ]"#,
    categories = r#"[
        "linear algebra",
    ]"#
)]
fn trace(matrix: SquareMatrix) -> Expression {
    matrix.diagonal().iter().fold(int(0), |acc, e| {
        acc + e.clone()
    })
}

#[function(
    name = "nullspace",
    description = "nullspace of a matrix",
    examples = r#"[]"#,
    categories = r#"[
        "linear algebra",
    ]"#
)]
fn nullspace(_matrix: Matrix) -> Expression {
    unimplemented!()
}

#[function(
    name = "eigenvals",
    description = "eigen values of a square matrix",
    examples = r#"[]"#,
    categories = r#"[
        "linear algebra",
    ]"#
)]
fn eigenvalues(_matrix: SquareMatrix) -> Expression {
    unimplemented!()
}

#[function(
    name = "eigenvecs",
    description = "eigen vectors of a square matrix",
    examples = r#"[]"#,
    categories = r#"[
        "linear algebra",
    ]"#
)]
fn eigenvectors(_matrix: SquareMatrix) -> Expression {
    unimplemented!()
}
