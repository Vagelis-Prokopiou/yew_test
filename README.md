# How to run

1) Install Rust (https://www.rust-lang.org/tools/install)
2) Run the following in the terminal (I use git bash usually on windows): `rustup target add wasm32-unknown-unknown`
3) Run the following in the terminal: `cargo install trunk`
4) `git clone` the project
5) `cd yew_test`
6) Run the following in the terminal: `trunk serve`
7) Open the browser to `http://127.0.0.1:8080`

# Interfacing with JavaScript ES6 modules

Currently `import` statements are not supported in the JS file.

See https://rustwasm.github.io/wasm-bindgen/reference/js-snippets.html (Caveats).

This enforcing the distinction of top level modules vs multi-level modules.

## Case 1: Top level ES6 modules

This is fully supported. Check `js/lib_a.js <=> src/js/lib_a.rs` for example usage.

## Case 2: Multi level ES6 modules

This is not currently supported. Therefore, a workaround must be used.

For example, for the uuid node module, I build the module with `browserify` in order to make it compatible for loading
in the browser.

Then, the methods of the module can be accessed from within Rust, through the global namespace.
Check `js/lib_uuid.js <=> src/js/lib_uuid.rs` for example usage.

The specific steps for achieving this are:

1) Install browserify globally: `sudo npm i -g browserify`
2) Download the module: `npm i uuid`
3) Create a file and require the module:

```js
// temp.js
const uuid = require('uuid');
window.uuid = uuid;
```

4) Create the file for the browser: `browserify temp.js -o lib_uuid.js`
5) Add the file to `index.html` with a script tag.