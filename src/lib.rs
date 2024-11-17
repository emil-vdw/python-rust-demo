use pyo3::prelude::*;

mod classes;
mod functions;

/// Main entry point for the Rust implementation
#[pymodule]
mod _internal {
    use super::*;

    #[pymodule_init]
    fn init(m: &Bound<'_, PyModule>) -> PyResult<()> {
        // Arbitrary code to run at the module initialization
        functions::register_module(m).expect("Failed to create functions module");
        Ok(())
    }

    #[pymodule]
    pub mod classes {
        use super::*;

        #[pymodule_export]
        use crate::classes::Supplier;

        #[pymodule_export]
        use crate::classes::Ingredient;

        #[pymodule_export]
        use crate::classes::ProductRef;

        #[pymodule_export]
        use crate::classes::Product;
    }
}
