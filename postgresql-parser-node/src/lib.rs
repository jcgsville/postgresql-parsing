use neon::prelude::*;
use postgresql_parser_core::parse_postgresql;

fn is_valid_postgresql(mut cx: FunctionContext) -> JsResult<JsBoolean> {
    let input = cx.argument::<JsString>(0)?;
    let input_str = input.value(&mut cx);
    let ast = parse_postgresql(&input_str);
    Ok(cx.boolean(ast.commands.len() > 0))
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("isValidPostgresql", is_valid_postgresql)?;
    Ok(())
}
