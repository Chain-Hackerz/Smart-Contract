use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::Item;

use chrono::Utc;

use serde_json::Result;


// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
// pub struct State  {
//     //count in our case is the size of the array
//     pub count: i32,
//     pub owner: Addr,
// }
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum Input{
    key_down{button: u64},
    key_up{button: u64},
    mouse_button_up{button: usize},
    mouse_button_down{button: usize},
    mouse_move{deltaX : i32, deltaY: i32},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Action {
    pub input: Input,
    pub time_stamp: i64,
}

impl Action {
    pub fn new(input: Input) -> Action{
        Action { input: input, time_stamp: Utc::now().timestamp() }
    }
    pub fn load(input: &str) -> Result<Action> {
        serde_json::from_str(input)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum KeyPresses {
    key_down{button: u64},
    key_up{button: u64},
    mouse_button_up{button: usize},
    mouse_button_down{button: usize},
    mouse_move{deltaX : i32, deltaY: i32},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Data {
    pub input: Vec<Action>,
}




pub const STATE: Item<Data> = Item::new("data");
