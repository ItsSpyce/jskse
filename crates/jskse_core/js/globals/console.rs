use boa_engine::{js_str, Context, JsError, JsObject, JsValue, NativeFunction, NativeObject};

fn op_console_log(this: &JsValue, args: &[JsValue], ctx: &mut Context) -> Result<JsValue, JsError> {
    Ok(JsValue::undefined())
}

fn op_console_error(
    this: &JsValue,
    args: &[JsValue],
    ctx: &mut Context,
) -> Result<JsValue, JsError> {
    Ok(JsValue::undefined())
}

fn op_console_warn(
    this: &JsValue,
    args: &[JsValue],
    ctx: &mut Context,
) -> Result<JsValue, JsError> {
    Ok(JsValue::undefined())
}

fn op_console_debug(
    this: &JsValue,
    args: &[JsValue],
    ctx: &mut Context,
) -> Result<JsValue, JsError> {
    Ok(JsValue::undefined())
}

pub fn setup_global(ctx: &mut Context, global: JsObject<dyn NativeObject>) -> Result<(), JsError> {
    let console = JsObject::with_null_proto();
    let console_log = NativeFunction::from_fn_ptr(op_console_log);
    let console_warn = NativeFunction::from_fn_ptr(op_console_warn);
    let console_error = NativeFunction::from_fn_ptr(op_console_error);
    let console_debug = NativeFunction::from_fn_ptr(op_console_debug);

    if let Err(e) = console.set(
        js_str!("log"),
        console_log.to_js_function(ctx.realm()),
        true,
        ctx,
    ) {
        return Err(e);
    }

    if let Err(e) = console.set(
        js_str!("warn"),
        console_warn.to_js_function(ctx.realm()),
        true,
        ctx,
    ) {
        return Err(e);
    }

    if let Err(e) = console.set(
        js_str!("error"),
        console_error.to_js_function(ctx.realm()),
        true,
        ctx,
    ) {
        return Err(e);
    }

    if let Err(e) = console.set(
        js_str!("debug"),
        console_debug.to_js_function(ctx.realm()),
        true,
        ctx,
    ) {
        return Err(e);
    }

    if let Err(e) = global.set(js_str!("console"), console, true, ctx) {
        return Err(e);
    }
    Ok(())
}
