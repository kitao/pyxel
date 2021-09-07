macro_rules! type_error {
    ($msg: expr) => {
        return Err(pyo3::exceptions::PyTypeError::new_err($msg))
    };
}

macro_rules! as_int {
    ($type: ty, $var: ident) => {
        if let Ok($var) = <$type>::extract($var) {
            $var
        } else if let Ok($var) = f64::extract($var) {
            $var as $type
        } else {
            type_error!("must be real number");
        }
    };
}

macro_rules! as_i32 {
    ($var: ident) => {
        as_int!(i32, $var)
    };
}

macro_rules! as_u32 {
    ($var: ident) => {
        as_int!(u32, $var)
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

macro_rules! define_list_index_method {
    () => {
        #[allow(dead_code)]
        fn index(&self, index: isize) -> usize {
            if index < 0 {
                (self.list().len() as isize + index) as usize
            } else {
                index as usize
            }
        }
    };
}

macro_rules! define_list_len_operator {
    ($self: ident, $list: expr) => {
        Ok($list($self).len())
    };
}

macro_rules! define_list_get_operator {
    ($self: ident, $list: expr, $index: ident) => {
        if $index < $list($self).len() as isize {
            Ok($list($self)[$index as usize].clone())
        } else {
            Err(pyo3::exceptions::PyIndexError::new_err(
                "list index out of range",
            ))
        }
    };
}

macro_rules! define_list_set_operator {
    ($self: ident, $list_mut: expr, $index: ident, $value: ident) => {
        if $index < $list_mut($self).len() as isize {
            $list_mut($self)[$index as usize] = $value;

            Ok(())
        } else {
            Err(pyo3::exceptions::PyIndexError::new_err(
                "list assignment index out of range",
            ))
        }
    };
}

macro_rules! define_list_del_operator {
    ($self: ident, $list_mut: expr, $index: ident) => {
        if $index < $list_mut($self).len() as isize {
            $list_mut($self).remove($index as usize);

            Ok(())
        } else {
            Err(pyo3::exceptions::PyIndexError::new_err(
                "list assignment index out of range",
            ))
        }
    };
}

macro_rules! define_list_edit_methods {
    ($elem_type: ty) => {
        pub fn append(&mut self, value: $elem_type) -> PyResult<()> {
            self.list_mut().push(value);

            Ok(())
        }

        pub fn insert(&mut self, index: isize, value: $elem_type) -> PyResult<()> {
            let index = self.index(index);

            self.list_mut().insert(index as usize, value);

            Ok(())
        }

        pub fn extend(&mut self, value: Vec<$elem_type>) -> PyResult<()> {
            self.list_mut().append(&mut value.clone());

            Ok(())
        }

        pub fn pop(&mut self, index: Option<isize>) -> PyResult<$elem_type> {
            if self.list().is_empty() {
                return Err(pyo3::exceptions::PyIndexError::new_err(
                    "pop from empty list",
                ));
            }

            let index = self.index(index.unwrap_or(self.list().len() as isize - 1));

            if index < self.list().len() {
                let value = self.list()[index as usize];
                self.list_mut().remove(index);

                Ok(value)
            } else {
                Err(pyo3::exceptions::PyIndexError::new_err(
                    "pop index out of range",
                ))
            }
        }

        pub fn clear(&mut self) -> PyResult<()> {
            self.list_mut().clear();

            Ok(())
        }
    };
}
