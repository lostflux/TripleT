
use std::fmt::{Display, Formatter, Result};
use serde::{Serialize, Deserialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Move {
  pub x: usize,
  pub y: usize
}

#[wasm_bindgen]
impl Move {
  pub fn new(x: usize, y: usize) -> Move {
    Move { x, y }
  }
}

impl Display for Move {
  fn fmt(&self, f: &mut Formatter) -> Result {
    write!(f, "<{}, {}>", self.x, self.y)?;
    Ok(())
  }
}

// impl JsObject for Move {
// }

// impl JsCast for Move {
//   fn instanceof(val: &JsValue) -> bool {
//     val.is_instance_of::<Move>()
//   }

//   fn unchecked_from_js(val: JsValue) -> Self {
//     todo!()
//   }

//   fn unchecked_from_js_ref(val: &JsValue) -> &Self {
//     todo!()
//   }
// }

// impl AsRef<JsValue> for Move {
//   fn as_ref(&self) -> &JsValue {
//     // self.as_ref()
//     todo!()
//   }
// }

// implement serialize and deserialize for Move
// impl Serialize for Move {
//   fn serialize(&self) -> JsValue {
//     JsValue::from_serde(self).unwrap()
//   }
// }

// impl AsRef<JsValue> for Move {
//   fn as_ref(&self) -> &JsValue {
//     self.as_ref()
//   }
// }

// impl JsCast for Move {
//   fn instanceof(val: &JsValue) -> bool {
//     val.is_instance_of::<Move>()
//   }

//   fn unchecked_from_js(val: JsValue) -> Self {
//     todo!()
//   }

//   fn unchecked_from_js_ref(val: &JsValue) -> &Self {
//     todo!()
//   }
// }

// impl JsObject for Move {
  
// }

