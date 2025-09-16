use chrono::NaiveDate;
use pyo3::prelude::*;

use crate::fixed_income::{Bond, DayCount, DayCountConvention, ZeroCouponBond};

// =============================================================================
// MAIN PYTHON MODULE
// =============================================================================

#[pymodule]
pub fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Fixed Income
    m.add_class::<PyDayCount>()?;
    m.add_class::<PyZeroCouponBond>()?;

    // Module-level convenience function
    #[pyfn(m)]
    fn calculate_year_fraction(start: &str, end: &str, convention: &str) -> PyResult<f64> {
        let day_count = PyDayCount::new(convention)?;
        day_count.year_fraction(start, end)
    }

    Ok(())
}

// =============================================================================
// DAYCOUNT BINDINGS
// =============================================================================

#[pyclass(name = "DayCount")]
#[derive(Clone)]
pub struct PyDayCount {
    inner: DayCount,
}

#[pymethods]
impl PyDayCount {
    #[new]
    pub fn new(convention: &str) -> PyResult<Self> {
        let inner = match convention.to_uppercase().as_str() {
            "ACT/365F" | "ACT365F" => DayCount::Act365F,
            "ACT/365" | "ACT365" => DayCount::Act365,
            "ACT/360" | "ACT360" => DayCount::Act360,
            "30/360US" | "30360US" => DayCount::Thirty360US,
            "30/360E" | "30360E" => DayCount::Thirty360E,
            "ACT/ACT ISDA" | "ACTACTISDA" => DayCount::ActActISDA,
            "ACT/ACT ICMA" | "ACTACTICMA" => DayCount::ActActICMA,
            _ => {
                return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                    "Unknown day count convention: {}",
                    convention
                )))
            }
        };
        Ok(PyDayCount { inner })
    }

    pub fn year_fraction(&self, start: &str, end: &str) -> PyResult<f64> {
        let start_date = NaiveDate::parse_from_str(start, "%Y-%m-%d").map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Invalid start date: {}", e))
        })?;
        let end_date = NaiveDate::parse_from_str(end, "%Y-%m-%d").map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Invalid end date: {}", e))
        })?;
        Ok(self.inner.year_fraction(start_date, end_date))
    }

    pub fn day_count(&self, start: &str, end: &str) -> PyResult<i32> {
        let start_date = NaiveDate::parse_from_str(start, "%Y-%m-%d").map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Invalid start date: {}", e))
        })?;
        let end_date = NaiveDate::parse_from_str(end, "%Y-%m-%d").map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Invalid end date: {}", e))
        })?;
        Ok(self.inner.day_count(start_date, end_date))
    }

    fn __repr__(&self) -> String {
        format!("DayCount({:?})", self.inner)
    }
}

// =============================================================================
// ZERO COUPON BINDINGS
// =============================================================================

#[pyclass(name = "ZeroCouponBond")]
pub struct PyZeroCouponBond {
    inner: ZeroCouponBond,
}

#[pymethods]
impl PyZeroCouponBond {
    #[new]
    pub fn new(face_value: f64, maturity: &str) -> PyResult<Self> {
        let maturity_date = NaiveDate::parse_from_str(maturity, "%Y-%m-%d").map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Invalid maturity date: {}", e))
        })?;
        Ok(PyZeroCouponBond {
            inner: ZeroCouponBond::new(face_value, maturity_date),
        })
    }

    pub fn price(&self, settlement: &str, ytm: f64, day_count: &PyDayCount) -> PyResult<f64> {
        let settlement_date = NaiveDate::parse_from_str(settlement, "%Y-%m-%d").map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                "Invalid settlement date: {}",
                e
            ))
        })?;
        match self.inner.price(settlement_date, ytm, day_count.inner) {
            Ok(price_result) => Ok(price_result.clean),
            Err(e) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                "Pricing error: {}",
                e
            ))),
        }
    }

    #[getter]
    pub fn face_value(&self) -> f64 {
        self.inner.face_value
    }

    #[getter]
    pub fn maturity(&self) -> String {
        self.inner.maturity.format("%Y-%m-%d").to_string()
    }

    fn __repr__(&self) -> String {
        format!(
            "ZeroCouponBond(face_value={}, maturity={})",
            self.inner.face_value,
            self.inner.maturity.format("%Y-%m-%d")
        )
    }
}
