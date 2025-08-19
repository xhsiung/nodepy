#![deny(clippy::all)]

use napi_derive::napi;
// Removing pyo3 imports as the implementation is broken
use pyo3::prelude::*;
use pyo3::types::PyDict;
use std::ffi::CString;
use std::fs::File;
use std::io::Read;

use napi::threadsafe_function::{ThreadsafeFunction, ThreadsafeFunctionCallMode};
use napi::Task;

pub struct AsyncTask {
  fnstr: String,
  jstr: String,
}

#[napi]
impl Task for AsyncTask {
  type Output = String;
  type JsValue = String;

  fn compute(&mut self) -> napi::Result<Self::Output> {
    match get_py_run(self.fnstr.clone(), self.jstr.clone()) {
      Ok(result) => Ok(result),
      Err(e) => Err(napi::Error::new(napi::Status::GenericFailure, format!("Error: {}", e))),
    }
  }

  fn resolve(&mut self, _env: napi::Env, output: Self::Output) -> napi::Result<Self::JsValue> {
    Ok(output)
  }
}

fn get_py_run(fnstr: String, jstr: String) -> PyResult<String> {
    Python::with_gil(|py| {
        let locals = PyDict::new(py);
        let mut code = String::new();
        File::open("py/main.py")?.read_to_string(&mut code)?;

        let code_cstr = CString::new(code).unwrap();
        py.run(code_cstr.as_c_str(), None, Some(&locals))?;

        let main_fn = locals
            .get_item(&fnstr)?
            .ok_or_else(|| pyo3::exceptions::PyAttributeError::new_err(format!("Function '{}' not found", fnstr)))?;

        let result = main_fn.call1((jstr,))?.extract::<String>()?;

        Ok(result)
    })
}


#[napi]
pub fn do_sync(fnstr: String, jstr: String) -> String {
    match get_py_run(fnstr, jstr) {
        Ok(result) => result,
        Err(e) => format!("Error: {}", e),
    }
}

#[napi]
pub fn do_async_task(fnstr: String, jstr: String) -> napi::Result<String> {
  let mut task = AsyncTask { fnstr, jstr };
  let result = task.compute()?;
  Ok(result)
}

#[napi]
pub fn do_async_task_callback(
  fnstr: String,
  jstr: String,
  cb: ThreadsafeFunction<String>,
) -> napi::Result<()> {
  let mut task = AsyncTask { fnstr, jstr };
  let result = task.compute()?;
  cb.call(Ok(result), ThreadsafeFunctionCallMode::Blocking);
  Ok(())
}
