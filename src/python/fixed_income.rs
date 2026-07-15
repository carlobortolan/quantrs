use chrono::NaiveDate;
use pyo3::prelude::*;

use crate::fixed_income::{Bond, CorporateBond, DayCount, DayCountConvention, ZeroCouponBond};

// =============================================================================
// MAIN PYTHON MODULE
// =============================================================================

#[pymodule]
pub fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Fixed Income
    m.add_class::<PyDayCount>()?;
    m.add_class::<PyPriceResult>()?;
    m.add_class::<PyZeroCouponBond>()?;
    m.add_class::<PyCorporateBond>()?;

    // Convenience function
    m.add_function(wrap_pyfunction!(calculate_year_fraction, m)?)?;

    Ok(())
}

#[pyfunction]
fn calculate_year_fraction(start: &str, end: &str, convention: &str) -> PyResult<f64> {
    let day_count = PyDayCount::new(convention)?;
    day_count.year_fraction(start, end)
}

// =============================================================================
// PRICE RESULT BINDINGS
// =============================================================================

#[pyclass(name = "PriceResult", from_py_object)]
#[derive(Clone)]
pub struct PyPriceResult {
    #[pyo3(get)]
    pub clean: f64,

    #[pyo3(get)]
    pub dirty: f64,

    #[pyo3(get)]
    pub accrued: f64,
}

impl From<crate::fixed_income::PriceResult> for PyPriceResult {
    fn from(value: crate::fixed_income::PriceResult) -> Self {
        Self {
            clean: value.clean,
            dirty: value.dirty,
            accrued: value.accrued,
        }
    }
}

// =============================================================================
// DAY COUNT BINDINGS
// =============================================================================

#[pyclass(name = "DayCount", from_py_object)]
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
            "ACT/360" | "ACT360" => DayCount::Act360,
            "30/360US" | "30360US" => DayCount::Thirty360US,
            "30/360E" | "30360E" => DayCount::Thirty360E,
            "ACT/ACT ISDA" | "ACTACTISDA" => DayCount::ActActISDA,
            "ACT/ACT ICMA" | "ACTACTICMA" => DayCount::ActActICMA,
            _ => {
                return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                    "Unknown day count convention: {}",
                    convention
                )));
            }
        };

        Ok(Self { inner })
    }

    pub fn year_fraction(&self, start: &str, end: &str) -> PyResult<f64> {
        if matches!(self.inner, DayCount::ActActICMA) {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "ACT/ACT ICMA requires coupon period dates and frequency",
            ));
        }

        let start_date = NaiveDate::parse_from_str(start, "%Y-%m-%d").map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Invalid start date: {}", e))
        })?;

        let end_date = NaiveDate::parse_from_str(end, "%Y-%m-%d").map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Invalid end date: {}", e))
        })?;

        Ok(self.inner.year_fraction(start_date, end_date))
    }

    pub fn day_count(&self, start: &str, end: &str) -> PyResult<u32> {
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
// ZERO COUPON BOND BINDINGS
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

        Ok(Self {
            inner: ZeroCouponBond::new(face_value, maturity_date),
        })
    }

    pub fn price(
        &self,
        settlement: &str,
        ytm: f64,
        day_count: &PyDayCount,
    ) -> PyResult<PyPriceResult> {
        let settlement_date = NaiveDate::parse_from_str(settlement, "%Y-%m-%d").map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                "Invalid settlement date: {}",
                e
            ))
        })?;

        self.inner
            .price(settlement_date, ytm, day_count.inner)
            .map(PyPriceResult::from)
            .map_err(|e| {
                PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Pricing error: {}", e))
            })
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
            self.inner.face_value, self.inner.maturity
        )
    }
}

// =============================================================================
// CORPORATE BOND BINDINGS
// =============================================================================

#[pyclass(name = "CorporateBond")]
pub struct PyCorporateBond {
    inner: CorporateBond,
}

#[pymethods]
impl PyCorporateBond {
    #[new]
    pub fn new(
        face_value: f64,
        coupon_rate: f64,
        issue_date: &str,
        maturity: &str,
        frequency: u32,
        credit_rating: String,
    ) -> PyResult<Self> {
        let issue_date = NaiveDate::parse_from_str(issue_date, "%Y-%m-%d").map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Invalid issue date: {}", e))
        })?;

        let maturity = NaiveDate::parse_from_str(maturity, "%Y-%m-%d").map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Invalid maturity date: {}", e))
        })?;

        Ok(Self {
            inner: CorporateBond::new(
                face_value,
                coupon_rate,
                issue_date,
                maturity,
                frequency,
                credit_rating,
            ),
        })
    }

    pub fn price(
        &self,
        settlement: &str,
        ytm: f64,
        day_count: &PyDayCount,
    ) -> PyResult<PyPriceResult> {
        let settlement = NaiveDate::parse_from_str(settlement, "%Y-%m-%d").map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                "Invalid settlement date: {}",
                e
            ))
        })?;

        self.inner
            .price(settlement, ytm, day_count.inner)
            .map(PyPriceResult::from)
            .map_err(|e| {
                PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Pricing error: {}", e))
            })
    }

    pub fn accrued_interest(&self, settlement: &str, day_count: &PyDayCount) -> PyResult<f64> {
        let settlement = NaiveDate::parse_from_str(settlement, "%Y-%m-%d").map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                "Invalid settlement date: {}",
                e
            ))
        })?;

        Ok(self.inner.accrued_interest(settlement, day_count.inner))
    }

    pub fn credit_spread(&self) -> f64 {
        self.inner.credit_spread()
    }

    #[getter]
    pub fn face_value(&self) -> f64 {
        self.inner.face_value
    }

    #[getter]
    pub fn coupon_rate(&self) -> f64 {
        self.inner.coupon_rate
    }

    #[getter]
    pub fn issue_date(&self) -> String {
        self.inner.issue_date.format("%Y-%m-%d").to_string()
    }

    #[getter]
    pub fn maturity(&self) -> String {
        self.inner.maturity.format("%Y-%m-%d").to_string()
    }

    #[getter]
    pub fn frequency(&self) -> u32 {
        self.inner.frequency
    }

    #[getter]
    pub fn credit_rating(&self) -> String {
        self.inner.credit_rating.clone()
    }

    fn __repr__(&self) -> String {
        format!(
            "CorporateBond(face_value={}, coupon_rate={}, maturity={})",
            self.inner.face_value, self.inner.coupon_rate, self.inner.maturity
        )
    }
}
