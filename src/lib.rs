use pyo3::prelude::*;

/// Calculate matrix determinant using LU decomposition
#[pyfunction]
fn determinant(matrix: Vec<Vec<f64>>) -> PyResult<f64> {
    let n = matrix.len();
    
    // Check if matrix is square
    if matrix.iter().any(|row| row.len() != n) {
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
            "Matrix must be square"
        ));
    }

    // Special cases for small matrices
    if n == 0 {
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
            "Empty matrix"
        ));
    }
    if n == 1 {
        return Ok(matrix[0][0]);
    }
    if n == 2 {
        return Ok(matrix[0][0] * matrix[1][1] - matrix[0][1] * matrix[1][0]);
    }
    
    // Create mutable copy for LU decomposition
    let mut lu = matrix.clone();
    
    // Keep track of row swaps for determinant sign
    let mut sign = 1.0;
    
    // LU decomposition with partial pivoting
    for k in 0..n-1 {
        // Find pivot (largest element in current column)
        let mut pivot_row = k;
        let mut pivot_val = lu[k][k].abs();
        
        for i in k+1..n {
            let val = lu[i][k].abs();
            if val > pivot_val {
                pivot_val = val;
                pivot_row = i;
            }
        }
        
        // Check for singularity
        if pivot_val < 1e-10 {
            return Ok(0.0);
        }
        
        // Swap rows if necessary
        if pivot_row != k {
            lu.swap(k, pivot_row);
            sign = -sign;
        }
        
        // Perform elimination
        for i in k+1..n {
            let factor = lu[i][k] / lu[k][k];
            lu[i][k] = factor;  // Store multiplier
            
            for j in k+1..n {
                lu[i][j] -= factor * lu[k][j];
            }
        }
    }
    
    // Compute determinant as product of diagonal elements
    let det = (0..n).fold(sign, |acc, i| acc * lu[i][i]);
    
    Ok(det)
}

/// Calculate determinant using Laplace expansion (for small matrices)
#[pyfunction]
fn determinant_laplace(matrix: Vec<Vec<f64>>) -> PyResult<f64> {
    let n = matrix.len();
    
    // Check if matrix is square
    if matrix.iter().any(|row| row.len() != n) {
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
            "Matrix must be square"
        ));
    }

    // Special cases
    if n == 0 {
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
            "Empty matrix"
        ));
    }
    if n == 1 {
        return Ok(matrix[0][0]);
    }
    if n == 2 {
        return Ok(matrix[0][0] * matrix[1][1] - matrix[0][1] * matrix[1][0]);
    }
    
    // Laplace expansion along first row
    let mut det = 0.0;
    let mut sign = 1.0;
    
    for j in 0..n {
        // Create minor matrix by excluding first row and current column
        let minor: Vec<Vec<f64>> = matrix.iter()
            .skip(1)
            .map(|row| {
                row.iter()
                   .enumerate()
                   .filter(|&(i, _)| i != j)
                   .map(|(_, &x)| x)
                   .collect()
            })
            .collect();
        
        // Recursive calculation
        let sub_det = determinant_laplace(minor)?;
        det += sign * matrix[0][j] * sub_det;
        sign = -sign;
    }
    
    Ok(det)
}

/// Main entry point for the Rust implementation
#[pymodule]
fn _madet<'py>(_py: Python<'py>, m: &Bound<'py, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(determinant, m)?)?;
    m.add_function(wrap_pyfunction!(determinant_laplace, m)?)?;
    Ok(())
}
