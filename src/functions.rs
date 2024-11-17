use std::collections::HashMap;

use pyo3::prelude::*;

use pyo3::{exceptions::PyValueError, pyfunction, wrap_pyfunction};

#[pyfunction]
fn fibonacci(n: usize) -> usize {
    let mut fib_memo: HashMap<usize, usize> = HashMap::with_capacity(n);

    if n < 1 {
        return n;
    }

    let mut prev: usize = 0;
    let mut cur: usize = 1;

    for i in 2..=n {
        let new = cur + prev;
        prev = cur;
        cur = new;
    }

    cur
}

#[pyfunction]
fn check_positive(x: i32) -> PyResult<()> {
    if x < 0 {
        Err(PyValueError::new_err("x is negative"))
    } else {
        Ok(())
    }
}

pub fn register_module(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    let functions_module = PyModule::new(parent_module.py(), "functions")?;

    functions_module.add_function(wrap_pyfunction!(fibonacci, &functions_module)?)?;
    functions_module.add_function(wrap_pyfunction!(check_positive, &functions_module)?)?;

    parent_module.add_submodule(&functions_module)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci_zero() {
        assert_eq!(fibonacci(0), 0);
    }

    #[test]
    fn test_fibonacci_one() {
        assert_eq!(fibonacci(1), 1);
    }

    #[test]
    fn test_fibonacci_two() {
        assert_eq!(fibonacci(2), 1);
    }

    #[test]
    fn test_fibonacci_small_numbers() {
        assert_eq!(fibonacci(3), 2);
        assert_eq!(fibonacci(4), 3);
        assert_eq!(fibonacci(5), 5);
        assert_eq!(fibonacci(6), 8);
        assert_eq!(fibonacci(7), 13);
    }

    #[test]
    fn test_fibonacci_larger_numbers() {
        assert_eq!(fibonacci(10), 55);
        assert_eq!(fibonacci(15), 610);
        assert_eq!(fibonacci(20), 6765);
    }

    // Property-based tests
    #[test]
    fn test_fibonacci_sequence_property() {
        // Test that each number is the sum of the two preceding ones
        let mut prev = fibonacci(0);
        let mut curr = fibonacci(1);

        for n in 2..20 {
            let next = fibonacci(n);
            assert_eq!(
                prev + curr,
                next,
                "Failed sequence property at position {}",
                n
            );
            prev = curr;
            curr = next;
        }
    }

    // Benchmark tests (requires nightly Rust)
    #[test]
    fn benchmark_fibonacci() {
        for i in 0..30 {
            fibonacci(i); // Simple performance check
        }
    }
}
