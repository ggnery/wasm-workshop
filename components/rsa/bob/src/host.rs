use std::error::Error;

use wasmtime::component::{Component, Linker, Instance};
use wasmtime::{Config, Engine, Store};
use wasmtime_wasi::{IoView, ResourceTable, WasiCtx, WasiCtxBuilder, WasiView};

wasmtime::component::bindgen!("cryptography");

pub struct Host {
    store: Store<State>,
    instance: Instance
}

impl Host {
    pub fn build(module_path: &str) -> Result<Host, Box<dyn Error>> {
        let mut config = Config::new();
        config.wasm_component_model(true);
    
        let engine = Engine::new(&config)?;

        let mut linker = Linker::<State>::new(&engine);
        wasmtime_wasi::add_to_linker_sync(&mut linker)?;

        let mut store = Store::new(
            &engine,
            State {
                ctx: WasiCtxBuilder::new().inherit_stdio().build(),
                table: ResourceTable::new()
            }
        );

        let component = Component::from_file(&engine, module_path)?;
        let instance = linker.instantiate(&mut store, &component)?;

        Ok(Host {
            store,
            instance
        })

    }    

    pub fn get_public_key(&mut self) -> Result<String, Box<dyn Error>> {
        let f = self.instance.get_typed_func::<(), (String,)>(&mut self.store, "get-public-key")?;  
    
        let result = f.call(&mut self.store, ())?;
        f.post_return(&mut self.store)?;
        

        Ok(result.0)
    }

    pub fn encrypt(&mut self, message: String, public_key: String) -> Result<Vec<u8>, Box<dyn Error>> {
       let f = self.instance.get_typed_func::<(String, String), (Vec<u8>,)>(&mut self.store, "encrypt")?;
    
        let result = f.call(&mut self.store, (message, public_key))?;
        f.post_return(&mut self.store)?;

        Ok(result.0)
    }

    pub fn decrypt(&mut self, encrypted_message: Vec<u8>)-> Result<String, Box<dyn Error>> {
        let f = self.instance.get_typed_func::<(Vec<u8>, ), (String,)>(&mut self.store, "decrypt")?;

        let result =  f.call(&mut self.store, (encrypted_message,))?;
        f.post_return(&mut self.store)?;

        Ok(result.0)
    }

}

struct State {
    ctx: WasiCtx,
    table: ResourceTable
}

impl IoView for State {
    fn table(&mut self) -> &mut ResourceTable { &mut self.table }
}

impl WasiView for State {
    fn ctx(&mut self) -> &mut WasiCtx { &mut self.ctx }
}
