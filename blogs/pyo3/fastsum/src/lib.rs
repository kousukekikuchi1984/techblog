use pyo3::prelude::*;

#[pyfunction]
pub fn rs_sum(n: u64) -> u64 {
    (0..n).filter(|x| x % 3 == 0).sum()
}

#[pymodule]
fn fastsum(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(rs_sum, m)?)?;
    Ok(())
}
