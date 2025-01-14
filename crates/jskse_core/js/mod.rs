mod globals;
mod modules;

use boa_engine::vm::Vm;
use boa_engine::{Context, JsError, Source};
use boa_runtime::register;

pub fn initialize_js_engine() -> Result<(), JsError> {
    let ctx = &mut Context::default();
    if let Err(e) = globals::load_globals(ctx, ctx.global_object()) {
        return Err(e);
    }
    ctx.run_jobs();
    Ok(())
}

#[cfg(test)]
pub fn can_setup_globals() {}
