#[macro_use]
extern crate serde_derive;
use serde::de::{DeserializeOwned, Deserialize};
use wasm_bindgen::prelude::*;

fn arr_serde_json<T: DeserializeOwned>(boxed: Box<[JsValue]>) -> Vec<T> {
    let res: Result<Vec<T>, _> = boxed.into_iter().map(JsValue::into_serde).collect();
    match res {
        Ok(x) => x,
        Err(e) => wasm_bindgen::throw_str(&format!("{}", e)),
    }
}

fn arr_serde_wb<T: DeserializeOwned>(boxed: Box<[JsValue]>) -> Vec<T> {
    let vec = boxed.into_vec();
    let res: Result<Vec<T>, _> = vec.into_iter().map(serde_wasm_bindgen::from_value).collect();
    match res {
        Ok(x) => x,
        Err(e) => wasm_bindgen::throw_str(&format!("{}", e)),
    }
}

#[derive(Deserialize)]
struct Struct<T> {
    #[serde(bound(deserialize = "T: Deserialize<'de>"))]
    #[allow(dead_code)]
    value: T
}

#[derive(Deserialize)]
struct ArrayStruct<T>(Vec<Struct<T>>);

#[wasm_bindgen]
pub fn strings_json(strings: Box<[JsValue]>) {
    arr_serde_json::<Struct<String>>(strings);
}

#[wasm_bindgen]
pub fn strings_serde_wb(strings: Box<[JsValue]>) {
    arr_serde_wb::<Struct<String>>(strings);
}

#[wasm_bindgen]
pub fn strings_serde_wb_complete(strings: JsValue) {
    let _: ArrayStruct<String> = serde_wasm_bindgen::from_value(strings)
        .expect_throw("threw in strings_serde_wb_complete");
}

#[wasm_bindgen]
pub fn numbers_json(numbers: Box<[JsValue]>) {
    arr_serde_json::<Struct<u32>>(numbers);
}

#[wasm_bindgen]
pub fn numbers_serde_wb(numbers: Box<[JsValue]>) {
    arr_serde_wb::<Struct<u32>>(numbers);
}

#[wasm_bindgen]
pub fn numbers_serde_wb_complete(numbers: JsValue) {
    let _: ArrayStruct<u32> = serde_wasm_bindgen::from_value(numbers).expect_throw("threw in numbers_serde_wb_complete");
}
