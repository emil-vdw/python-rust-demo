use pyo3::{
    exceptions::{PyKeyError, PyValueError},
    prelude::*,
    types::{PyDict, PyList},
};
use uuid::Uuid;

#[pyclass]
#[derive(Clone)]
pub enum Supplier {
    Edeka,
    SystemU,
    GS1,
}

#[pyclass]
#[derive(Clone)]
pub struct Ingredient {
    name: String,
}

#[pyclass]
#[derive(Clone)]
pub struct Product {
    id: Uuid,
    supplier: Supplier,
    gtin: String,
    description: String,
    ingredients: Vec<Ingredient>,
}

#[pyclass]
pub struct ProductRef {
    #[pyo3(get)]
    product: Py<Product>,
}

#[pymethods]
impl Product {
    #[new]
    pub fn __new__<'py>(
        supplier: Supplier,
        gtin: &str,
        description: &str,
        ingredients: &Bound<'py, PyList>,
    ) -> PyResult<Self> {
        let ingredients: Vec<Ingredient> = ingredients
            .iter()
            .map(|ingredient_dict| ingredient_dict.extract::<Ingredient>())
            .collect::<Result<Vec<Ingredient>, PyErr>>()?;

        Ok(Self {
            id: Uuid::new_v4(),
            supplier,
            gtin: gtin.to_owned(),
            description: description.to_owned(),
            ingredients,
        })
    }
}

// impl<'py> FromPyObject<'py> for Ingredient {
//     fn extract_bound(ingredient_dict: &Bound<'py, PyAny>) -> PyResult<Self> {
//         let dict: &Bound<PyDict> = ingredient_dict.downcast()?;

//         Ok(Self {
//             name: dict.get_item("name").and_then(|v| {
//                 if let Some(ingredient_name) = v {
//                     let name: String = ingredient_name.extract()?;
//                     Ok(name)
//                 } else {
//                     Err(PyValueError::new_err("Ingredient name must be a string"))
//                 }
//             })?,
//         })
//     }
// }

#[pymethods]
impl ProductRef {
    #[getter]
    fn inner(&self, py: Python<'_>) -> Py<Product> {
        self.product.clone_ref(py)
    }
    
    #[new]
    fn __new__<'py>(
        py: Python<'_>,
        supplier: Supplier,
        gtin: &str,
        description: &str,
        ingredients: &Bound<'py, PyList>,
    ) -> PyResult<Self> {
        Ok(Self {
            product: Py::new(py, Product::__new__(supplier, gtin, description, ingredients)?)?,
        })
    }
}
