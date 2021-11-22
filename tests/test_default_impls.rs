#![allow(non_snake_case)]
use pyo3::class::PyMethodDefType;
use pyo3::prelude::*;
use pyo3::py_run;

mod common;

// Tests for PyClassDefaultSlots
#[pyclass]
struct TestDefaultSlots;

// generated using `Cargo expand`
// equivalent to
// ```
// impl TestDefaultSlots {{
//     fn __str__(&self) -> &'static str {
//         "default"
//     }
// }
// ```
impl TestDefaultSlots {
    fn __pyo3__str__(&self) -> &'static str {
        "default"
    }
}

impl ::pyo3::class::impl_::PyClassDefaultSlots<TestDefaultSlots>
    for ::pyo3::class::impl_::PyClassImplCollector<TestDefaultSlots>
{
    fn py_class_default_slots(self) -> &'static [::pyo3::ffi::PyType_Slot] {
        &[{
            unsafe extern "C" fn __wrap(
                _raw_slf: *mut ::pyo3::ffi::PyObject,
            ) -> *mut ::pyo3::ffi::PyObject {
                let _slf = _raw_slf;
                ::pyo3::callback::handle_panic(|_py| {
                    let _cell = _py
                        .from_borrowed_ptr::<::pyo3::PyAny>(_slf)
                        .downcast::<::pyo3::PyCell<TestDefaultSlots>>()?;
                    let _ref = _cell.try_borrow()?;
                    let _slf = &_ref;
                    ::pyo3::callback::convert(_py, TestDefaultSlots::__pyo3__str__(_slf))
                })
            }
            ::pyo3::ffi::PyType_Slot {
                slot: ::pyo3::ffi::Py_tp_str,
                pfunc: __wrap as ::pyo3::ffi::reprfunc as _,
            }
        }]
    }
}

#[test]
fn test_default_slot_exists() {
    Python::with_gil(|py| {
        let test_object = Py::new(py, TestDefaultSlots).unwrap();
        py_assert!(py, test_object, "str(test_object) == 'default'");
    })
}

#[pyclass]
struct OverrideSlot;

// generated using `Cargo expand`
// equivalent to
// ```
// impl OverrideMagicMethod {
//     fn __str__(&self) -> &'static str {
//         "default"
//     }
// }
// ```
impl OverrideSlot {
    fn __pyo3__str__(&self) -> &'static str {
        "default"
    }
}

impl ::pyo3::class::impl_::PyClassDefaultSlots<OverrideSlot>
    for ::pyo3::class::impl_::PyClassImplCollector<OverrideSlot>
{
    fn py_class_default_slots(self) -> &'static [::pyo3::ffi::PyType_Slot] {
        &[{
            unsafe extern "C" fn __wrap(
                _raw_slf: *mut ::pyo3::ffi::PyObject,
            ) -> *mut ::pyo3::ffi::PyObject {
                let _slf = _raw_slf;
                ::pyo3::callback::handle_panic(|_py| {
                    let _cell = _py
                        .from_borrowed_ptr::<::pyo3::PyAny>(_slf)
                        .downcast::<::pyo3::PyCell<OverrideSlot>>()?;
                    let _ref = _cell.try_borrow()?;
                    let _slf = &_ref;
                    ::pyo3::callback::convert(_py, OverrideSlot::__pyo3__str__(_slf))
                })
            }
            ::pyo3::ffi::PyType_Slot {
                slot: ::pyo3::ffi::Py_tp_str,
                pfunc: __wrap as ::pyo3::ffi::reprfunc as _,
            }
        }]
    }
}

#[pymethods]
impl OverrideSlot {
    fn __str__(&self) -> &str {
        "overriden"
    }
}

#[test]
fn test_override_slot() {
    Python::with_gil(|py| {
        let test_object = Py::new(py, OverrideSlot).unwrap();
        py_assert!(py, test_object, "str(test_object) == 'overriden'");
    })
}
