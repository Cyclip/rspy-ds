mod queue;
mod circular_queue;
mod linked_list;
// Tree is not implemented yet (erroneous)
// mod tree;

use pyo3::prelude::*;

/// A Python module implemented in Rust.
#[pymodule]
fn rspy_ds(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<queue::Queue>()?;
    m.add_class::<circular_queue::CircularQueue>()?;
    m.add_class::<linked_list::LinkedList>()?;
    // m.add_class::<tree::Tree>()?;
    Ok(())
}