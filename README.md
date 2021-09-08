# Crawdad

Chess in the browser, via Rust and WASM

This is a _work in progress_.

## Running locally

Requirements:
* rust, cargo installed
* wasm-pack installed
* Some sort of local dev server

```
wasm-pack build --target web
cd client
<your_local_dev_server_of_choice_cmd> // JS module import rejected due to CORS in most browsers if html opened as a file
```

