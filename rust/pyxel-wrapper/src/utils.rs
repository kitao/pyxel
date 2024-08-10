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
                    if let Ok($pyany) = <$type>::extract_bound(&$pyany) {
                        break $block;
                    }
                    types += stringify!($type);
                )*
                python_type_error!(format!("must be {}", types));
            }
        }
    };
}

macro_rules! wrap_as_python_list {
    ($wrapper_name:ident, $inner_type:ty, $len:expr, $get_type:ty, $get:expr, $set_type:ty, $set:expr, $list_type:ty, $from_list:expr, $to_list:expr) => {
        #[pyclass]
        #[derive(Clone)]
        pub struct $wrapper_name {
            inner: $inner_type,
        }

        impl $wrapper_name {
            pub const fn wrap(inner: $inner_type) -> Self {
                Self { inner }
            }
        }

        #[pymethods]
        impl $wrapper_name {
            fn __len__(&self) -> PyResult<usize> {
                Ok($len(&self.inner))
            }

            fn __getitem__(&self, idx: isize) -> PyResult<$get_type> {
                if idx < $len(&self.inner) as isize {
                    Ok($get(&self.inner, idx as usize))
                } else {
                    Err(pyo3::exceptions::PyIndexError::new_err(
                        "list index out of range",
                    ))
                }
            }

            fn __setitem__(&mut self, idx: isize, value: $set_type) -> PyResult<()> {
                if idx < $len(&self.inner) as isize {
                    $set(&self.inner, idx as usize, value);
                    Ok(())
                } else {
                    Err(pyo3::exceptions::PyIndexError::new_err(
                        "list assignment index out of range",
                    ))
                }
            }

            pub fn from_list(&mut self, lst: $list_type) -> PyResult<()> {
                $from_list(&self.inner, lst);
                Ok(())
            }

            pub fn to_list(&self) -> PyResult<$list_type> {
                Ok($to_list(&self.inner))
            }
        }
    };
}
