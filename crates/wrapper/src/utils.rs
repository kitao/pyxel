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

macro_rules! impl_len_method_for_list {
    ($self: ident) => {
        Ok($self.list().len())
    };
}

macro_rules! impl_getitem_method_for_list {
    ($self: ident, $index: ident) => {
        if $index < $self.list().len() as isize {
            Ok($self.list()[$index as usize].clone())
        } else {
            Err(pyo3::exceptions::PyIndexError::new_err(
                "list index out of range",
            ))
        }
    };
}

macro_rules! impl_setitem_method_for_list {
    ($self: ident, $index: ident, $value: ident) => {
        if $index < $self.list_mut().len() as isize {
            $self.list_mut()[$index as usize] = $value;
            Ok(())
        } else {
            Err(pyo3::exceptions::PyIndexError::new_err(
                "list assignment index out of range",
            ))
        }
    };
}

macro_rules! impl_from_list_method_for_list {
    ($self: ident, $list: ident) => {{
        *$self.list_mut() = $list;
        Ok(())
    }};
}

macro_rules! impl_to_list_method_for_list {
    ($self: ident) => {
        Ok($self.list().to_vec())
    };
}
