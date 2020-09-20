# Benchmark: Passing arrays of structs to Rust/`wasm-bindgen`

All benchmarks on a 2010 MacPro5,1 with an E5620 @ 2.40GHz in it. To reproduce 
run `yarn && yarn start -p` and switch to your browser.

`json` refers to using the 
[`serde-serialize`](https://rustwasm.github.io/docs/wasm-bindgen/reference/arbitrary-data-with-serde.html) 
feature of `wasm-bindgen`, which uses `serde_json` under the hood. `serde_wb` 
refers to 
[serde-wasm-bindgen](https://github.com/cloudflare/serde-wasm-bindgen). Refer 
to `src/lib.rs`.

We're essentially trying to pass a 100-item array of this struct from 
JavaScript to Rust, where `T` is `String` or alternatively `u32`,

```rust
struct Struct<T> {
    value: T
}
```

_Last updated 20 September 2020_

## Firefox 80.0.1 (64-bit)

Benchmark | Time
-- | --
strings_json | 157 ms [1.57 ms / iter]
strings_serde_wb | 142 ms [1.42 ms / iter]
strings_serde_wb_complete | 204 ms [2.04 ms / iter]
numbers_json | 117 ms [1.17 ms / iter]
numbers_serde_wb | 124 ms [1.24 ms / iter]
numbers_serde_wb_complete | 167 ms [1.67 ms / iter]

## Chrome 85.0.4183.102

Benchmark | Time
-- | --
strings_json | 182.9 ms [1.829 ms / iter]
strings_serde_wb | 100.93 ms [1.0093 ms / iter]
strings_serde_wb_complete | 118.19 ms [1.1819 ms / iter]
numbers_json | 137.795 ms [1.37795 ms / iter]
numbers_serde_wb | 72.36 ms [0.7236 ms / iter]
numbers_serde_wb_complete | 83.715 ms [0.83715 ms / iter]

## Safari 13.1.2 (14609.3.5.1.5) (from macOS 10.14.6)

Benchmark | Time
-- | --
strings_json | 204 ms [2.04 ms / iter]
strings_serde_wb | 66 ms [0.66 ms / iter]
strings_serde_wb_complete | 85 ms [0.85 ms / iter]
numbers_json | 144 ms [1.44 ms / iter]
numbers_serde_wb | 52 ms [0.52 ms / iter]
numbers_serde_wb_complete | 66 ms [0.66 ms / iter]

> Editor's note: whoa. WebKit's doing some low-overhead JS<->Wasm calls.
