use std::sync::{Mutex, Once};

use crate::{HashMapContext, ContextWithMutableVariables, ContextWithMutableFunctions, Value, EvalexprResult, Function, Context};

static mut GLOBAL_CONTEXT: Option<Mutex<HashMapContext>> = None;
static INIT: Once = Once::new();

fn get_context() -> &'static Mutex<HashMapContext> {
    unsafe {
        INIT.call_once(|| {
            GLOBAL_CONTEXT = Some(Mutex::new(HashMapContext::new()));
        });
        GLOBAL_CONTEXT.as_ref().expect("Global context not initialized")
    }
}

pub fn context() -> &'static Mutex<HashMapContext> {
    get_context()
}

pub fn with_context<R, F: FnOnce(&HashMapContext) -> R>(f: F) -> R {
    let guard = get_context().lock().unwrap();
    f(&*guard)
}

pub fn with_context_mut<R, F: FnOnce(&mut HashMapContext) -> R>(f: F) -> R {
    let mut guard = get_context().lock().unwrap();
    f(&mut *guard)
}

pub fn clear() {
    with_context_mut(|ctx| ctx.clear());
}

pub fn set_value(identifier: String, value: Value) -> EvalexprResult<()> {
    with_context_mut(|ctx| ctx.set_value(identifier, value))
}

pub fn set_function(identifier: String, function: Function) -> EvalexprResult<()> {
    with_context_mut(|ctx| ctx.set_function(identifier, function))
}

pub fn get_value_copy(identifier: &str) -> Option<Value> {
    with_context(|ctx| ctx.get_value(identifier).cloned())
}

use crate::error::EvalexprError;

pub fn call_function_copy(identifier: &str, argument: &Value) -> EvalexprResult<Option<Value>> {
    with_context(|ctx| match ctx.call_function(identifier, argument) {
        Ok(v) => Ok(Some(v)),
        Err(EvalexprError::FunctionIdentifierNotFound(_)) => Ok(None),
        Err(e) => Err(e),
    })
}

impl Context for &'static Mutex<HashMapContext> {
    fn get_value(&self, identifier: &str) -> Option<&Value> {
        let guard = self.lock().unwrap();
        guard.get_value(identifier)
    }

    fn call_function(&self, identifier: &str, argument: &Value) -> EvalexprResult<Value> {
        let guard = self.lock().unwrap();
        guard.call_function(identifier, argument)
    }

    fn are_builtin_functions_disabled(&self) -> bool {
        let guard = self.lock().unwrap();
        guard.are_builtin_functions_disabled()
    }

    fn set_builtin_functions_disabled(&mut self, disabled: bool) -> EvalexprResult<()> {
        let mut guard = self.lock().unwrap();
        guard.set_builtin_functions_disabled(disabled)
    }
}

impl ContextWithMutableVariables for &'static Mutex<HashMapContext> {
    fn set_value(&mut self, identifier: String, value: Value) -> EvalexprResult<()> {
        let mut guard = self.lock().unwrap();
        guard.set_value(identifier, value)
    }
}

impl ContextWithMutableFunctions for &'static Mutex<HashMapContext> {
    fn set_function(&mut self, identifier: String, function: Function) -> EvalexprResult<()> {
        let mut guard = self.lock().unwrap();
        guard.set_function(identifier, function)
    }
}
