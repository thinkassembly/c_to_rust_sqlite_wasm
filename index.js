const js = import("./dist-browser/c_to_rust_sqlite_wasm");

js.then(js => {
  js.start();

});
