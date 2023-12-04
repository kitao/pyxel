macro_rules! wrap_as_python_list {
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

            fn inner_len(&self) -> usize {
                $len(&self.inner)
            }

            fn inner_get(&self, index: usize) -> $value_type {
                $get(&self.inner, index)
            }

            fn inner_set(&mut self, index: usize, value: $value_type) {
                $set(&self.inner, index, value);
            }
        }

        #[pymethods]
        impl $wrapper_name {
            fn __len__(&self) -> PyResult<usize> {
                Ok(self.inner_len())
            }

            fn __getitem__(&self, idx: isize) -> PyResult<$value_type> {
                if idx < self.inner_len() as isize {
                    Ok(self.inner_get(idx as usize))
                } else {
                    Err(pyo3::exceptions::PyIndexError::new_err(
                        "list index out of range",
                    ))
                }
            }

            fn __setitem__(&mut self, idx: isize, value: $value_type) -> PyResult<()> {
                if idx < self.inner_len() as isize {
                    self.inner_set(idx as usize, value);
                    Ok(())
                } else {
                    Err(pyo3::exceptions::PyIndexError::new_err(
                        "list assignment index out of range",
                    ))
                }
            }

            pub fn from_list(&mut self, lst: Vec<$value_type>) -> PyResult<()> {
                for index in 0..lst.len() {
                    self.inner_set(index, lst[index]);
                }
                Ok(())
            }

            pub fn to_list(&self) -> PyResult<Vec<$value_type>> {
                let mut vec = Vec::with_capacity(self.inner_len());
                for i in 0..self.inner_len() {
                    vec.push(self.inner_get(i));
                }
                Ok(vec)
            }
        }
    };

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

            fn inner_len(&self) -> usize {
                $len(&self.inner)
            }

            fn inner_get(&self, index: usize) -> $value_type {
                $get(&self.inner, index)
            }

            fn inner_set(&mut self, index: usize, value: $value_type) {
                $set(&self.inner, index, value);
            }

            fn inner_insert(&mut self, index: usize, value: $value_type) {
                $insert(&self.inner, index, value);
            }

            fn inner_remove(&mut self, index: usize) {
                $remove(&self.inner, index);
            }
        }

        #[pymethods]
        impl $wrapper_name {
            fn __len__(&self) -> PyResult<usize> {
                Ok(self.inner_len())
            }

            fn __getitem__(&self, idx: isize) -> PyResult<$value_type> {
                if idx < self.inner_len() as isize {
                    Ok(self.inner_get(idx as usize))
                } else {
                    Err(pyo3::exceptions::PyIndexError::new_err(
                        "list index out of range",
                    ))
                }
            }

            fn __setitem__(&mut self, idx: isize, value: $value_type) -> PyResult<()> {
                if idx < self.inner_len() as isize {
                    self.inner_set(idx as usize, value);
                    Ok(())
                } else {
                    Err(pyo3::exceptions::PyIndexError::new_err(
                        "list assignment index out of range",
                    ))
                }
            }

            pub fn from_list(&mut self, lst: Vec<$value_type>) -> PyResult<()> {
                while self.inner_len() > 0 {
                    self.inner_remove(0);
                }
                for value in lst {
                    self.inner_insert(self.inner_len(), value.clone());
                }
                Ok(())
            }

            pub fn to_list(&self) -> PyResult<Vec<$value_type>> {
                let mut vec = Vec::with_capacity(self.inner_len());
                for i in 0..self.inner_len() {
                    vec.push(self.inner_get(i));
                }
                Ok(vec)
            }
        }
    };

    ($wrapper_name:ident, $value_type:ty, $nested_value_type:ty, $inner_type:ty, $len:expr, $get:expr, $set:expr, $insert:expr, $remove:expr) => {
        #[pyclass]
        #[derive(Clone)]
        pub struct $wrapper_name {
            inner: $inner_type,
        }

        impl $wrapper_name {
            pub const fn wrap(inner: $inner_type) -> Self {
                Self { inner }
            }

            fn inner_len(&self) -> usize {
                $len(&self.inner)
            }

            fn inner_get(&self, index: usize) -> $value_type {
                $get(&self.inner, index)
            }

            fn inner_set(&mut self, index: usize, value: $value_type) {
                $set(&self.inner, index, value);
            }

            fn inner_insert(&mut self, index: usize, value: $value_type) {
                $insert(&self.inner, index, value);
            }

            fn inner_remove(&mut self, index: usize) {
                $remove(&self.inner, index);
            }
        }

        #[pymethods]
        impl $wrapper_name {
            fn __len__(&self) -> PyResult<usize> {
                Ok(self.inner_len())
            }

            fn __getitem__(&self, idx: isize) -> PyResult<$value_type> {
                if idx < self.inner_len() as isize {
                    Ok(self.inner_get(idx as usize))
                } else {
                    Err(pyo3::exceptions::PyIndexError::new_err(
                        "list index out of range",
                    ))
                }
            }

            fn __setitem__(&mut self, idx: isize, value: $value_type) -> PyResult<()> {
                if idx < self.inner_len() as isize {
                    self.inner_set(idx as usize, value);
                    Ok(())
                } else {
                    Err(pyo3::exceptions::PyIndexError::new_err(
                        "list assignment index out of range",
                    ))
                }
            }

            pub fn from_list(&mut self, lst: Vec<Vec<$nested_value_type>>) -> PyResult<()> {
                self.inner.lock().set(&lst);
                Ok(())
            }

            pub fn to_list(&self) -> PyResult<Vec<Vec<$nested_value_type>>> {
                let mut vec = Vec::with_capacity(self.inner_len());
                for i in 0..self.inner_len() {
                    vec.push(self.inner_get(i).to_list().unwrap());
                }
                Ok(vec)
            }
        }
    };
}
