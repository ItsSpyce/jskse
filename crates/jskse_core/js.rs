#![allow(static_mut_refs)]

use crate::module_loader::JskseModuleLoader;
use crate::{event_loop, ops::*};
use deno_core::futures::{self, FutureExt};
use deno_core::PollEventLoopOptions;
use deno_core::{error::AnyError, extension, JsRuntime, RuntimeOptions};
use std::env::current_dir;
use std::rc::Rc;
use std::sync::Mutex;

static JSKSE_SNAPSHOT: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/jskse.bin"));
static mut RUNTIME: Mutex<Option<JsRuntime>> = Mutex::new(None);

extension!(
    jskse,
    ops = [
        console::op_console_log,
        console::op_console_error,
        console::op_console_warn,
        console::op_console_debug,
    ],
);

async fn jskse(file_path: &str) -> Result<(), AnyError> {
    let cwd = current_dir()?;
    let main_module = deno_core::resolve_path(file_path, &cwd)?;

    let mut js_runtime = JsRuntime::new(RuntimeOptions {
        module_loader: Some(Rc::new(JskseModuleLoader::new())),
        startup_snapshot: Some(JSKSE_SNAPSHOT),
        extensions: vec![jskse::init_ops()],
        ..Default::default()
    });
    let mod_id = js_runtime.load_main_es_module(&main_module).await?;
    let _ = js_runtime.mod_evaluate(mod_id).then(|result| {
        if result.is_err() {
            eprintln!("Error evaluating module: {:?}", result);
        }
        futures::future::ready(())
    });
    let poll_options = PollEventLoopOptions {
        pump_v8_message_loop: true,
        wait_for_inspector: true,
    };
    let _ = js_runtime.run_event_loop(poll_options);
    // move js_runtime into RUNTIME

    unsafe {
        *RUNTIME.lock().unwrap() = Some(js_runtime);
    }
    Ok(())
}

pub fn initialize_engine() {
    // we can't use Tokio because it uses MSVC incompatible libraries
    jskse("./example.js");
    let event_loop =
        event_loop::SKSEEventLoop::new(unsafe { RUNTIME.lock().unwrap().as_mut().unwrap() });
}
