use pyo3::prelude::*;

// procedural attribute macro from pyo3 crate (turns normal rust functions into python functions)
#[pyfunction]
fn square(x: f64) -> f64 {
    x * x
}

// how we register the functions we wrote that we want to expose to python
#[pymodule]
fn rustkern_native(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(square, m)?)?;
    Ok(())
}

/*

whole concept of FFI (foreign function interface) is what makes this possible
- translation layer between 2 languages
- languages generally disagree on a lot of things: types, errors, memory, etc. so FFI is difficult to make
- generally speaking C ABI is the agreed upon "middleman"
- this doesn't mean the code is actually converted to C, but the functions in rust are compiled so it follows machine level rules that C ABI sets

pretty cool, an ABI (application binary interface) is like a few levels lower than an API
ABI is contracts on the machine code level

*/