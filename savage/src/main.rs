// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2021  Philipp Emanuel Weidmann <pew@worldwidemann.com>

use savage_core::expression::Expression::*;
use savage_core::expression::RationalRepresentation::*;

fn main() {
    println!(
        "{:?}",
        Sum(
            Box::new(Integer(123.into())),
            Box::new(Rational((456.into(), 789.into()).into(), Fraction)),
        ),
    );
}
