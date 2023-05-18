use wasm_bindgen::prelude::*;
// use js_sys::{Object, Reflect};

#[wasm_bindgen]
pub fn func1(a: i32, b: i32) -> i32 {
    a + b
}

#[wasm_bindgen]
pub fn func2() -> String {
    "Hello".to_string()
}

#[wasm_bindgen]
pub fn func3() -> Vec<JsValue> {
    vec![
        JsValue::from("Hello"),
        JsValue::from("World"),
    ]
}

#[wasm_bindgen]
pub struct User {
    id: i32,
    name: String,
}

#[wasm_bindgen]
impl User {
    // #[wasm_bindgen(constructor)]
    // pub fn new(id: i32, name: String) -> User {
    //     User { id, name }
    // }

    #[wasm_bindgen(getter)]
    pub fn id(&self) -> i32 {
        self.id
    }

    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.name.clone()
    }
}

#[wasm_bindgen]
pub fn func4() -> User {
    User { id: 1, name: "Taro".into() }
}

// #[wasm_bindgen]
// pub fn filter(word: &str) -> js_sys::Array {
//     let items: Vec<JsValue> = data_included::ITEMS.iter()
//         .filter(|i| i.name.contains(word) || i.description.contains(word))
//         .map(|i| {
//             let obj = Object::new();
//             Reflect::set(&obj, &"id".into(), &JsValue::from(i.id)).unwrap();
//             Reflect::set(&obj, &"name".into(), &JsValue::from(i.name.clone())).unwrap();
//             Reflect::set(&obj, &"description".into(), &JsValue::from(i.description.clone())).unwrap();
//             JsValue::from(obj)
//         })
//         .collect();
//
//     items.into_iter().collect::<js_sys::Array>()
// }