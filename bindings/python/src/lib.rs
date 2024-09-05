use chrono::NaiveDate;
use human_chrono_parser::{locales::Locale, HumanDateExpr};
use pyo3::{
    exceptions::{PyRuntimeError, PyValueError},
    prelude::*,
};

#[pyfunction]
fn parse(input: String, locale_name: String) -> PyResult<PyHumanDateExpr> {
    let locale = get_locale(&locale_name)?;
    human_chrono_parser::parse(&mut input.as_str(), &locale)
        .map(|expr| PyHumanDateExpr { inner: expr })
        .map_err(|err| PyRuntimeError::new_err(format!("{}", err)))
}

fn get_locale(locale_name: &String) -> PyResult<Locale> {
    match locale_name.as_ref() {
        "pt-BR" => Ok(Locale::BrazilianPortuguese),
        "en" => Ok(Locale::English),
        _ => Err(PyValueError::new_err(format!(
            "Unknown locale: {}",
            locale_name
        ))),
    }
}

#[pyfunction]
fn extract_all(input: String, locale_name: String) -> PyResult<Vec<PyHumanDateExpr>> {
    let locale = get_locale(&locale_name)?;
    Ok(
        human_chrono_parser::extract_all(&mut input.as_str(), &locale)
            .into_iter()
            .map(|expr| PyHumanDateExpr { inner: expr })
            .collect(),
    )
}

#[pyclass(name = "HumanDateExpr", eq)]
#[derive(PartialEq)]
struct PyHumanDateExpr {
    inner: HumanDateExpr,
}

#[pymethods]
impl PyHumanDateExpr {
    pub fn relative_to(&self, now: NaiveDate) -> PyResult<Option<NaiveDate>> {
        Ok(self.inner.relative_to(&now))
    }
}

#[pymodule(name = "human_chrono_parser")]
fn human_chrono_parser_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(parse, m)?)?;
    m.add_function(wrap_pyfunction!(extract_all, m)?)?;
    Ok(())
}
