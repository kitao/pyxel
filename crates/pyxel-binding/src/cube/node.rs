use pyo3::prelude::*;

use pyxel::cube::Node as InnerNode;

use super::color_ramp::ColorRamp;
use super::light::Light;
use super::mat4::Mat4;

define_subclass_wrapper!(Node, pyxel::cube::Node);

#[pymethods]
impl Node {
    // Constructor

    #[new]
    fn new() -> Self {
        Self::wrap(InnerNode::new())
    }

    // Identification

    #[getter]
    fn name(&self) -> String {
        self.inner_ref().name.clone()
    }

    #[setter]
    fn set_name(&self, v: String) {
        self.inner_mut().name = v;
    }

    // Transform

    #[getter]
    fn transform(&self) -> Mat4 {
        Mat4::wrap(self.inner_ref().transform.clone())
    }

    #[setter]
    fn set_transform(&self, v: PyRef<'_, Mat4>) {
        self.inner_mut().transform = v.inner.clone();
    }

    // Cascade flags

    #[getter]
    fn active(&self) -> bool {
        self.inner_ref().active
    }

    #[setter]
    fn set_active(&self, v: bool) {
        self.inner_mut().active = v;
    }

    #[getter]
    fn visible(&self) -> bool {
        self.inner_ref().visible
    }

    #[setter]
    fn set_visible(&self, v: bool) {
        self.inner_mut().visible = v;
    }

    // Inheritable subtree state

    #[getter]
    fn light(&self) -> Option<Light> {
        self.inner_ref()
            .light
            .as_ref()
            .map(|l| Light::wrap(l.clone()))
    }

    #[setter]
    fn set_light(&self, v: Option<PyRef<'_, Light>>) {
        self.inner_mut().light = v.as_ref().map(|l| l.inner.clone());
    }

    #[getter]
    fn color_ramp(&self) -> Option<ColorRamp> {
        self.inner_ref()
            .color_ramp
            .as_ref()
            .map(|r| ColorRamp::wrap(r.clone()))
    }

    #[setter]
    fn set_color_ramp(&self, v: Option<PyRef<'_, ColorRamp>>) {
        self.inner_mut().color_ramp = v.as_ref().map(|r| r.inner.clone());
    }

    // Hierarchy (read-only properties)

    #[getter]
    fn parent(&self) -> Option<Node> {
        InnerNode::parent(&self.inner).map(Node::wrap)
    }

    #[getter]
    fn children(&self) -> Vec<Node> {
        InnerNode::children(&self.inner)
            .into_iter()
            .map(Node::wrap)
            .collect()
    }

    // Methods

    fn world_transform(&self) -> Mat4 {
        Mat4::wrap(InnerNode::world_transform(&self.inner))
    }

    fn find(&self, name: &str) -> Option<Node> {
        InnerNode::find(&self.inner, name).map(Node::wrap)
    }

    fn add_child(&self, child: PyRef<'_, Node>) {
        InnerNode::add_child(&self.inner, &child.inner);
    }

    fn remove_child(&self, child: PyRef<'_, Node>) {
        InnerNode::remove_child(&self.inner, &child.inner);
    }

    fn destroy(&self) {
        InnerNode::destroy(&self.inner);
    }

    // Dunder

    fn __repr__(&self) -> String {
        let n = self.inner_ref();
        format!("Node(name={:?}, children={})", n.name, n.children.len())
    }
}

pub fn add_node_class(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Node>()?;
    Ok(())
}
