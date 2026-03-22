use crossbeam_utils::atomic::AtomicCell;
use rustpython_vm::builtins::{PyInt, PyList, PySlice, PyStrRef};
use rustpython_vm::protocol::{PyMappingMethods, PySequenceMethods};
use rustpython_vm::types::{AsMapping, AsSequence};
use rustpython_vm::{pyclass, PyObjectRef, PyPayload, PyResult, VirtualMachine};

use crate::channel_wrapper::PyChannel;
use crate::image_wrapper::PyImage;
use crate::music_wrapper::PyMusic;
use crate::sound_wrapper::PySound;
use crate::tilemap_wrapper::PyTilemap;
use crate::tone_wrapper::PyTone;

// Helper: resolve negative index
fn resolve_index(index: i32, len: usize) -> usize {
    if index < 0 {
        (len as i32 + index) as usize
    } else {
        index as usize
    }
}

// Macro to define a ptr-vec collection with getitem, setitem (index + slice), len
macro_rules! define_ptr_collection {
    ($name:ident, $pyname:literal, $item_type:ty, $global_fn:path, $err_msg:literal) => {
        #[pyclass(module = "pyxel", name = $pyname)]
        #[derive(Debug, PyPayload)]
        pub struct $name;

        #[pyclass(with(AsSequence, AsMapping))]
        impl $name {
            #[pymethod(magic)]
            fn getitem(&self, index: i32, vm: &VirtualMachine) -> PyResult<PyObjectRef> {
                let vec = $global_fn();
                let idx = resolve_index(index, vec.len());
                if idx >= vec.len() {
                    return Err(vm.new_index_error(concat!($err_msg, " index out of range").into()));
                }
                Ok(vm.new_pyobj(<$item_type>::wrap(vec[idx])))
            }

            #[pymethod(magic)]
            fn setitem(
                &self,
                key: PyObjectRef,
                value: PyObjectRef,
                vm: &VirtualMachine,
            ) -> PyResult<()> {
                if key.payload::<PySlice>().is_some() {
                    // Slice assignment: treat as full replacement
                    let py_list = value
                        .payload::<PyList>()
                        .ok_or_else(|| vm.new_type_error("expected list".into()))?;
                    let items = py_list.borrow_vec();
                    let mut new_vec = Vec::with_capacity(items.len());
                    for item in items.iter() {
                        let ptr = item
                            .payload::<$item_type>()
                            .map(|t| t.inner)
                            .ok_or_else(|| vm.new_type_error("wrong type in list".into()))?;
                        new_vec.push(ptr);
                    }
                    *$global_fn() = new_vec;
                    Ok(())
                } else if let Some(idx_int) = key.payload::<PyInt>() {
                    // Single index assignment
                    let i: i64 = idx_int
                        .as_bigint()
                        .try_into()
                        .map_err(|_| vm.new_overflow_error("index too large".into()))?;
                    let vec = $global_fn();
                    let idx = resolve_index(i as i32, vec.len());
                    if idx >= vec.len() {
                        return Err(
                            vm.new_index_error(concat!($err_msg, " index out of range").into())
                        );
                    }
                    let ptr = value
                        .payload::<$item_type>()
                        .map(|t| t.inner)
                        .ok_or_else(|| vm.new_type_error("wrong type".into()))?;
                    vec[idx] = ptr;
                    Ok(())
                } else {
                    Err(vm.new_type_error("indices must be integers or slices".into()))
                }
            }

            #[pymethod(magic)]
            fn len(&self) -> usize {
                $global_fn().len()
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
                        use rustpython_vm::builtins::PyInt;
                        let idx = key
                            .payload::<PyInt>()
                            .and_then(|v| v.as_bigint().try_into().ok())
                            .ok_or_else(|| vm.new_type_error("expected int index".into()))?;
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

define_ptr_collection!(PyImages, "_Images", PyImage, pyxel::images, "image");
define_ptr_collection!(PySounds, "_Sounds", PySound, pyxel::sounds, "sound");
define_ptr_collection!(
    PyTilemaps,
    "_Tilemaps",
    PyTilemap,
    pyxel::tilemaps,
    "tilemap"
);
define_ptr_collection!(PyMusics, "_Musics", PyMusic, pyxel::musics, "music");
define_ptr_collection!(
    PyChannels,
    "_Channels",
    PyChannel,
    pyxel::channels,
    "channel"
);
define_ptr_collection!(PyTones, "_Tones", PyTone, pyxel::tones, "tone");

// Collection type for pyxel.colors
#[pyclass(module = "pyxel", name = "_Colors")]
#[derive(Debug, PyPayload)]
pub struct PyColors;

#[pyclass(with(AsSequence, AsMapping))]
impl PyColors {
    #[pymethod(magic)]
    fn getitem(&self, index: i32, vm: &VirtualMachine) -> PyResult<PyObjectRef> {
        let colors = pyxel::colors();
        let idx = resolve_index(index, colors.len());
        if idx >= colors.len() {
            return Err(vm.new_index_error("color index out of range".into()));
        }
        Ok(vm.new_pyobj(colors[idx]))
    }

    #[pymethod(magic)]
    fn setitem_mapping(
        &self,
        key: PyObjectRef,
        value: PyObjectRef,
        vm: &VirtualMachine,
    ) -> PyResult<()> {
        if key.payload::<PySlice>().is_some() {
            // Slice assignment: colors[:] = [...]
            let py_list = value
                .payload::<PyList>()
                .ok_or_else(|| vm.new_type_error("expected list".into()))?;
            let items = py_list.borrow_vec();
            let colors = pyxel::colors();
            for (i, item) in items.iter().enumerate() {
                if i >= colors.len() {
                    break;
                }
                let v: u32 = item
                    .payload::<PyInt>()
                    .and_then(|n| n.as_bigint().try_into().ok())
                    .ok_or_else(|| vm.new_type_error("expected int".into()))?;
                colors[i] = v;
            }
            Ok(())
        } else if let Some(idx_int) = key.payload::<PyInt>() {
            let i: i64 = idx_int
                .as_bigint()
                .try_into()
                .map_err(|_| vm.new_overflow_error("index too large".into()))?;
            let colors = pyxel::colors();
            let idx = resolve_index(i as i32, colors.len());
            if idx >= colors.len() {
                return Err(vm.new_index_error("color index out of range".into()));
            }
            let v: u32 = value
                .payload::<PyInt>()
                .and_then(|n| n.as_bigint().try_into().ok())
                .ok_or_else(|| vm.new_type_error("expected int".into()))?;
            colors[idx] = v;
            Ok(())
        } else {
            Err(vm.new_type_error("indices must be integers or slices".into()))
        }
    }

    #[pymethod(magic)]
    fn setitem(&self, index: i32, value: u32, vm: &VirtualMachine) -> PyResult<()> {
        let colors = pyxel::colors();
        let idx = resolve_index(index, colors.len());
        if idx >= colors.len() {
            return Err(vm.new_index_error("color index out of range".into()));
        }
        colors[idx] = value;
        Ok(())
    }

    #[pymethod(magic)]
    fn len(&self) -> usize {
        pyxel::colors().len()
    }
}

impl AsMapping for PyColors {
    fn as_mapping() -> &'static PyMappingMethods {
        static AS_MAPPING: PyMappingMethods = PyMappingMethods {
            length: AtomicCell::new(None),
            subscript: AtomicCell::new(Some(|mapping, key, vm| {
                let idx = key
                    .payload::<PyInt>()
                    .and_then(|v| v.as_bigint().try_into().ok())
                    .ok_or_else(|| vm.new_type_error("expected int index".into()))?;
                PyColors::mapping_downcast(mapping).getitem(idx, vm)
            })),
            ass_subscript: AtomicCell::new(Some(|mapping, key, value, vm| {
                if let Some(value) = value {
                    PyColors::mapping_downcast(mapping).setitem_mapping(key.to_owned(), value, vm)
                } else {
                    Err(vm.new_type_error("cannot delete colors".into()))
                }
            })),
        };
        &AS_MAPPING
    }
}

impl AsSequence for PyColors {
    fn as_sequence() -> &'static PySequenceMethods {
        static AS_SEQUENCE: PySequenceMethods = PySequenceMethods {
            length: AtomicCell::new(Some(|seq, _vm| Ok(PyColors::sequence_downcast(seq).len()))),
            concat: AtomicCell::new(None),
            repeat: AtomicCell::new(None),
            item: AtomicCell::new(Some(|seq, i, vm| {
                PyColors::sequence_downcast(seq).getitem(i as i32, vm)
            })),
            ass_item: AtomicCell::new(Some(|seq, i, value, vm| {
                if let Some(value) = value {
                    let v = value
                        .payload::<PyInt>()
                        .and_then(|n| n.as_bigint().try_into().ok())
                        .ok_or_else(|| vm.new_type_error("expected int".into()))?;
                    PyColors::sequence_downcast(seq).setitem(i as i32, v, vm)
                } else {
                    Err(vm.new_type_error("cannot delete color".into()))
                }
            })),
            contains: AtomicCell::new(None),
            inplace_concat: AtomicCell::new(None),
            inplace_repeat: AtomicCell::new(None),
        };
        &AS_SEQUENCE
    }
}

// Module __getattr__ for dynamic variables and collections
pub fn module_getattr(name: PyStrRef, vm: &VirtualMachine) -> PyResult<PyObjectRef> {
    match name.as_str() {
        // System
        "width" => Ok(vm.new_pyobj(*pyxel::width())),
        "height" => Ok(vm.new_pyobj(*pyxel::height())),
        "frame_count" => Ok(vm.new_pyobj(*pyxel::frame_count())),

        // Input
        "mouse_x" => Ok(vm.new_pyobj(*pyxel::mouse_x())),
        "mouse_y" => Ok(vm.new_pyobj(*pyxel::mouse_y())),
        "mouse_wheel" => Ok(vm.new_pyobj(*pyxel::mouse_wheel())),
        "input_keys" => {
            let keys: Vec<PyObjectRef> = pyxel::input_keys()
                .iter()
                .map(|&k| vm.new_pyobj(k))
                .collect();
            Ok(vm.new_pyobj(keys))
        }
        "input_text" => Ok(vm.new_pyobj(pyxel::input_text().clone())),
        "dropped_files" => {
            let files: Vec<PyObjectRef> = pyxel::dropped_files()
                .iter()
                .map(|s| vm.new_pyobj(s.clone()))
                .collect();
            Ok(vm.new_pyobj(files))
        }

        // Graphics
        "colors" => Ok(vm.new_pyobj(PyColors)),
        "images" => Ok(vm.new_pyobj(PyImages)),
        "tilemaps" => Ok(vm.new_pyobj(PyTilemaps)),
        "screen" => Ok(vm.new_pyobj(PyImage::wrap(std::ptr::from_mut(pyxel::screen())))),
        "cursor" => Ok(vm.new_pyobj(PyImage::wrap(std::ptr::from_mut(pyxel::cursor_image())))),
        "font" => Ok(vm.new_pyobj(PyImage::wrap(std::ptr::from_mut(pyxel::font_image())))),

        // Audio
        "channels" => Ok(vm.new_pyobj(PyChannels)),
        "tones" => Ok(vm.new_pyobj(PyTones)),
        "sounds" => Ok(vm.new_pyobj(PySounds)),
        "musics" => Ok(vm.new_pyobj(PyMusics)),

        _ => Err(vm.new_attribute_error(format!(
            "module 'pyxel' has no attribute '{}'",
            name.as_str()
        ))),
    }
}
