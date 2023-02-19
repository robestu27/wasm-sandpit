//extern crate lazy_static;
mod tree;

use wasm_bindgen::prelude::*;
use web_sys::console;
use serde::Serialize;

use std::sync::Arc;
use std::sync::Mutex;

use crate::tree::PrimeTreeNode;

// This creates some "state" we can refer back to, in this example we can
// see the state ("inner") increases with each click
lazy_static::lazy_static! {
    static ref STATE : Arc<Mutex<u32>> = {
        let inner = 0u32;
        Arc::new(Mutex::new(inner))
    };
}

static mut XXX:usize  =0usize;

#[derive(Serialize)]

/// Simple Party struct used as an example.
/// To render in a PrimeVue tree, use Serde's rename function
struct Party {
    
    #[serde(rename="key")]
    party_id: i32,

    #[serde(rename="label")]
    legal_name: String,

    country_risk: String,
}


// #[wasm_bindgen(start)]
// pub fn start() {
//     console::log_1(&"Rust -- start()".into());
//     let document = web_sys::window().unwrap().document().unwrap();
//     let canvas = document.get_element_by_id("canvas").unwrap();
//     let canvas: web_sys::HtmlCanvasElement = canvas
//         .dyn_into::<web_sys::HtmlCanvasElement>()
//         .map_err(|_| ())
//         .unwrap();

//     let context = canvas
//         .get_context("2d")
//         .unwrap()
//         .unwrap()
//         .dyn_into::<web_sys::CanvasRenderingContext2d>()
//         .unwrap();

//     context.begin_path();

//     // Draw the outer circle.
//     context
//         .arc(75.0, 75.0, 50.0, 0.0, f64::consts::PI * 2.0)
//         .unwrap();

//     // Draw the mouth.
//     context.move_to(110.0, 75.0);
//     context.arc(75.0, 75.0, 35.0, 0.0, f64::consts::PI).unwrap();

//     // Draw the left eye.
//     context.move_to(65.0, 65.0);
//     context
//         .arc(60.0, 65.0, 5.0, 0.0, f64::consts::PI * 2.0)
//         .unwrap();

//     // Draw the right eye.
//     context.move_to(95.0, 65.0);
//     context
//         .arc(90.0, 65.0, 5.0, 0.0, f64::consts::PI * 1.0)
//         .unwrap();

//     context.stroke();
// }

fn log(msg: &str) {
    console::log_1(&wasm_bindgen::JsValue::from_str(msg));
    //web_sys::console::log_1(msg.into());
}

#[wasm_bindgen]
pub fn get_arrayx() -> JsValue {
    // console::log_1(&"I am mole and I live in a hole".into());

    // Get our ap state
    let mut mg = STATE.lock().expect("Failed to unlock");
    log(&format!("[rust] get_array --> {}", *mg));
    
    // Unsafe!! - even if it works because of Javascripts threading model (check)
    // its fugly Rust - and to use the value we need to move -- se better in STATE
    let unsafe_x = unsafe {XXX += 1; XXX };

    *mg += 1;
    let vec_size = *mg as usize;
    let mut v: Vec<Party> = Vec::with_capacity(vec_size);
    
    for i in 0..vec_size {
        v.push(
            Party {legal_name: format!("[{}.{}] Party Name {}", *mg, unsafe_x, i), party_id: 23007+i as i32, country_risk: String::from("DE")}
        );
    }

    log("[rust] get_array <--");
    serde_wasm_bindgen::to_value(&v).unwrap()

}


#[wasm_bindgen]
pub fn test_log() {
    console::log_1(&"I am 40 years old and minister for overseas development".into());
}


/// Creates a tree structure and passes back to be rendered in a PrimeFaces Tree
#[wasm_bindgen]
pub fn rust_get_tree() -> JsValue {

    log("[rust] get_tree --> ");

    let mut tree = PrimeTreeNode::new( 93485, "Group node".to_string());

    let mut party_idx = 0;

    for _i in 0..200 {

        // let p = &v[party_idx];
        party_idx += 1;

        let new_node_1 = tree.add_child(party_idx, format!("Party id - {party_idx}"));

        for _j in 0..5 {
            // let p = &v[party_idx];
            party_idx += 1;
            new_node_1.add_child(party_idx, format!("Party id - {party_idx}"));
        }
    }

    log("[rust] get_tree <-- ");
    serde_wasm_bindgen::to_value(&tree).unwrap()

}
