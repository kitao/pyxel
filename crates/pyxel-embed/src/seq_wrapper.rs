// Live sequence wrappers for Sound.notes/tones/volumes/effects, Tone.wavetable, etc.
// These wrap a raw pointer to the parent and provide direct read/write access to the Vec field.

use crossbeam_utils::atomic::AtomicCell;
use rustpython_vm::builtins::{PyInt, PyList, PySlice};
use rustpython_vm::protocol::{PyMappingMethods, PySequenceMethods};
use rustpython_vm::types::{AsMapping, AsSequence};
use rustpython_vm::{pyclass, PyObjectRef, PyPayload, PyResult, VirtualMachine};

use crate::helpers::*;

macro_rules! define_live_seq {
    (
        $name:ident, $pyname:literal, $parent:ty, $elem:ty,
        $field:ident, $to_pyobj:expr, $from_pyobj:expr
    ) => {
        #[pyclass(module = "pyxel", name = $pyname)]
        #[derive(Debug, PyPayload)]
        pub struct $name {
            pub inner: *mut $parent,
        }

        unsafe impl Send for $name {}
        unsafe impl Sync for $name {}

        impl $name {
            pub fn wrap(inner: *mut $parent) -> Self {
                Self { inner }
            }

            #[allow(clippy::mut_from_ref)]
            fn parent(&self) -> &mut $parent {
                unsafe { &mut *self.inner }
            }
        }

        #[pyclass(with(AsSequence, AsMapping))]
        impl $name {
            #[pymethod(magic)]
            fn getitem(&self, index: i32, vm: &VirtualMachine) -> PyResult<PyObjectRef> {
                let vec = &self.parent().$field;
                let idx = if index < 0 {
                    (vec.len() as i32 + index) as usize
                } else {
                    index as usize
                };
                if idx >= vec.len() {
                    return Err(vm.new_index_error("index out of range".into()));
                }
                let val = vec[idx];
                Ok(($to_pyobj)(val, vm))
            }

            #[pymethod(magic)]
            fn setitem(
                &self,
                key: PyObjectRef,
                value: PyObjectRef,
                vm: &VirtualMachine,
            ) -> PyResult<()> {
                if key.payload::<PySlice>().is_some() {
                    // Slice assignment: replace all
                    let py_list = value
                        .payload::<PyList>()
                        .ok_or_else(|| vm.new_type_error("expected list".into()))?;
                    let items = py_list.borrow_vec();
                    let mut new_vec = Vec::with_capacity(items.len());
                    for item in items.iter() {
                        new_vec.push(($from_pyobj)(item, vm)?);
                    }
                    self.parent().$field = new_vec;
                    Ok(())
                } else if let Some(idx_int) = key.payload::<PyInt>() {
                    let i: i64 = idx_int
                        .as_bigint()
                        .try_into()
                        .map_err(|_| vm.new_overflow_error("index too large".into()))?;
                    let vec = &mut self.parent().$field;
                    let idx = if i < 0 {
                        (vec.len() as i64 + i) as usize
                    } else {
                        i as usize
                    };
                    if idx >= vec.len() {
                        return Err(vm.new_index_error("index out of range".into()));
                    }
                    vec[idx] = ($from_pyobj)(&value, vm)?;
                    Ok(())
                } else {
                    Err(vm.new_type_error("indices must be integers or slices".into()))
                }
            }

            #[pymethod(magic)]
            fn len(&self) -> usize {
                self.parent().$field.len()
            }
        }

        impl AsSequence for $name {
            fn as_sequence() -> &'static PySequenceMethods {
                static AS_SEQUENCE: PySequenceMethods = PySequenceMethods {
                    length: AtomicCell::new(Some(|seq, _vm| {
                        Ok($name::sequence_downcast(seq).len())
                    })),
                    concat: AtomicCell::new(None),
                    repeat: AtomicCell::new(None),
                    item: AtomicCell::new(Some(|seq, i, vm| {
                        $name::sequence_downcast(seq).getitem(i as i32, vm)
                    })),
                    ass_item: AtomicCell::new(None),
                    contains: AtomicCell::new(None),
                    inplace_concat: AtomicCell::new(None),
                    inplace_repeat: AtomicCell::new(None),
                };
                &AS_SEQUENCE
            }
        }

        impl AsMapping for $name {
            fn as_mapping() -> &'static PyMappingMethods {
                static AS_MAPPING: PyMappingMethods = PyMappingMethods {
                    length: AtomicCell::new(None),
                    subscript: AtomicCell::new(Some(|mapping, key, vm| {
                        let idx: i32 = key
                            .payload::<PyInt>()
                            .and_then(|v| v.as_bigint().try_into().ok())
                            .ok_or_else(|| vm.new_type_error("expected int".into()))?;
                        $name::mapping_downcast(mapping).getitem(idx, vm)
                    })),
                    ass_subscript: AtomicCell::new(Some(|mapping, key, value, vm| {
                        if let Some(value) = value {
                            $name::mapping_downcast(mapping).setitem(key.to_owned(), value, vm)
                        } else {
                            Err(vm.new_type_error("cannot delete items".into()))
                        }
                    })),
                };
                &AS_MAPPING
            }
        }
    };
}

// Sound field wrappers
define_live_seq!(
    PyNotes,
    "_Notes",
    pyxel::Sound,
    pyxel::SoundNote,
    notes,
    |v: pyxel::SoundNote, vm: &VirtualMachine| vm.new_pyobj(v),
    |obj: &PyObjectRef, vm: &VirtualMachine| -> PyResult<pyxel::SoundNote> {
        let v = c(obj, vm)?;
        Ok(v as pyxel::SoundNote)
    }
);

define_live_seq!(
    PyTones,
    "_Tones_Seq",
    pyxel::Sound,
    pyxel::SoundTone,
    tones,
    |v: pyxel::SoundTone, vm: &VirtualMachine| vm.new_pyobj(v),
    |obj: &PyObjectRef, vm: &VirtualMachine| -> PyResult<pyxel::SoundTone> { c(obj, vm) }
);

define_live_seq!(
    PyVolumes,
    "_Volumes",
    pyxel::Sound,
    pyxel::SoundVolume,
    volumes,
    |v: pyxel::SoundVolume, vm: &VirtualMachine| vm.new_pyobj(v),
    |obj: &PyObjectRef, vm: &VirtualMachine| -> PyResult<pyxel::SoundVolume> { c(obj, vm) }
);

define_live_seq!(
    PyEffects,
    "_Effects",
    pyxel::Sound,
    pyxel::SoundEffect,
    effects,
    |v: pyxel::SoundEffect, vm: &VirtualMachine| vm.new_pyobj(v),
    |obj: &PyObjectRef, vm: &VirtualMachine| -> PyResult<pyxel::SoundEffect> { c(obj, vm) }
);

// Tone wavetable wrapper
define_live_seq!(
    PyWavetable,
    "_Wavetable",
    pyxel::Tone,
    pyxel::ToneSample,
    wavetable,
    |v: pyxel::ToneSample, vm: &VirtualMachine| vm.new_pyobj(v),
    |obj: &PyObjectRef, vm: &VirtualMachine| -> PyResult<pyxel::ToneSample> { u(obj, vm) }
);
