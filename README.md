# Rust Multi-packages Template

This solution aims to be a simple demonstration of a multi-package template for Rust. It relies on a feature of Cargo called *workspace*. More information on this topic can be found on [the official documentation](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html). It deliberately complicates an implementation that matches the age of a human with the age of a horse or a rabbit.

Start by inspecting the main entry point of the executable: `example_app/src/main.rs`. Besides the infrastructure code to interact with the console, the crate calls a function defined in `example_enums`, that relies on `example_maths`.

## Solution layout of the template
```
|-- Cargo.lock (Cargo.lock is only located at the root level)
|-- Cargo.toml (Cargo file to set up the workspace)
|-- LICENSE
|-- README.md
|-- example_app (executable package; depends on example_enums)
|   |-- Cargo.toml
|   `-- src
|       `-- main.rs
|-- example_enums (library package; depends on example_maths)
|   |-- Cargo.toml
|   |-- src
|   |   |-- conversions.rs
|   |   `-- lib.rs
|   `-- tests (integration test)
|       `-- integration_tests.rs
`-- example_maths (library package)
|   |-- Cargo.toml
|   `-- src
|       |-- bin (a folder that can contains executable crates)
|       |   `-- multiply_demo.rs (an executable embedded in the package)
|       `-- lib.rs (source + unit tests)
`-- target (the binaries are built at the root level)
```

## Cargo files governance

The template has several `Cargo.toml`, so each package sets up their respective configuration and dependencies.
The file `Cargo.toml` located at the root sets up the workspace, by listing the packages.


## Building

Once ``rustc`` and ``cargo`` installed, execute the following command:

```
cargo build
```


## Running the tests

The tests includes the unit tests embedded in the source files (e.g. `example_maths/src/lib.rs`), the integration tests
(e.g. `example_enums/tests/*`) and the code contained in the documentation.

```
cargo test
```


## Running the executables

The template contains two executables: the crate in `example_app` and the crate in `example_maths/src/bin/`.

```
cargo run -p example_app
cargo run --bin multiply_demo
```


## Generating the documentation

```
cargo doc
```

The resulting documentation can be found in the folder `target/doc`.


## License

Distributed under the MIT License (See accompanying file /LICENSE).