const fs = require('node:fs');
const wasmBuffer = fs.readFileSync('./hello.wasm');

const importObject = {
  "": {
    hello() {
      console.log("Hello from JavaScript!")
    }
  }
};  


const mod = new WebAssembly.Module(wasmBuffer);
const instance = new WebAssembly.Instance(mod, importObject);
instance.exports.run();
