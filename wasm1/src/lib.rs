use js_sys::JsString;
// use js_sys::JsString;
use wasm_bindgen::prelude::*;
// use js_sys::{Object, Reflect};
// use serde_wasm_bindgen;
use serde::{Serialize, Deserialize};
use serde_json;

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


#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub name: String,
}

impl User {
//     #[wasm_bindgen(constructor)]
    pub fn new(id: i32, name: &str) -> User {
        User { id, name: name.into() }
    }
//
//     #[wasm_bindgen(getter)]
//     pub fn id(&self) -> i32 {
//         self.id
//     }
//
//     #[wasm_bindgen(getter)]
//     pub fn name(&self) -> String {
//         self.name.clone()
//     }
}

// #[wasm_bindgen]
// pub fn func4() -> User {
//     User { id: 1, name: JsValue::from("Taro") }
// }

#[wasm_bindgen]
pub fn func4()->String {
    let user = User{id: 1, name: "taro".to_string()};
    serde_json::to_string(&user).unwrap()
}

#[derive(Serialize)]
struct Users {
    pub users: Vec<User>,
}

impl Users {
    pub fn new(users: Vec<User>) -> Users {
        Users { users }
    }
}

#[wasm_bindgen]
pub fn func5() -> String {
    let users: Users = Users::new(
        vec![
            User::new(1, "Taro"),
            User::new(2, "Jiro"),
            User::new(3, "Saburo"),
        ]
    );
    serde_json::to_string(&users).unwrap()
}
