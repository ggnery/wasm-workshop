use wasmtime::*;
use wasmtime_wasi::preview1::{self, WasiP1Ctx};
use wasmtime_wasi::WasiCtxBuilder;

fn main() -> Result<()> {
    let engine = Engine::default();
    let module = Module::from_file(&engine, "target/wasm32-wasip1/debug/wasm_lib.wasm")?;
    let mut linker: Linker<WasiP1Ctx> = Linker::new(&engine);

    let wasi_ctx = WasiCtxBuilder::new()
    .inherit_stdio()
    .inherit_env()
    .build_p1();

    let mut store = Store::new(&engine, wasi_ctx);

    linker.func_wrap("env", "alert", |index: i32| {
        println!("Hello from host function, received {index}")
    }).unwrap();

    preview1::add_to_linker_sync(&mut linker, |t| t)?;
    linker.instantiate_pre(&module)?;
    let instance = linker.instantiate(&mut store, &module)?;
    let greet = instance.get_typed_func::<i32, ()>(&mut store, "greet")?;
    greet.call(&mut store, 42)?;

    Ok(())
}
