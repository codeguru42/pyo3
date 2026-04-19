use crate::cpython::pystate::Py_tracefunc;
use crate::object::PyObject;

extern_libpython! {
    // skipped non-limited _PyEval_CallTracing

    // skipped private _PyEval_EvalFrameDefault

    // skipped private _PyEval_RequestCodeExtraIndex

    pub fn PyEval_SetProfile(trace_func: Option<Py_tracefunc>, arg1: *mut PyObject);
    pub fn PyEval_SetTrace(trace_func: Option<Py_tracefunc>, arg1: *mut PyObject);
}
