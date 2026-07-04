# PocketPy Local Patches

The vendored PocketPy source is based on upstream version 2.0.6. Pyxel keeps the
source checked in for reproducible builds and carries only the local compatibility
patches required by the `pyxel-pocket` tests.

- Add integer `x` and `X` format support for Pyxel examples that use hexadecimal
  format specifiers.
- Add mixed integer/float floor division and float floor division so PocketPy
  matches the Python arithmetic used by shipped examples.
- Add mixed integer/float modulo and float modulo so animation and platformer
  scripts can use Python's numeric operators unchanged.
- Add value construction for `Enum` objects so the runtime compatibility layer
  can approximate `IntEnum(value)` lookups used by shipped apps.

The focused coverage lives in `crates/pyxel-pocket/tests/runtime_smoke.rs`:
arithmetic compatibility tests cover the numeric patches, and the shipped
example/app screenshot tests cover the visible Pyxel behavior that depends on
them.
