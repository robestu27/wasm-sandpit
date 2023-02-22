//extern crate lazy_static;
mod tree;

extern crate serde_json;

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

/// Simple Party struct used as an example.
/// To render in a PrimeVue tree, use Serde's rename function
#[derive(Serialize)]
pub struct Party {
    
    #[serde(rename="key")]
    party_id: i32,

    #[serde(rename="label")]
    legal_name: String,

    #[serde(rename="countryRisk")]
    country_risk: String,

    #[serde(rename="limitCurr")]
    limit_curr: String,

    #[serde(rename="limitAmt")]
    limit_amt: f64,
}

/// Version used for 
// #[derive(Serialize)]
// struct PartyTreeTable {
    
//     #[serde(rename="key")]
//     party_id: i32,


//     #[serde(rename="data.name")]
//     legal_name: String,

//     #[serde(rename="data.country")]
//     country_risk: String,


// }

fn new_party(id: i32) -> Party {

    let mnt = if id % 2 == 0 {
        id as f64 * 10007.234f64
    } else {
        id as f64 * 304767.717f64
    };

    Party {
        party_id: id,
        legal_name: format!("A generated party name {id}"),
        country_risk: "DE".into(),
        limit_amt: mnt,
        limit_curr: "EUR".into()
    }
}


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
            new_party(23007i32 + i as i32)
            //Party {legal_name: format!("[{}.{}] Party Name {}", *mg, unsafe_x, i), party_id: 23007+i as i32, country_risk: String::from("DE")}
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

    let group = new_party(93485);
    //Party {party_id: 93485, legal_name: "Party id - 93485".into(), country_risk: "DE".into() };
    let mut tree = PrimeTreeNode::new( 93485, group);

    let mut party_idx = 0;

    for _i in 0..200 {

        // let p = &v[party_idx];
        party_idx += 1;

        let p = new_party(party_idx) ; // Party {party_id: party_idx, legal_name: format!("Party id - {party_idx}"), country_risk: "DE".into() };

        let new_node_1 = tree.add_child(party_idx, p);

        for _j in 0..5 {
            // let p = &v[party_idx];
            party_idx += 1;
            let p = new_party(party_idx) ; //Party {party_id: party_idx, legal_name: format!("Party id - {party_idx}"), country_risk: "GB".into() };
            new_node_1.add_child(party_idx, p);
        }
    }

    log("[rust] get_tree <-- ");
    serde_wasm_bindgen::to_value(&tree).unwrap()

}


