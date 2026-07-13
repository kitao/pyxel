use std::ops::{Deref, DerefMut};
use std::sync::MutexGuard;

use pyo3::prelude::*;

pub(crate) struct MutexFieldMut<'a, T, U> {
    owner: MutexGuard<'a, T>,
    field_ref: fn(&T) -> &U,
    field_mut: fn(&mut T) -> &mut U,
}

impl<'a, T, U> MutexFieldMut<'a, T, U> {
    pub(crate) fn new(
        owner: MutexGuard<'a, T>,
        field_ref: fn(&T) -> &U,
        field_mut: fn(&mut T) -> &mut U,
    ) -> Self {
        Self {
            owner,
            field_ref,
            field_mut,
        }
    }
}

impl<T, U> Deref for MutexFieldMut<'_, T, U> {
    type Target = U;

    fn deref(&self) -> &Self::Target {
        (self.field_ref)(&self.owner)
    }
}

impl<T, U> DerefMut for MutexFieldMut<'_, T, U> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        (self.field_mut)(&mut self.owner)
    }
}

// Rc helpers

macro_rules! rc_ref {
    ($rc:expr) => {
        ($rc).borrow()
    };
}

macro_rules! rc_mut {
    ($rc:expr) => {
        ($rc).borrow_mut()
    };
}

macro_rules! audio_ref {
    ($audio:expr) => {
        ($audio)
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    };
}

macro_rules! audio_mut {
    ($audio:expr) => {
        ($audio)
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    };
}

// Error helpers

macro_rules! deprecation_warning {
    ($name:ident, $msg:expr) => {
        static $name: std::sync::Once = std::sync::Once::new();
        $name.call_once(|| println!($msg));
    };
}

macro_rules! invalid_index_error {
    ($parameter:literal, $resource:literal) => {
        pyo3::exceptions::PyValueError::new_err(concat!(
            $parameter,
            " must be a valid ",
            $resource,
            " index"
        ))
    };
    ($parameter:literal, $resource:literal, list) => {
        pyo3::exceptions::PyValueError::new_err(concat!(
            $parameter,
            " must contain only valid ",
            $resource,
            " indices"
        ))
    };
}

macro_rules! validate_index {
    ($index:expr, $len:expr, $parameter:literal, $resource:literal) => {
        if ($index as usize) >= $len {
            return Err(invalid_index_error!($parameter, $resource));
        }
    };
    ($index:expr, $len:expr, $parameter:literal, $resource:literal, list) => {
        if ($index as usize) >= $len {
            return Err(invalid_index_error!($parameter, $resource, list));
        }
    };
}

macro_rules! python_type_error {
    ($msg:expr) => {
        return Err(pyo3::exceptions::PyTypeError::new_err($msg))
    };
}

// Type conversion

macro_rules! cast_pyany {
    ($value:ident, $expected:literal, $(($type:ty, $block:block)),*) => {
        {
            loop {
                $(
                    let any_ref: &pyo3::Bound<'_, pyo3::PyAny> = $value.as_any();
                    let borrowed: pyo3::Borrowed<'_, '_, pyo3::PyAny> = any_ref.into();
                    if let Ok($value) = <$type>::extract(borrowed) {
                        break $block;
                    }
                )*
                python_type_error!($expected);
            }
        }
    };
}

macro_rules! value_to_py_any {
    ($py:expr, $value:expr) => {
        $value.into_pyobject($py).unwrap().into()
    };
}

macro_rules! instance_to_py_any {
    ($py:expr, $instance:expr) => {{
        $instance.into_pyobject($py).unwrap().into_any().unbind()
    }};
}

pub(crate) fn ctypes_array_from_address(
    py: Python<'_>,
    element_type: &str,
    length: usize,
    address: usize,
) -> PyResult<Py<PyAny>> {
    let ctypes = py.import("ctypes")?;
    let array_type = ctypes
        .getattr(element_type)?
        .call_method1("__mul__", (length,))?;
    Ok(array_type
        .call_method1("from_address", (address,))?
        .unbind())
}

// Index / slice helpers

macro_rules! resolve_index {
    ($index:expr, $len:expr) => {{
        let index: isize = $index;
        let len: usize = $len;
        let resolved = if index < 0 {
            index + len as isize
        } else {
            index
        };
        if resolved < 0 || resolved as usize >= len {
            Err(pyo3::exceptions::PyIndexError::new_err(
                "list index out of range",
            ))
        } else {
            Ok(resolved as usize)
        }
    }};
}

pub(crate) struct SliceIndices {
    next: isize,
    step: isize,
    remaining: usize,
}

impl Iterator for SliceIndices {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }
        let index = self.next as usize;
        self.next += self.step;
        self.remaining -= 1;
        Some(index)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.remaining, Some(self.remaining))
    }
}

impl ExactSizeIterator for SliceIndices {}

pub(crate) fn slice_indices(start: isize, step: isize, len: usize) -> SliceIndices {
    SliceIndices {
        next: start,
        step,
        remaining: len,
    }
}

// Collect items into a PyList and return its iterator
macro_rules! items_to_pyiter {
    ($py:expr, $items:expr) => {{
        let list = pyo3::types::PyList::new($py, $items)?;
        Ok(list.call_method0("__iter__")?.unbind())
    }};
}

// Sequence impl blocks

// Read-only sequence methods: __len__, __getitem__ (with slicing + negative index),
// __iter__, __reversed__, __repr__, __bool__
macro_rules! impl_python_sequence_read {
    ($wrapper_name:ident, $inner_type:ty, $len:expr, $get_type:ty, $get:expr) => {
        #[pymethods]
        impl $wrapper_name {
            fn __len__(&self) -> usize {
                $len(&self.inner)
            }

            fn __getitem__<'py>(
                &self,
                py: Python<'py>,
                key: &Bound<'py, PyAny>,
            ) -> PyResult<Py<PyAny>> {
                use pyo3::types::PySlice;
                if let Ok(slice) = key.cast::<PySlice>() {
                    let len = $len(&self.inner);
                    let indices = slice.indices(len as isize)?;
                    let items = $crate::utils::slice_indices(
                        indices.start,
                        indices.step,
                        indices.slicelength,
                    )
                    .map(|i| $get(&self.inner, i));
                    let list = pyo3::types::PyList::new(py, items)?;
                    Ok(list.into_any().unbind())
                } else {
                    let idx: isize = key.extract()?;
                    let i = resolve_index!(idx, $len(&self.inner))?;
                    let value = $get(&self.inner, i);
                    let obj = pyo3::IntoPyObject::into_pyobject(value, py)
                        .map_err(Into::<PyErr>::into)?;
                    Ok(obj.into_any().unbind())
                }
            }

            fn __iter__(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
                let items = (0..$len(&self.inner)).map(|i| $get(&self.inner, i));
                items_to_pyiter!(py, items)
            }

            fn __reversed__(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
                let items = (0..$len(&self.inner)).rev().map(|i| $get(&self.inner, i));
                items_to_pyiter!(py, items)
            }

            fn __repr__(&self, py: Python) -> PyResult<String> {
                let len = $len(&self.inner);
                let items = (0..len).map(|i| $get(&self.inner, i));
                let list = pyo3::types::PyList::new(py, items)?;
                Ok(format!(
                    "{}{}",
                    stringify!($wrapper_name),
                    list.repr()?.to_string_lossy()
                ))
            }

            fn __bool__(&self) -> bool {
                $len(&self.inner) > 0
            }
        }
    };
}

// Comparison methods for primitive types: __contains__, __eq__, __add__, __mul__
macro_rules! impl_python_sequence_cmp {
    ($wrapper_name:ident, $inner_type:ty, $len:expr, $get_type:ty, $get:expr) => {
        #[pymethods]
        impl $wrapper_name {
            fn __contains__(&self, value: $get_type) -> bool {
                (0..$len(&self.inner)).any(|i| $get(&self.inner, i) == value)
            }

            fn __eq__<'py>(&self, _py: Python<'py>, other: &Bound<'py, PyAny>) -> PyResult<bool> {
                if let Ok(other_list) = other.extract::<Vec<$get_type>>() {
                    let len = $len(&self.inner);
                    if len != other_list.len() {
                        return Ok(false);
                    }
                    Ok((0..len).all(|i| $get(&self.inner, i) == other_list[i]))
                } else if let Ok(other_self) = other.extract::<$wrapper_name>() {
                    let len = $len(&self.inner);
                    if len != $len(&other_self.inner) {
                        return Ok(false);
                    }
                    Ok((0..len).all(|i| $get(&self.inner, i) == $get(&other_self.inner, i)))
                } else {
                    Ok(false)
                }
            }

            fn __add__<'py>(
                &self,
                py: Python<'py>,
                other: &Bound<'py, PyAny>,
            ) -> PyResult<Py<PyAny>> {
                let len = $len(&self.inner);
                let other_items: Vec<$get_type> = other.extract()?;
                let items = (0..len).map(|i| $get(&self.inner, i)).chain(other_items);
                let list = pyo3::types::PyList::new(py, items)?;
                Ok(list.into_any().unbind())
            }

            fn __mul__(&self, py: Python<'_>, n: isize) -> PyResult<Py<PyAny>> {
                let len = $len(&self.inner);
                let items = (0..len).map(|i| $get(&self.inner, i));
                let list = pyo3::types::PyList::new(py, items)?;
                Ok(list.call_method1("__mul__", (n,))?.unbind())
            }
        }
    };
}

// Mutable sequence methods: __setitem__, __delitem__, __iadd__,
// append, extend, insert, pop, clear
// Single-element mutations operate directly on the internal Vec via $list_mut
// (O(1) amortized) instead of copying the whole Vec through $to_list/$from_list.
// $to_raw / $from_raw adapt between the PyO3-facing type ($set_type / $get_type)
// and the storage type ($raw_item), e.g. Image wrapper <-> pyxel::RcImage.
macro_rules! impl_python_sequence_write {
    (
        $wrapper_name:ident, $inner_type:ty, $len:expr,
        $get_type:ty, $set_type:ty, $set:expr,
        $raw_item:ty, $list_mut:expr, $to_raw:expr, $from_raw:expr,
        $list_type:ty, $from_list:expr, $to_list:expr
    ) => {
        #[pymethods]
        impl $wrapper_name {
            fn __setitem__<'py>(
                &self,
                _py: Python<'py>,
                key: &Bound<'py, PyAny>,
                value: &Bound<'py, PyAny>,
            ) -> PyResult<()> {
                use pyo3::types::PySlice;
                if let Ok(slice) = key.cast::<PySlice>() {
                    let len = $len(&self.inner);
                    let indices = slice.indices(len as isize)?;
                    let new_values: Vec<$set_type> = value.extract()?;
                    if indices.step == 1 {
                        let start = indices.start as usize;
                        let end = indices.stop.max(indices.start) as usize;
                        let mut vec = $list_mut(&self.inner);
                        std::ops::DerefMut::deref_mut(&mut vec)
                            .splice(start..end, new_values.into_iter().map($to_raw));
                    } else {
                        if new_values.len() != indices.slicelength {
                            return Err(pyo3::exceptions::PyValueError::new_err(format!(
                                "attempt to assign sequence of size {} to extended slice of size {}",
                                new_values.len(),
                                indices.slicelength
                            )));
                        }
                        let positions = $crate::utils::slice_indices(
                            indices.start,
                            indices.step,
                            indices.slicelength,
                        );
                        for (pos, val) in positions.zip(new_values) {
                            $set(&self.inner, pos, val);
                        }
                    }
                    Ok(())
                } else {
                    let idx: isize = key.extract()?;
                    let i = resolve_index!(idx, $len(&self.inner))?;
                    let val: $set_type = value.extract()?;
                    $set(&self.inner, i, val);
                    Ok(())
                }
            }

            fn __delitem__<'py>(
                &self,
                _py: Python<'py>,
                key: &Bound<'py, PyAny>,
            ) -> PyResult<()> {
                use pyo3::types::PySlice;
                if let Ok(slice) = key.cast::<PySlice>() {
                    let len = $len(&self.inner);
                    let indices = slice.indices(len as isize)?;
                    let mut idx_list: Vec<usize> = $crate::utils::slice_indices(
                        indices.start,
                        indices.step,
                        indices.slicelength,
                    )
                    .collect();
                    // Remove from end to preserve earlier indices
                    idx_list.sort_unstable_by(|a, b| b.cmp(a));
                    let mut vec = $list_mut(&self.inner);
                    for i in idx_list {
                        std::ops::DerefMut::deref_mut(&mut vec).remove(i);
                    }
                    Ok(())
                } else {
                    let idx: isize = key.extract()?;
                    let i = resolve_index!(idx, $len(&self.inner))?;
                    $list_mut(&self.inner).remove(i);
                    Ok(())
                }
            }

            fn __iadd__(&self, values: Vec<$set_type>) {
                $list_mut(&self.inner).extend(values.into_iter().map($to_raw));
            }

            fn append(&self, value: $set_type) {
                $list_mut(&self.inner).push($to_raw(value));
            }

            fn extend(&self, values: Vec<$set_type>) {
                $list_mut(&self.inner).extend(values.into_iter().map($to_raw));
            }

            #[pyo3(signature = (index, value))]
            fn insert(&self, index: isize, value: $set_type) {
                let mut vec = $list_mut(&self.inner);
                let len = vec.len();
                let i = if index < 0 {
                    let resolved = index + len as isize;
                    if resolved < 0 { 0 } else { resolved as usize }
                } else if index as usize > len {
                    len
                } else {
                    index as usize
                };
                std::ops::DerefMut::deref_mut(&mut vec).insert(i, $to_raw(value));
            }

            #[pyo3(signature = (index=None))]
            fn pop(&self, index: Option<isize>) -> PyResult<$get_type> {
                let mut vec = $list_mut(&self.inner);
                let len = vec.len();
                if len == 0 {
                    return Err(pyo3::exceptions::PyIndexError::new_err(
                        "pop from empty list",
                    ));
                }
                let idx = index.unwrap_or(-1);
                let i = resolve_index!(idx, len)?;
                let raw: $raw_item = std::ops::DerefMut::deref_mut(&mut vec).remove(i);
                Ok($from_raw(raw))
            }

            fn clear(&self) {
                $list_mut(&self.inner).clear();
            }

            fn from_list(&self, vec: $list_type) -> PyResult<()> {
                deprecation_warning!(
                    FROM_LIST_ONCE,
                    concat!(stringify!($wrapper_name), ".from_list() is deprecated. Use slice assignment instead.")
                );
                $from_list(&self.inner, vec);
                Ok(())
            }

            fn to_list(&self, py: Python) -> PyResult<Py<PyAny>> {
                deprecation_warning!(
                    TO_LIST_ONCE,
                    concat!(stringify!($wrapper_name), ".to_list() is deprecated. Use list(seq) instead.")
                );
                let vec = $to_list(&self.inner);
                let list = pyo3::types::PyList::new(py, vec)?;
                Ok(list.unbind().into_any().into())
            }
        }
    };
}

// Sequence wrappers

// Wrapper for primitive-type sequences with comparison ops.
// Primitive case: internal Vec holds $set_type directly, so raw conversions are identities.
macro_rules! wrap_as_python_primitive_sequence {
    (
        $wrapper_name:ident, $inner_type:ty, $len:expr,
        $get_type:ty, $get:expr,
        $set_type:ty, $set:expr,
        $list_mut:expr,
        $list_type:ty, $from_list:expr, $to_list:expr
    ) => {
        #[pyclass(sequence, unsendable, from_py_object)]
        #[derive(Clone)]
        pub struct $wrapper_name {
            inner: $inner_type,
        }

        impl $wrapper_name {
            pub const fn wrap(inner: $inner_type) -> Self {
                Self { inner }
            }
        }

        impl_python_sequence_read!($wrapper_name, $inner_type, $len, $get_type, $get);
        impl_python_sequence_cmp!($wrapper_name, $inner_type, $len, $get_type, $get);
        impl_python_sequence_write!(
            $wrapper_name,
            $inner_type,
            $len,
            $get_type,
            $set_type,
            $set,
            $set_type,
            $list_mut,
            (|v: $set_type| v),
            (|v: $set_type| v),
            $list_type,
            $from_list,
            $to_list
        );
    };
}

// Wrapper for object/wrapper-type sequences (no Copy/PartialEq).
// Object case: internal Vec holds $raw_item (e.g. pyxel::RcImage) while PyO3
// sees wrapper $set_type. $to_raw / $from_raw bridge the two.
macro_rules! wrap_as_python_object_sequence {
    (
        $wrapper_name:ident, $inner_type:ty, $len:expr,
        $get_type:ty, $get:expr,
        $set_type:ty, $set:expr,
        $raw_item:ty, $list_mut:expr, $to_raw:expr, $from_raw:expr,
        $list_type:ty, $from_list:expr, $to_list:expr
    ) => {
        #[pyclass(sequence, unsendable, skip_from_py_object)]
        #[derive(Clone)]
        pub struct $wrapper_name {
            inner: $inner_type,
        }

        impl $wrapper_name {
            pub const fn wrap(inner: $inner_type) -> Self {
                Self { inner }
            }
        }

        impl_python_sequence_read!($wrapper_name, $inner_type, $len, $get_type, $get);
        impl_python_sequence_write!(
            $wrapper_name,
            $inner_type,
            $len,
            $get_type,
            $set_type,
            $set,
            $raw_item,
            $list_mut,
            $to_raw,
            $from_raw,
            $list_type,
            $from_list,
            $to_list
        );
    };
}

// Class wrapper

macro_rules! define_wrapper {
    ($wrapper_name:ident, $inner_type:ty) => {
        #[pyclass(unsendable, from_py_object)]
        #[derive(Clone)]
        pub struct $wrapper_name {
            pub(crate) inner: std::rc::Rc<std::cell::RefCell<$inner_type>>,
        }

        impl $wrapper_name {
            pub fn wrap(inner: std::rc::Rc<std::cell::RefCell<$inner_type>>) -> Self {
                Self { inner }
            }

            // Some wrappers need only one accessor, but keeping both keeps the macro uniform.
            #[allow(dead_code)]
            pub(crate) fn inner_ref(&self) -> std::cell::Ref<'_, $inner_type> {
                rc_ref!(self.inner)
            }

            // Python methods mutate shared engine resources through PyO3 &self receivers.
            #[allow(dead_code)]
            pub(crate) fn inner_mut(&self) -> std::cell::RefMut<'_, $inner_type> {
                rc_mut!(self.inner)
            }
        }
    };
}

macro_rules! define_audio_wrapper {
    ($wrapper_name:ident, $inner_type:ty, $shared_type:ty) => {
        #[pyclass(unsendable, from_py_object)]
        #[derive(Clone)]
        pub struct $wrapper_name {
            pub(crate) inner: $shared_type,
        }

        impl $wrapper_name {
            pub fn wrap(inner: $shared_type) -> Self {
                Self { inner }
            }

            pub(crate) fn inner_ref(&self) -> std::sync::MutexGuard<'_, $inner_type> {
                audio_ref!(self.inner)
            }

            pub(crate) fn inner_mut(&self) -> std::sync::MutexGuard<'_, $inner_type> {
                audio_mut!(self.inner)
            }
        }
    };
}
