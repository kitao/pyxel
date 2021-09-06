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

macro_rules! sequence_len {
    ($vec: expr) => {
        Ok($vec.len())
    };
}

macro_rules! sequence_get {
    ($vec: expr, $idx: ident) => {
        if let Some(value) = $vec.get($idx as usize) {
            Ok(value.clone())
        } else {
            Err(pyo3::exceptions::PyIndexError::new_err(
                "list index out of range",
            ))
        }
    };
}

macro_rules! sequence_set {
    ($vec: expr, $idx: ident, $value: ident) => {{
        $vec[$idx as usize] = $value;

        Ok(())
    }};
}

macro_rules! sequence_del {
    ($vec: expr, $idx: ident) => {
        if ($idx < $vec.len() as isize) && ($idx >= 0) {
            $vec.remove($idx as usize);
            Ok(())
        } else {
            Err(pyo3::exceptions::PyIndexError::new_err(
                "list index out of range",
            ))
        }
    };
}

/*
    fn __concat__(&self, other: PyRef<'p, Self>) -> Self {
        let mut elements = self.elements.clone();
        elements.extend_from_slice(&other.elements);
        Self { elements }
    }

    fn __repeat__(&self, count: isize) -> PyResult<Self> {
        if count >= 0 {
            let mut elements = Vec::with_capacity(self.elements.len() * count as usize);
            for _ in 0..count {
                elements.extend(&self.elements);
            }
            Ok(Self { elements })
        } else {
            Err(PyValueError::new_err("invalid repeat count"))
        }
    }

    fn __inplace_concat__(&'p mut self, other: Self::Other) -> Self::Result
    where
        Self: PySequenceInplaceConcatProtocol<'p>,
    {
        unimplemented!()
    }

    fn __inplace_repeat__(&'p mut self, count: Self::Index) -> Self::Result
    where
        Self: PySequenceInplaceRepeatProtocol<'p>,
    {
        unimplemented!()
    }
*/
