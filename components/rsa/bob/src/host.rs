use std::error::Error;

use wasmtime::component::*;
use wasmtime::{Config, Engine, Store};
use wasmtime_wasi::preview1::WasiP1Ctx;
use wasmtime_wasi::{WasiCtxBuilder, add_to_linker_sync};


bindgen!();
pub struct Host {
    store: Store<WasiP1Ctx>,
    instance: Cryptography
}

impl Host {
    pub fn build(module_path: &str) -> Result<Host, Box<dyn Error>> {
        let mut config = Config::new();
        config.wasm_component_model(true);

        let engine = Engine::new(&config)?;
        let component = Component::from_file(&engine, module_path)?;
        
        let wasi_ctx = WasiCtxBuilder::new()
            .inherit_stdio()  
            .inherit_args() 
            .build_p1();

        let mut linker = Linker::new(&engine);
        let mut store = Store::new(&engine, wasi_ctx);

        add_to_linker_sync(&mut linker)?;

        let instance = Cryptography::instantiate(&mut store, &component, &linker)?;
        
        Ok(Host { 
            instance,
            store
        })
    }    

    pub fn get_public_key(&mut self) -> Result<String, Box<dyn Error>> {
        let mut outputs  = [Val::String(String::new())];
        self.instance.get_public_key.call(&mut self.store, &[], &mut outputs)?;

        let result = std::mem::replace(&mut outputs[0], Val::String(String::new()));

        let public_key = if let Val::String(s) = result {
            Ok(s)
        } else {
            Err("Couldn't run get_public_key func".into())
             
        };
        
        public_key
    }
    

    
}

