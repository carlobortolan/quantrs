// pub mod data;
pub mod fixed_income;
// pub mod options;

use pyo3::prelude::*;

#[pymodule]
fn quantrs(m: &Bound<'_, PyModule>) -> PyResult<()> {
    fixed_income::register(m)?;
    // options::register(m)?;
    // data::register(m)?;
    Ok(())
}
