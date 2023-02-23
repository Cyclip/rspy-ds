mod queue;

use pyo3::prelude::*;

/// A Python module implemented in Rust.
#[pymodule]
fn rspy_ds(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<queue::Queue>()?;
    Ok(())
}