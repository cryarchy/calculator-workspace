# calculator-workspace

This is a basic example that shows how to:

-   run a function exported from a WebAssembly component via an interface using wasmtime,
-   provide a host function to a WebAssembly component and invoke it in the wasm component.

This Rust workspace contains three packages:

add:

-   defines a function named `add`,
-   is meant to be compiled into a wasm component.
    calculator:
-   defines a function named `eval-expression`,
-   is meant to be compiled into a wasm component.
    runner:
-   contains the host application's code,
-   links a function which is called from the calculator wasm component,
-   invokes `eval-expression` function exported from the calculator wasm component.

To run this example:

-   Run the following command from the `add` folder:
    `cargo component build --release`
-   Run the following command from the `calculaator` folder:
    `cargo component build --release`
-   From the workspace root folder run:
    `cargo run --bin runner`
