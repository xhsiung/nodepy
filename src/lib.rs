#![deny(clippy::all)]

use napi_derive::napi;
// Removing pyo3 imports as the implementation is broken
use pyo3::prelude::*;
use pyo3::types::PyDict;
use std::ffi::CString;
use std::fs::File;
use std::io::Read;

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
pub async fn do_async(fnstr: String, jstr: String) -> String {
    match get_py_run(fnstr, jstr) {
        Ok(result) => result,
        Err(e) => format!("Error: {}", e),
    }
}

#[napi(ts_args_type = "fnstr: string, jstr: string, cb: (err: null | Error, result: string) => void")]
pub fn do_sync_callback(
  fnstr: String,
  jstr: String,
  cb: napi::threadsafe_function::ThreadsafeFunction<String>,
) {
  std::thread::spawn(move || {
    match get_py_run(fnstr, jstr) {
      Ok(result) => cb.call(Ok(result), napi::threadsafe_function::ThreadsafeFunctionCallMode::Blocking),
      Err(e) => cb.call(Err(napi::Error::new(napi::Status::GenericFailure, format!("Error: {}", e))), napi::threadsafe_function::ThreadsafeFunctionCallMode::Blocking),
    };
  });
}

#[napi(ts_args_type = "fnstr: string, jstr: string, cb: (err: null | Error, result: string) => void")]
pub async fn do_async_callback(
  fnstr: String,
  jstr: String,
  cb: napi::threadsafe_function::ThreadsafeFunction<String>,
) {
  std::thread::spawn(move || {
    match get_py_run(fnstr, jstr) {
      Ok(res) => cb.call(Ok(res), napi::threadsafe_function::ThreadsafeFunctionCallMode::Blocking),
      Err(e) => cb.call(
        Err(napi::Error::new(napi::Status::GenericFailure, format!("Error: {}", e))),
        napi::threadsafe_function::ThreadsafeFunctionCallMode::Blocking,
      ),
    };
  });
}
