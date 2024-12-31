# Common Errors

## it looks like the Rust project used to create this Wasm file was linked against

Error:

```txt
it looks like the Rust project used to create this Wasm file was linked against
version of wasm-bindgen that uses a different bindgen format than this binary:

  rust Wasm file schema version: 0.2.96
    this binary schema version: 0.2.95

Currently the bindgen format is unstable enough that these two schema versions
must exactly match. You can accomplish this by either updating this binary or
the wasm-bindgen dependency in the Rust project.

You should be able to update the wasm-bindgen dependency with:

    cargo update -p wasm-bindgen --precise 0.2.95

don't forget to recompile your Wasm file! Alternatively, you can update the
binary with:

    cargo install -f wasm-bindgen-cli --version 0.2.96

if this warning fails to go away though and you're not sure what to do feel free
to open an issue at https://github.com/rustwasm/wasm-bindgen/issues!
```

Solution:

This is caused due to mismatch of cargo-leptos to wasm-bindgen. If local build
passes and CI fails, it's probably due to different versions of cargo-leptos in docker
and nix flake
