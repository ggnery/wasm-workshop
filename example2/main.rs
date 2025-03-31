use wasmtime::*;

fn main() -> Result<()> {
    let engine = Engine::default();
    let module = Module::from_file(&engine, "hello.wat")?;

    let mut store = Store::new(&engine, ());

    let hello_func = Func::wrap(&mut store, || {
        println!("Hello from Rust!");
    });

    let imports = [hello_func.into()];
    let instance = Instance::new(&mut store, &module, &imports)?;
    let run = instance.get_typed_func::<(), ()>(&mut store, "run")?;
    run.call(&mut store, ())?;

    Ok(())
}
