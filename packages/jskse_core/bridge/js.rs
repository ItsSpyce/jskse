use boa_engine::{Context, Source};

use crate::{log_error, log_info};

pub fn initialize_engine() {
    let mut context = Context::default();
    let source = Source::from_bytes(
        r#"
  console.log('Hello world!')
  42
  "#,
    );
    let result = context.eval(source);
    match result {
        Ok(value) => {
            log_info(format!("Result: {:?}", value));
        }
        Err(error) => {
            log_error(format!("Error: {:?}", error));
        }
    }
}
