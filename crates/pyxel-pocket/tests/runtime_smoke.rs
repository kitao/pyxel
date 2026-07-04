#[test]
fn exec_source_accepts_simple_python() {
    pyxel_pocket::Runtime::new()
        .exec_source("x = 1 + 2", "<test>")
        .unwrap();
}

#[test]
fn pyxel_module_exposes_constants_without_initializing_core() {
    pyxel_pocket::Runtime::new()
        .exec_source(
            "\
import pyxel
assert pyxel.KEY_Q >= 0
assert pyxel.COLOR_WHITE >= 0
",
            "<test>",
        )
        .unwrap();
}
