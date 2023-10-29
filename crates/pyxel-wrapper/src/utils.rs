macro_rules! type_error {
    ($msg: expr) => {
        return Err(pyo3::exceptions::PyTypeError::new_err($msg))
    };
}

macro_rules! type_switch {
    ($var: ident, $type1: ty, $block1: block, $type2: ty, $block2: block) => {
        if let Ok($var) = <$type1>::extract($var) {
            $block1
        } else if let Ok($var) = <$type2>::extract($var) {
            $block2
        } else {
            type_error!(format!(
                "must be {} or {}",
                stringify!($type1),
                stringify!($type2)
            ));
        }
    };

    ($var: ident, $type1: ty, $block1: block, $type2: ty, $block2: block, $type3: ty, $block3: block, $type4: ty, $block4: block) => {
        if let Ok($var) = <$type1>::extract($var) {
            $block1
        } else if let Ok($var) = <$type2>::extract($var) {
            $block2
        } else if let Ok($var) = <$type3>::extract($var) {
            $block3
        } else if let Ok($var) = <$type4>::extract($var) {
            $block4
        } else {
            type_error!(format!(
                "must be {}, {}, {}, or {}",
                stringify!($type1),
                stringify!($type2),
                stringify!($type3),
                stringify!($type4)
            ));
        }
    };
}

macro_rules! wrap_as_python_list {
    ($wrapper_name:ident, $value_type:ty, $inner_type:ty, $len:expr, $get:expr, $set:expr, $insert:expr, $remove:expr) => {
        #[pyclass]
        #[derive(Clone)]
        pub struct $wrapper_name {
            inner: $inner_type,
        }

        impl $wrapper_name {
            pub const fn wrap(inner: $inner_type) -> Self {
                Self { inner }
            }

            fn len(&self) -> usize {
                $len(&self.inner)
            }

            fn get(&self, index: usize) -> $value_type {
                $get(&self.inner, index)
            }

            fn set(&mut self, index: usize, value: $value_type) {
                $set(&self.inner, index, value);
            }

            fn insert(&mut self, index: usize, value: $value_type) {
                $insert(&self.inner, index, value);
            }

            fn remove(&mut self, index: usize) {
                $remove(&self.inner, index);
            }
        }

        #[pymethods]
        impl $wrapper_name {
            fn __len__(&self) -> PyResult<usize> {
                Ok(self.len())
            }

            fn __getitem__(&self, idx: isize) -> PyResult<$value_type> {
                if idx < self.len() as isize {
                    Ok(self.get(idx as usize))
                } else {
                    Err(pyo3::exceptions::PyIndexError::new_err(
                        "list index out of range",
                    ))
                }
            }

            fn __setitem__(&mut self, idx: isize, value: $value_type) -> PyResult<()> {
                if idx < self.len() as isize {
                    self.set(idx as usize, value);
                    Ok(())
                } else {
                    Err(pyo3::exceptions::PyIndexError::new_err(
                        "list assignment index out of range",
                    ))
                }
            }

            pub fn from_list(&mut self, lst: Vec<$value_type>) -> PyResult<()> {
                while self.len() > 0 {
                    self.remove(0);
                }
                for value in lst {
                    self.insert(self.len(), value.clone());
                }
                Ok(())
            }

            pub fn to_list(&self) -> PyResult<Vec<$value_type>> {
                let mut vec = Vec::with_capacity(self.len());
                for i in 0..self.len() {
                    vec.push(self.get(i));
                }
                Ok(vec)
            }
        }
    };
}
