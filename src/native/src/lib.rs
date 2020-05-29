use neon::prelude::*;
use pyo3::prelude::*;
use pyo3::types::*;
use pyo3::{py_run, wrap_pyfunction };
use std::fs::File;
use std::io::Read;

fn getPyRun( fnstr: String, jstr: String )->String{
    let gil = Python::acquire_gil();
    let py = gil.python();
    let locals = PyDict::new(py);
    //locals.set_item("myname","alex");

    let mut ff = File::open("py/main.py").unwrap();
    let mut code = String::new();
    ff.read_to_string(&mut code);

    py.run(
        code.as_str(),
        None,
        Some(locals)).unwrap();

    //get method
    let main = locals.get_item( fnstr ).unwrap().to_object(py);
    let result = main.call1(py,( jstr, )).unwrap().extract::<String>(py).unwrap();
    format!("{}" , result )
}


struct MyTask{
    argument: String,
    fnstr: String
}

impl Task for MyTask {
    type Output = String;
    type Error = ();
    type JsEvent = JsString;

    fn perform(&self) -> Result<Self::Output, Self::Error> {
        let ojstr = getPyRun(  self.fnstr.clone(),self.argument.clone()  );
        Ok(  ojstr )
    }

    fn complete<'a>(self, mut cx: TaskContext<'a>, result: Result<Self::Output, Self::Error>) -> JsResult<'_, Self::JsEvent> {
        Ok( cx.string( result.unwrap() ) )
    }
}


fn doSync(mut cx: FunctionContext) -> JsResult<JsString> {
    let fnstr = cx.argument::<JsString>(0)?.value();
    let jstr = cx.argument::<JsString>(1)?.value();
    let ojstr = getPyRun( fnstr,jstr );

    Ok(cx.string(ojstr ))
}

fn doAsync(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let fnstr = cx.argument::<JsString>(0)?.value();
    let jstr = cx.argument::<JsString>(1)?.value();
    let cb = cx.argument::<JsFunction>(2)?;
    let task = MyTask { argument: jstr , fnstr: fnstr };
    task.schedule( cb );

    Ok( cx.undefined())
}

//neon start
register_module!(mut cx, {
   cx.export_function("doSync" , doSync);
   cx.export_function("doAsync" , doAsync);
   Ok(())
});
