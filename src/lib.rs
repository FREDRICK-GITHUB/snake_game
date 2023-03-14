
use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(name);
}

//importing the JS Alert() to be used to display the name above in greet()
#[wasm_bindgen]
extern {
    pub fn alert(name: &str);
}