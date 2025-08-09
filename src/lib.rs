use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;

pub mod mdict;
pub mod util;

use mdict::mdx::Mdx;

#[pyclass]
pub struct PyMdx {
    inner: Mdx,
}

#[pyclass]
pub struct PyRecord {
    #[pyo3(get)]
    pub text: String,
    #[pyo3(get)]
    pub definition: String,
}

#[pymethods]
impl PyMdx {
    #[new]
    fn new(data: &[u8]) -> PyResult<Self> {
        let mdx = Mdx::new(data);
        Ok(PyMdx { inner: mdx })
    }

    fn items(&self) -> Vec<PyRecord> {
        self.inner
            .items()
            .map(|record| PyRecord {
                text: record.text.to_string(),
                definition: record.definition,
            })
            .collect()
    }

    fn get_entries_count(&self) -> usize {
        self.inner.records_offset.len()
    }
}

#[pyfunction]
fn parse_mdx_file(file_path: &str) -> PyResult<PyMdx> {
    let data = std::fs::read(file_path)
        .map_err(|e| PyValueError::new_err(format!("Failed to read file: {}", e)))?;
    PyMdx::new(&data)
}

#[pyfunction]
fn parse_mdx_bytes(data: &[u8]) -> PyResult<PyMdx> {
    PyMdx::new(data)
}

#[pymodule]
fn mdict_rs(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyMdx>()?;
    m.add_class::<PyRecord>()?;
    m.add_function(wrap_pyfunction!(parse_mdx_file, m)?)?;
    m.add_function(wrap_pyfunction!(parse_mdx_bytes, m)?)?;
    Ok(())
}