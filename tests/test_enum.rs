use pyo3::prelude::*;
use pyo3::{py_run, wrap_pyfunction};

mod common;

#[pyclass]
#[derive(Debug, PartialEq, Clone)]
pub enum MyEnum {
    Variant,
    OtherVariant,
}

#[test]
fn test_enum_class_attr() {
    Python::with_gil(|py| {
        let my_enum = py.get_type::<MyEnum>();
        let var = Py::new(py, MyEnum::Variant).unwrap();
        py_assert!(py, my_enum var, "my_enum.Variant == var");
    })
}

#[pyfunction]
fn return_enum() -> MyEnum {
    MyEnum::Variant
}

#[test]
fn test_return_enum() {
    let gil = Python::acquire_gil();
    let py = gil.python();
    let f = wrap_pyfunction!(return_enum)(py).unwrap();
    let mynum = py.get_type::<MyEnum>();

    py_run!(py, f mynum, "assert f() == mynum.Variant")
}

#[pyfunction]
fn enum_arg(e: MyEnum) {
    assert_eq!(MyEnum::OtherVariant, e)
}

#[test]
fn test_enum_arg() {
    Python::with_gil(|py| {
        let f = wrap_pyfunction!(enum_arg)(py).unwrap();
        let mynum = py.get_type::<MyEnum>();

        py_run!(py, f mynum, "f(mynum.OtherVariant)")
    })
}

#[test]
fn test_enum_eq() {
    Python::with_gil(|py| {
        let var1 = Py::new(py, MyEnum::Variant).unwrap();
        let var2 = Py::new(py, MyEnum::Variant).unwrap();
        let other_var = Py::new(py, MyEnum::OtherVariant).unwrap();
        py_assert!(py, var1 var2, "var1 == var2");
        py_assert!(py, var1 other_var, "var1 != other_var");
    })
}

#[test]
fn test_default_repr_correct() {
    Python::with_gil(|py| {
        let var1 = Py::new(py, MyEnum::Variant).unwrap();
        let var2 = Py::new(py, MyEnum::OtherVariant).unwrap();
        py_assert!(py, var1, "repr(var1) == 'MyEnum.Variant'");
        py_assert!(py, var2, "repr(var2) == 'MyEnum.OtherVariant'");
    })
}
