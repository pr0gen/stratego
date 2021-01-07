use pyo3::GILGuard;
use pyo3::Python;

use crate::error::StrategoError;
use crate::py_bindings::load_stratego_ai_module;

pub fn get_gild_holder() -> Result<GilHolder, StrategoError> {
    GilHolder::new()
}

pub struct GilHolder(GILGuard);

impl GilHolder {
    fn new() -> Result<Self, StrategoError> {
        let gil: GILGuard = Python::acquire_gil();
        if let Err(e) = load_stratego_ai_module(&gil.python()) {
            Err(e)
        } else {
            Ok(GilHolder(gil))
        }
    }

    pub fn get(&self) -> &GILGuard {
        &self.0
    }
}
