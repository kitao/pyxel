use std::collections::HashSet;

use pyo3::basic::CompareOp;
use pyo3::exceptions::PyKeyError;
use pyo3::prelude::*;
use pyo3::types::{PyFrozenSet, PySet, PyTuple};

// A live set view keeps tag queries and Python edits on the same native data.
#[pyclass(module = "pyxel.cube", name = "_Tags", unsendable, from_py_object)]
#[derive(Clone)]
pub struct Tags {
    inner: pyxel::cube::RcNode,
}

impl Tags {
    pub const fn wrap(inner: pyxel::cube::RcNode) -> Self {
        Self { inner }
    }

    pub fn extract(value: &Bound<'_, PyAny>) -> PyResult<HashSet<String>> {
        if let Ok(tags) = value.cast::<Self>() {
            Ok(rc_ref!(&tags.borrow().inner).tags.clone())
        } else {
            value.extract()
        }
    }

    pub fn extract_filter(value: &Bound<'_, PyAny>) -> PyResult<Vec<String>> {
        if let Ok(tags) = value.cast::<Self>() {
            Ok(rc_ref!(&tags.borrow().inner).tags.iter().cloned().collect())
        } else if let Ok(tags) = value.cast::<PyFrozenSet>() {
            tags.iter().map(|tag| tag.extract()).collect()
        } else {
            value
                .cast::<PySet>()?
                .iter()
                .map(|tag| tag.extract())
                .collect()
        }
    }

    fn iterable(value: &Bound<'_, PyAny>) -> PyResult<HashSet<String>> {
        value.try_iter()?.map(|item| item?.extract()).collect()
    }

    fn iterables(values: &Bound<'_, PyTuple>) -> PyResult<Vec<HashSet<String>>> {
        values.iter().map(|value| Self::iterable(&value)).collect()
    }

    fn snapshot(&self) -> HashSet<String> {
        rc_ref!(&self.inner).tags.clone()
    }

    fn operand<'py>(value: &Bound<'py, PyAny>) -> PyResult<Bound<'py, PyAny>> {
        if let Ok(tags) = value.cast::<Self>() {
            let values = tags.borrow().snapshot();
            Ok(PySet::new(value.py(), values)?.into_any())
        } else {
            Ok(value.clone())
        }
    }
}

macro_rules! impl_tag_set_operators {
    ($(($method:ident, $reverse:ident, $in_place:ident, $operation:ident)),+ $(,)?) => {
        #[pymethods]
        impl Tags {
            $(
                fn $method(&self, other: &Bound<'_, PyAny>) -> PyResult<Py<PyAny>> {
                    let other = Self::operand(other)?;
                    let tags = PySet::new(other.py(), self.snapshot())?;
                    Ok(tags.call_method1(stringify!($method), (other,))?.unbind())
                }

                fn $reverse(&self, other: &Bound<'_, PyAny>) -> PyResult<Py<PyAny>> {
                    let other = Self::operand(other)?;
                    if other.cast::<PySet>().is_err() && other.cast::<PyFrozenSet>().is_err() {
                        return Ok(other.py().NotImplemented());
                    }
                    let tags = PySet::new(other.py(), self.snapshot())?;
                    Ok(other.call_method1(stringify!($method), (tags,))?.unbind())
                }

                fn $in_place(&self, other: &Bound<'_, PyAny>) -> PyResult<()> {
                    let other = Self::extract(other)?;
                    let result = rc_ref!(&self.inner).tags.$operation(&other).cloned().collect();
                    rc_mut!(&self.inner).tags = result;
                    Ok(())
                }
            )+
        }
    };
}

impl_tag_set_operators!(
    (__or__, __ror__, __ior__, union),
    (__and__, __rand__, __iand__, intersection),
    (__sub__, __rsub__, __isub__, difference),
    (__xor__, __rxor__, __ixor__, symmetric_difference),
);

#[pymethods]
impl Tags {
    fn __len__(&self) -> usize {
        rc_ref!(&self.inner).tags.len()
    }

    fn __contains__(&self, value: &str) -> bool {
        rc_ref!(&self.inner).tags.contains(value)
    }

    fn __iter__(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        let tags = PySet::new(py, self.snapshot())?;
        Ok(tags.call_method0("__iter__")?.unbind())
    }

    fn __repr__(&self, py: Python<'_>) -> PyResult<String> {
        Ok(PySet::new(py, self.snapshot())?.repr()?.to_string())
    }

    fn __richcmp__(&self, other: &Bound<'_, PyAny>, op: CompareOp) -> PyResult<Py<PyAny>> {
        let py = other.py();
        let other = if let Ok(tags) = other.cast::<Self>() {
            let values = tags.borrow().snapshot();
            PySet::new(py, values)?.into_any()
        } else {
            other.clone()
        };
        let tags = PySet::new(py, self.snapshot())?;
        Ok(tags.rich_compare(&other, op)?.unbind())
    }

    fn add(&self, value: String) {
        rc_mut!(&self.inner).tags.insert(value);
    }

    fn discard(&self, value: &str) {
        rc_mut!(&self.inner).tags.remove(value);
    }

    fn remove(&self, value: &str) -> PyResult<()> {
        if rc_mut!(&self.inner).tags.remove(value) {
            Ok(())
        } else {
            Err(PyKeyError::new_err(value.to_owned()))
        }
    }

    fn pop(&self) -> PyResult<String> {
        let mut node = rc_mut!(&self.inner);
        let value = node
            .tags
            .iter()
            .next()
            .cloned()
            .ok_or_else(|| PyKeyError::new_err("pop from an empty set"))?;
        node.tags.remove(&value);
        Ok(value)
    }

    fn clear(&self) {
        rc_mut!(&self.inner).tags.clear();
    }

    fn copy(&self) -> HashSet<String> {
        self.snapshot()
    }

    #[pyo3(signature = (*others))]
    fn update(&self, others: &Bound<'_, PyTuple>) -> PyResult<()> {
        let others = Self::iterables(others)?;
        rc_mut!(&self.inner)
            .tags
            .extend(others.into_iter().flatten());
        Ok(())
    }

    #[pyo3(signature = (*others))]
    fn difference_update(&self, others: &Bound<'_, PyTuple>) -> PyResult<()> {
        let others = Self::iterables(others)?;
        rc_mut!(&self.inner)
            .tags
            .retain(|tag| !others.iter().any(|other| other.contains(tag)));
        Ok(())
    }

    #[pyo3(signature = (*others))]
    fn intersection_update(&self, others: &Bound<'_, PyTuple>) -> PyResult<()> {
        let others = Self::iterables(others)?;
        rc_mut!(&self.inner)
            .tags
            .retain(|tag| others.iter().all(|other| other.contains(tag)));
        Ok(())
    }

    fn symmetric_difference_update(&self, other: &Bound<'_, PyAny>) -> PyResult<()> {
        let other = Self::iterable(other)?;
        let mut node = rc_mut!(&self.inner);
        for tag in other {
            if !node.tags.remove(&tag) {
                node.tags.insert(tag);
            }
        }
        Ok(())
    }

    #[pyo3(signature = (*others))]
    fn union(&self, others: &Bound<'_, PyTuple>) -> PyResult<HashSet<String>> {
        let others = Self::iterables(others)?;
        let mut result = self.snapshot();
        result.extend(others.into_iter().flatten());
        Ok(result)
    }

    #[pyo3(signature = (*others))]
    fn difference(&self, others: &Bound<'_, PyTuple>) -> PyResult<HashSet<String>> {
        let others = Self::iterables(others)?;
        let mut result = self.snapshot();
        result.retain(|tag| !others.iter().any(|other| other.contains(tag)));
        Ok(result)
    }

    #[pyo3(signature = (*others))]
    fn intersection(&self, others: &Bound<'_, PyTuple>) -> PyResult<HashSet<String>> {
        let others = Self::iterables(others)?;
        let mut result = self.snapshot();
        result.retain(|tag| others.iter().all(|other| other.contains(tag)));
        Ok(result)
    }

    fn symmetric_difference(&self, other: &Bound<'_, PyAny>) -> PyResult<HashSet<String>> {
        let other = Self::iterable(other)?;
        Ok(rc_ref!(&self.inner)
            .tags
            .symmetric_difference(&other)
            .cloned()
            .collect())
    }

    fn isdisjoint(&self, other: &Bound<'_, PyAny>) -> PyResult<bool> {
        let other = Self::iterable(other)?;
        Ok(rc_ref!(&self.inner).tags.is_disjoint(&other))
    }

    fn issubset(&self, other: &Bound<'_, PyAny>) -> PyResult<bool> {
        let other = Self::iterable(other)?;
        Ok(rc_ref!(&self.inner).tags.is_subset(&other))
    }

    fn issuperset(&self, other: &Bound<'_, PyAny>) -> PyResult<bool> {
        let other = Self::iterable(other)?;
        Ok(rc_ref!(&self.inner).tags.is_superset(&other))
    }
}
