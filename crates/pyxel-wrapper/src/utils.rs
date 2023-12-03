use pyo3::types::PyDict;
use pyo3::Python;

pub fn warn_with_python_caller(message: &str) {
    Python::with_gil(|py| {
        let locals = PyDict::new(py);
        py.run(
            "import traceback; stack_info = traceback.extract_stack()[-2][:2]",
            None,
            Some(locals),
        )
        .unwrap();
        let stack_info = locals.get_item("stack_info").unwrap();
        if let Ok((file, line)) = stack_info.unwrap().extract::<(String, i64)>() {
            print!("{file}:{line}: ");
        }
        println!("{message}");
    });
}

macro_rules! python_type_error {
    ($msg: expr) => {
        return Err(pyo3::exceptions::PyTypeError::new_err($msg))
    };
}

macro_rules! cast_pyany {
    ($pyany: ident, ($type1: ty, $block1: block), ($type2: ty, $block2: block)) => {
        if let Ok($pyany) = <$type1>::extract($pyany) {
            $block1
        } else if let Ok($pyany) = <$type2>::extract($pyany) {
            $block2
        } else {
            python_type_error!(format!(
                "must be {} or {}",
                stringify!($type1),
                stringify!($type2)
            ));
        }
    };

    ($pyany: ident, ($type1: ty, $block1: block), ($type2: ty, $block2: block), ($type3: ty, $block3: block), ($type4: ty, $block4: block)) => {
        if let Ok($pyany) = <$type1>::extract($pyany) {
            $block1
        } else if let Ok($pyany) = <$type2>::extract($pyany) {
            $block2
        } else if let Ok($pyany) = <$type3>::extract($pyany) {
            $block3
        } else if let Ok($pyany) = <$type4>::extract($pyany) {
            $block4
        } else {
            python_type_error!(format!(
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

    ($wrapper_name:ident, $value_type:ty, $inner_type:ty, $len:expr, $get:expr, $set:expr) => {
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
                for index in 0..lst.len() {
                    self.set(index, lst[index]);
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
