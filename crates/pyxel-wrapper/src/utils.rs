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
    ($pyany: ident, $(($type: ty, $block: block)),*) => {
        {
            let mut types = String::new();
            loop {
                $(
                    if !types.is_empty() {
                        types += ", "
                    }
                    if let Ok($pyany) = <$type>::extract($pyany) {
                        break $block;
                    }
                    types += stringify!($type);
                )*
                python_type_error!(format!("must be {}", types));
            }
        }
    };
}
