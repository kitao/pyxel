macro_rules! deprecation_warning {
    ($name:ident, $msg:expr) => {
        static $name: std::sync::Once = std::sync::Once::new();
        $name.call_once(|| println!($msg));
    };
}

macro_rules! validate_index {
    ($index:expr, $len:expr, $name:expr) => {
        if ($index as usize) >= $len {
            return Err(pyo3::exceptions::PyValueError::new_err(format!(
                "Invalid {} index",
                $name
            )));
        }
    };
}

macro_rules! python_type_error {
    ($msg: expr) => {
        return Err(pyo3::exceptions::PyTypeError::new_err($msg))
    };
}

macro_rules! cast_pyany {
    ($pyany: ident, $(($type: ty, $block: block)),*) => {
        {
            let mut types = String::new();
            loop {
                $(
                    if !types.is_empty() {
                        types += ", "
                    }
                    let any_ref: &pyo3::Bound<'_, pyo3::PyAny> = $pyany.as_any();
                    let borrowed: pyo3::Borrowed<'_, '_, pyo3::PyAny> = any_ref.into();
                    if let Ok($pyany) = <$type>::extract(borrowed) {
                        break $block;
                    }
                    types += stringify!($type);
                )*
                python_type_error!(format!("must be {}", types));
            }
        }
    };
}

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

macro_rules! collect_slice_indices {
    ($start:expr, $stop:expr, $step:expr) => {{
        let start: isize = $start;
        let stop: isize = $stop;
        let step: isize = $step;
        let mut indices = Vec::new();
        let mut i = start;
        if step > 0 {
            while i < stop {
                indices.push(i as usize);
                i += step;
            }
        } else {
            while i > stop {
                indices.push(i as usize);
                i += step;
            }
        }
        indices
    }};
}

// Collect items into a PyList and return its iterator
macro_rules! items_to_pyiter {
    ($py:expr, $items:expr) => {{
        let list = pyo3::types::PyList::new($py, $items)?;
        Ok(list.call_method0("__iter__")?.unbind())
    }};
}

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
                    let idx_list =
                        collect_slice_indices!(indices.start, indices.stop, indices.step);
                    let items: Vec<$get_type> =
                        idx_list.iter().map(|&i| $get(&self.inner, i)).collect();
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
                let items: Vec<$get_type> = (0..$len(&self.inner))
                    .map(|i| $get(&self.inner, i))
                    .collect();
                items_to_pyiter!(py, items)
            }

            fn __reversed__(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
                let items: Vec<$get_type> = (0..$len(&self.inner))
                    .rev()
                    .map(|i| $get(&self.inner, i))
                    .collect();
                items_to_pyiter!(py, items)
            }

            fn __repr__(&self, py: Python) -> PyResult<String> {
                let len = $len(&self.inner);
                let items: Vec<$get_type> = (0..len).map(|i| $get(&self.inner, i)).collect();
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
                let mut items: Vec<$get_type> = (0..len).map(|i| $get(&self.inner, i)).collect();
                let other_items: Vec<$get_type> = other.extract()?;
                items.extend(other_items);
                let list = pyo3::types::PyList::new(py, items)?;
                Ok(list.into_any().unbind())
            }

            fn __mul__(&self, py: Python<'_>, n: isize) -> PyResult<Py<PyAny>> {
                let len = $len(&self.inner);
                let items: Vec<$get_type> = (0..len).map(|i| $get(&self.inner, i)).collect();
                let repeated = if n > 0 {
                    items.repeat(n as usize)
                } else {
                    Vec::new()
                };
                let list = pyo3::types::PyList::new(py, repeated)?;
                Ok(list.into_any().unbind())
            }
        }
    };
}

// Mutable sequence methods: __setitem__, __delitem__, __iadd__,
// append, extend, insert, pop, clear
// All mutations go through $to_list/$from_list for bulk operations.
macro_rules! impl_python_sequence_write {
    (
        $wrapper_name:ident, $inner_type:ty, $len:expr,
        $get_type:ty, $set_type:ty, $set:expr,
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
                        let end = indices.stop as usize;
                        let mut lst = $to_list(&self.inner);
                        lst.splice(start..end, new_values);
                        $from_list(&self.inner, lst);
                    } else {
                        let idx_list = collect_slice_indices!(
                            indices.start,
                            indices.stop,
                            indices.step
                        );
                        if new_values.len() != idx_list.len() {
                            return Err(pyo3::exceptions::PyValueError::new_err(format!(
                                "attempt to assign sequence of size {} to extended slice of size {}",
                                new_values.len(),
                                idx_list.len()
                            )));
                        }
                        for (pos, val) in idx_list.into_iter().zip(new_values) {
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
                    let mut idx_list = collect_slice_indices!(
                        indices.start,
                        indices.stop,
                        indices.step
                    );
                    // Remove from end to preserve earlier indices
                    idx_list.sort_unstable_by(|a, b| b.cmp(a));
                    let mut lst = $to_list(&self.inner);
                    for i in idx_list {
                        lst.remove(i);
                    }
                    $from_list(&self.inner, lst);
                    Ok(())
                } else {
                    let idx: isize = key.extract()?;
                    let i = resolve_index!(idx, $len(&self.inner))?;
                    let mut lst = $to_list(&self.inner);
                    lst.remove(i);
                    $from_list(&self.inner, lst);
                    Ok(())
                }
            }

            fn __iadd__(&self, values: Vec<$set_type>) {
                let mut lst = $to_list(&self.inner);
                lst.extend(values);
                $from_list(&self.inner, lst);
            }

            fn append(&self, value: $set_type) {
                let mut lst = $to_list(&self.inner);
                lst.push(value);
                $from_list(&self.inner, lst);
            }

            fn extend(&self, values: Vec<$set_type>) {
                let mut lst = $to_list(&self.inner);
                lst.extend(values);
                $from_list(&self.inner, lst);
            }

            #[pyo3(signature = (index, value))]
            fn insert(&self, index: isize, value: $set_type) {
                let mut lst = $to_list(&self.inner);
                let len = lst.len();
                let i = if index < 0 {
                    let resolved = index + len as isize;
                    if resolved < 0 { 0 } else { resolved as usize }
                } else if index as usize > len {
                    len
                } else {
                    index as usize
                };
                lst.insert(i, value);
                $from_list(&self.inner, lst);
            }

            #[pyo3(signature = (index=None))]
            fn pop(&self, index: Option<isize>) -> PyResult<$get_type> {
                let mut lst = $to_list(&self.inner);
                let len = lst.len();
                if len == 0 {
                    return Err(pyo3::exceptions::PyIndexError::new_err(
                        "pop from empty sequence",
                    ));
                }
                let idx = index.unwrap_or(-1);
                let i = resolve_index!(idx, len)?;
                let value = lst.remove(i);
                $from_list(&self.inner, lst);
                Ok(value)
            }

            fn clear(&self) {
                $from_list(&self.inner, Vec::new());
            }

            fn from_list(&self, lst: $list_type) -> PyResult<()> {
                deprecation_warning!(
                    FROM_LIST_ONCE,
                    concat!(stringify!($wrapper_name), ".from_list() is deprecated. Use slice assignment instead.")
                );
                $from_list(&self.inner, lst);
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

// Wrapper for primitive-type sequences with comparison ops.
macro_rules! wrap_as_python_sequence {
    (
        $wrapper_name:ident, $inner_type:ty, $len:expr,
        $get_type:ty, $get:expr,
        $set_type:ty, $set:expr,
        $list_type:ty, $from_list:expr, $to_list:expr
    ) => {
        #[pyclass(sequence, from_py_object)]
        #[derive(Clone)]
        pub struct $wrapper_name {
            inner: $inner_type,
        }

        unsafe impl Send for $wrapper_name {}
        unsafe impl Sync for $wrapper_name {}

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
            $list_type,
            $from_list,
            $to_list
        );
    };
}

// Wrapper for object/wrapper-type sequences (no Copy/PartialEq).
macro_rules! wrap_as_python_object_sequence {
    (
        $wrapper_name:ident, $inner_type:ty, $len:expr,
        $get_type:ty, $get:expr,
        $set_type:ty, $set:expr,
        $list_type:ty, $from_list:expr, $to_list:expr
    ) => {
        #[pyclass(sequence, skip_from_py_object)]
        #[derive(Clone)]
        pub struct $wrapper_name {
            inner: $inner_type,
        }

        unsafe impl Send for $wrapper_name {}
        unsafe impl Sync for $wrapper_name {}

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
            $list_type,
            $from_list,
            $to_list
        );
    };
}

macro_rules! value_to_pyobj {
    ($py:expr, $value:expr) => {
        $value.into_pyobject($py).unwrap().into()
    };
}

macro_rules! class_to_pyobj {
    ($py:expr, $instance:expr) => {{
        $instance.into_pyobject($py).unwrap().into_any().unbind()
    }};
}

macro_rules! define_wrapper {
    ($name:ident, $inner:ty) => {
        #[pyclass(from_py_object)]
        #[derive(Clone, Copy)]
        pub struct $name {
            pub(crate) inner: *mut $inner,
        }

        unsafe impl Send for $name {}
        unsafe impl Sync for $name {}

        impl $name {
            pub fn wrap(inner: *mut $inner) -> Self {
                Self { inner }
            }

            #[allow(dead_code)]
            fn inner_ref(&self) -> &$inner {
                unsafe { &*self.inner }
            }

            #[allow(clippy::mut_from_ref)]
            fn inner_mut(&self) -> &mut $inner {
                unsafe { &mut *self.inner }
            }
        }
    };
}
