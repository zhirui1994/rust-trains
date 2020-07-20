const js = import("../pkg/hello_wasm.js");
js.then(js => {
    console.log(js.add(500, 20))
    js.greet("WebAssembly");
});