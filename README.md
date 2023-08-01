# wikifunctions

this is a project for dealing with wikifunctions objects in JSON.

This project is dual-licensed under MIT and Apache 2.0.

# Rust function evaluator

the function evaluator is currently a demo, with plans to be incorporated to the
main project. The evaluator currently uses libloading to compile and load functions
dynamically.

Contributions are welcomed!

## How to run

Make sure you have the latest stable Rust toolchain installed on the system.

The runtime library is required for compiling the evaluated function, so compile it using this command:

```
cargo build -p wfrt
```

Then, from the workspace root, you can run the evaluator using the following command:

```
cargo run -p wf-evaluator
```

When you run the evaluator, it doesn't print anything. This is because it needs to be supplied with
input. As a starting point, you can copy-paste `test-data/evaluator_input.json` into its standard
input to see the output. (serialzing output as wikifunction JSON currently unimplemented)

## evaluator functionality checklist

* [x] parse json input
* [x] compile and execute functions
* [ ] deserialization
    * [x] Deserializing strings
    * [ ] Deserializing booleans
    * [ ] Deserializing lists/maps/pairs
* [ ] serializing return values
* [ ] compile to WASM instead of native object format?
