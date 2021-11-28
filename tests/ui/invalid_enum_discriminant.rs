//! As of rust 1.56, enums without #[repr] cannot have discriminant larger than `isize`.
//! The implementation of #[pyclass] depends on this implementation detail.
//! This file tests if this implementation detail is still true.

use pyo3::prelude::*;

#[pyclass]
enum DiscriminantTooLarge{
    Var = 1 << 64,
}

#[pyclass]
enum DiscriminantOverflow{
    Var1 = isize::MAX,
    Overflow,
}

fn main() {}
