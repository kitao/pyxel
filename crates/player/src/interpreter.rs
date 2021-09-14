use cpython::{GILGuard, Python};

use crate::utils::command_args;

pub struct Interpreter {
    gil: GILGuard,
}

impl<'a> Interpreter {
    pub fn new() -> Interpreter {
        let interpreter = Interpreter {
            gil: Python::acquire_gil(),
        };

        interpreter.init_import_path();

        interpreter
    }

    pub fn run_code(&self, code: &str) {
        self.gil.python().run(code, None, None).unwrap();
    }

    pub fn run_file(&self, filename: &str) {
        let code = format!(
            r#"
import importlib.util

spec = importlib.util.spec_from_file_location("__main__", "{}")
file = importlib.util.module_from_spec(spec)
spec.loader.exec_module(file)

del importlib.util, spec, file
        "#,
            filename
        );

        self.run_code(&code);
    }

    fn init_import_path(&self) {
        let code = format!(
            r#"
import os
import sys

dirname = os.path.dirname("{}")

sys.path.insert(1, os.path.join(dirname, "../../.."))
sys.path.insert(1, os.path.join(dirname, "../.."))
sys.path.insert(1, os.path.join(dirname, ".."))
sys.path.insert(1, os.path.join(dirname, "."))

del os, sys, dirname
            "#,
            command_args()[0]
        );

        self.run_code(&code);
    }
}
