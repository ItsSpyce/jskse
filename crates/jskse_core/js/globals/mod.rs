use boa_engine::{Context, JsError, JsObject, NativeObject};

mod console;

pub fn load_globals(ctx: &mut Context, global: JsObject<dyn NativeObject>) -> Result<(), JsError> {
    crate::js::globals::console::setup_global(ctx, global);
    Ok(())
}
