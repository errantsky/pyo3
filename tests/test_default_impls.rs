#![allow(non_snake_case)]
use pyo3::class::PyMethodDefType;
use pyo3::prelude::*;
use pyo3::py_run;

mod common;

// Tests for PyClassDefaultImpls
#[pyclass]
struct TestDefaultImpl;

// generated using `Cargo expand`
// equivalent to
// ```
// impl TestDefaultImpl {
//     #[staticmethod]
//     fn default() {}
//     #[staticmethod]
//     fn overriden() -> bool { false }
// }
// ```
impl TestDefaultImpl {
    fn __pyo3__default() {}
    fn __pyo3__overriden() -> bool {
        false
    }
}

impl pyo3::class::impl_::PyClassDefaultImpls<TestDefaultImpl>
    for pyo3::class::impl_::PyClassImplCollector<TestDefaultImpl>
{
    fn py_class_default_impls(self) -> &'static [PyMethodDefType] {
        static METHODS: &[::pyo3::class::methods::PyMethodDefType] = &[
            ::pyo3::class::PyMethodDefType::Static(
                ::pyo3::class::methods::PyMethodDef::noargs(
                    "default\u{0}",
                    ::pyo3::class::methods::PyCFunction({
                        unsafe extern "C" fn __wrap(
                            _slf: *mut ::pyo3::ffi::PyObject,
                            _args: *mut ::pyo3::ffi::PyObject,
                        ) -> *mut ::pyo3::ffi::PyObject {
                            ::pyo3::callback::handle_panic(|_py| {
                                ::pyo3::callback::convert(_py, TestDefaultImpl::__pyo3__default())
                            })
                        }
                        __wrap
                    }),
                    "\u{0}",
                )
                .flags(::pyo3::ffi::METH_STATIC),
            ),
            ::pyo3::class::PyMethodDefType::Static(
                ::pyo3::class::methods::PyMethodDef::noargs(
                    "overriden\u{0}",
                    ::pyo3::class::methods::PyCFunction({
                        unsafe extern "C" fn __wrap(
                            _slf: *mut ::pyo3::ffi::PyObject,
                            _args: *mut ::pyo3::ffi::PyObject,
                        ) -> *mut ::pyo3::ffi::PyObject {
                            ::pyo3::callback::handle_panic(|_py| {
                                ::pyo3::callback::convert(_py, TestDefaultImpl::__pyo3__overriden())
                            })
                        }
                        __wrap
                    }),
                    "\u{0}",
                )
                .flags(::pyo3::ffi::METH_STATIC),
            ),
        ];
        METHODS
    }
}

#[pymethods]
impl TestDefaultImpl {
    #[staticmethod]
    fn overriden() -> bool {
        true
    }
}

#[test]
fn test_default_impl_exists() {
    Python::with_gil(|py| {
        let test_object = Py::new(py, TestDefaultImpl).unwrap();
        py_run!(py, test_object, "test_object.default()");
    })
}

#[test]
fn test_default_impl_is_overriden() {
    Python::with_gil(|py| {
        let test_object = Py::new(py, TestDefaultImpl).unwrap();
        py_assert!(py, test_object, "test_object.overriden() == True");
    })
}

#[pyclass]
struct OverrideMagicMethod;

// generated using `Cargo expand`
// equivalent to
// ```
// impl OverrideMagicMethod {
//     fn __str__(&self) -> &'static str {
//         "default"
//     }
// }
// ```
impl OverrideMagicMethod {
    fn __pyo3__str__(&self) -> &'static str {
        "default"
    }
}

impl ::pyo3::class::impl_::PyClassDefaultSlots<OverrideMagicMethod>
    for ::pyo3::class::impl_::PyClassImplCollector<OverrideMagicMethod>
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
                        .downcast::<::pyo3::PyCell<OverrideMagicMethod>>()?;
                    let _ref = _cell.try_borrow()?;
                    let _slf = &_ref;
                    ::pyo3::callback::convert(_py, OverrideMagicMethod::__pyo3__str__(_slf))
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
impl OverrideMagicMethod {
    fn __str__(&self) -> &str {
        "overriden"
    }
}

#[test]
fn test_override_magic_method() {
    Python::with_gil(|py| {
        let test_object = Py::new(py, OverrideMagicMethod).unwrap();
        py_assert!(py, test_object, "str(test_object) == 'overriden'");
    })
}
