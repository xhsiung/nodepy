#![deny(clippy::all)]

use napi_derive::napi;
// Removing pyo3 imports as the implementation is broken
use pyo3::prelude::*;
use pyo3::types::PyDict;
use std::ffi::CString;
use std::fs::File;
use std::io::Read;
use napi::bindgen_prelude::*;

use napi::threadsafe_function::{ThreadsafeFunction, ThreadsafeFunctionCallMode};
use napi::Task;

pub struct Work {
  fnstr: String,
  jstr: String,
  callback: Option<ThreadsafeFunction<String>>,
}

#[napi]
impl Task for Work {
  type Output = String;
  type JsValue = String;

  fn compute(&mut self) -> napi::Result<Self::Output> {
    match get_py_run(self.fnstr.clone(), self.jstr.clone()) {
      Ok(result) => Ok(result),
      Err(e) => Err(napi::Error::new(napi::Status::GenericFailure, format!("Error: {}", e))),
    }
  }

  fn resolve(&mut self, _env: napi::Env, output: Self::Output) -> napi::Result<Self::JsValue> {
    if let Some(callback) = &self.callback {
      callback.call(Ok(output.clone()), ThreadsafeFunctionCallMode::Blocking);
    }
    Ok(output)
  }
}

fn get_py_run(fnstr: String, jstr: String) -> PyResult<String> {
    Python::with_gil(|py| {
        let locals = PyDict::new(py);
        let mut code = String::new();
        File::open("py/__init__.py")?.read_to_string(&mut code)?;

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
pub fn do_async_task(fnstr: String, jstr: String) -> AsyncTask<Work> {
  AsyncTask::new(Work {
    fnstr,
    jstr,
    callback: None,
  })
}

#[napi]
pub fn do_async_task_callback(
  fnstr: String,
  jstr: String,
  callback: ThreadsafeFunction<String>,
) -> AsyncTask<Work> {
  AsyncTask::new(Work {
    fnstr,
    jstr,
    callback: Some(callback),
  })
}


