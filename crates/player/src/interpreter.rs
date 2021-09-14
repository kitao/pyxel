use cpython::{FromPyObject, GILGuard, PyDict, Python};

use crate::utils::command_args;

pub struct Interpreter {
    gil: GILGuard,
    locals: PyDict,
}

impl<'a> Interpreter {
    pub fn new() -> Interpreter {
        let gil = Python::acquire_gil();
        let locals = PyDict::new(gil.python());

        Interpreter { gil, locals }
    }

    pub fn add_import_paths(&self, paths: &[&str]) {
        self.import("os");
        self.import("sys");

        for path in paths {
            let code = format!(
                r#"
sys.path.append(os.path.join(os.path.dirname("{}"), "{}"))
                "#,
                command_args()[0],
                path
            );

            self.run_code(&code);
        }
    }

    pub fn import(&self, module: &str) {
        let py = self.gil.python();

        self.locals
            .set_item(py, module, py.import(module).unwrap())
            .unwrap();
    }

    pub fn eval<T>(&self, code: &str) -> T
    where
        for<'s> T: FromPyObject<'s>,
    {
        let py = self.gil.python();

        py.eval(code, None, Some(&self.locals))
            .unwrap()
            .extract(py)
            .unwrap()
    }

    pub fn run_code(&self, code: &str) {
        self.gil
            .python()
            .run(code, None, Some(&self.locals))
            .unwrap();
    }

    pub fn run_file(&self, filename: &str) {
        self.import("importlib");
        self.import("importlib.util");

        let code = format!(
            r#"
spec = importlib.util.spec_from_file_location("__main__", "{}")
file = importlib.util.module_from_spec(spec)
spec.loader.exec_module(file)
del spec, file
            "#,
            filename
        );

        self.run_code(&code);
    }
}
