//! Access the in-game market to buy or sell resources.
//!
//! [Screeps documentation](https://docs.screeps.com/api/#Game-market)

use js_sys::{Array, JsString, Object};
use serde::Deserialize;
use wasm_bindgen::prelude::*;

use crate::{
    constants::{MarketResourceType, OrderType, ResourceType},
    local::RoomName,
};

#[wasm_bindgen]
extern "C" {
    /// Object with info about your credits and market orders and methods for
    /// managing your participation in the market from [`Game::market`]
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Game-market)
    ///
    /// [`Game::market`]: crate::game::Game::market
    #[wasm_bindgen]
    pub type MarketInfo;

    /// Your current credit balance.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Game.market.credits)
    #[wasm_bindgen(method, getter)]
    pub fn credits(this: &MarketInfo) -> f64;

    /// An [`Array`] of the last 100 [`Transaction`]s sent to your terminals.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Game.market.incomingTransactions)
    #[wasm_bindgen(method, getter = incomingTransactions)]
    pub fn incoming_transactions(this: &MarketInfo) -> Array;

    /// An [`Array`] of the last 100 [`Transaction`]s sent from your terminals.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Game.market.outgoingTransactions)
    #[wasm_bindgen(method, getter = outgoingTransactions)]
    pub fn outgoing_transactions(this: &MarketInfo) -> Array;

    /// An [`Object`] with your current buy and sell orders on the market, with
    /// order ID [`JsString`] keys and [`MyOrder`] values.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Game.market.orders)
    #[wasm_bindgen(method, getter)]
    pub fn orders(this: &MarketInfo) -> Object;

    // todo maybe just implement a native version of this instead?
    /// Get the amount of energy required to send a given amount of any resource
    /// from one room to another.  See [`TERMINAL_SEND_COST_SCALE`] for
    /// information about the calculation.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Game.market.calcTransactionCost)
    ///
    /// [`TERMINAL_SEND_COST_SCALE`]: crate::constants::TERMINAL_SEND_COST_SCALE
    #[wasm_bindgen(method, js_name = calcTransactionCost)]
    pub fn calc_transaction_cost(
        this: &MarketInfo,
        amount: u32,
        room_1: &JsString,
        room_2: &JsString,
    ) -> u32;

    /// Cancel one of your existing orders on the market, without refunding
    /// associated fees.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Game.market.cancelOrder)
    #[wasm_bindgen(method, js_name = cancelOrder)]
    pub fn cancel_order(this: &MarketInfo, order_id: &JsString) -> i8;

    /// Cancel one of your existing orders on the market, without refunding
    /// associated fees.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Game.market.changeOrderPrice)
    #[wasm_bindgen(method, js_name = changeOrderPrice)]
    pub fn change_order_price(this: &MarketInfo, order_id: &JsString, new_price: f64) -> i8;

    // todo type to serialize call options into
    /// Create a new order on the market.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Game.market.createOrder)
    #[wasm_bindgen(method, js_name = createOrder)]
    pub fn create_order(this: &MarketInfo, order_parameters: &Object) -> i8;

    /// Execute a trade on an order on the market. Name of a room with a
    /// terminal from which to send or receive resources is required unless the
    /// order is for an account resource.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Game.market.deal)
    #[wasm_bindgen(method)]
    pub fn deal(
        this: &MarketInfo,
        order_id: &JsString,
        amount: u32,
        room_name: Option<&JsString>,
    ) -> i8;

    /// Adds more capacity to one of your existing orders, offering or
    /// requesting more of the resource and incurring additional fees.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Game.market.extendOrder)
    #[wasm_bindgen(method, js_name = extendOrder)]
    pub fn extend_order(this: &MarketInfo, order_id: &JsString, add_amount: u32) -> i8;

    // todo type to serialize call options into - special efficient behavior when
    // passed a `{resourceType: type}` filter
    /// Get an [`Array`[] of all [`Order`]s on the market, with an optional
    /// filter object. Note that a filter key of `resourceType` with a type
    /// restriction has special handling in the engine to be more efficient
    /// ([source]).
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Game.market.getAllOrders)
    ///
    /// [source]: https://github.com/screeps/engine/blob/f7a09e637c20689084fcf4eb43eacdfd51d31476/src/game/market.js#L37
    #[wasm_bindgen(method, js_name = getAllOrders)]
    pub fn get_all_orders(this: &MarketInfo, filter: &Object) -> Array;

    // todo this is probably breaking in an interesting way on private servers due to the {} return https://github.com/screeps/engine/pull/131 - maybe catch?
    /// Get information about the price history on the market for the last 14
    /// days for a given resource as an [`Array`] of [`OrderHistoryRecord`]s, or
    /// for all resources if `None`. Warning: returns an empty [`Object`]
    /// instead of an array if there is no history for the resource, verifying
    /// the type is recommended before use if the market might be empty.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Game.market.getHistory)
    #[wasm_bindgen(method, js_name = getHistory)]
    pub fn get_history(this: &MarketInfo, resource: Option<ResourceType>) -> JsValue;

    /// Get an object with information about a specific order, in the same
    /// format as returned by [`MarketInfo::get_all_orders`]
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Game.market.getOrderById)
    #[wasm_bindgen(method, js_name = getOrderById)]
    pub fn get_order_by_id(this: &MarketInfo, order_id: &JsString) -> Option<Order>;
}

#[wasm_bindgen]
extern "C" {
    /// An object that represents an order on the market.
    #[wasm_bindgen]
    pub type Order;
    /// The order ID, which can be used to retrieve the order, or execute a
    /// trade using [`MarketInfo::deal`].
    #[wasm_bindgen(method, getter)]
    pub fn id(this: &Order) -> JsString;
    /// Tick of order creation, `None` for intershard orders.
    #[wasm_bindgen(method, getter)]
    pub fn created(this: &Order) -> Option<u32>;
    /// Timestamp of order creation in milliseconds since epoch.
    #[wasm_bindgen(method, getter = createdTimestamp)]
    pub fn created_timestamp(this: &Order) -> u64;
    /// The [`OrderType`] of the order (whether the owner is looking to buy or
    /// sell the given resource).
    #[wasm_bindgen(method, getter = type)]
    pub fn order_type(this: &Order) -> OrderType;
    /// The resource type this order is for.
    #[wasm_bindgen(method, getter = resourceType)]
    pub fn resource_type(this: &Order) -> MarketResourceType;
    /// Room that owns the order, `None` for intershard orders.
    #[wasm_bindgen(method, getter = roomName)]
    pub fn room_name(this: &Order) -> Option<JsString>;
    /// The amount of resource currently ready to be traded (loaded in the
    /// terminal).
    #[wasm_bindgen(method, getter)]
    pub fn amount(this: &Order) -> u32;
    /// The total remaining amount of the resource to be traded before this
    /// order has been completely filled and removed.
    #[wasm_bindgen(method, getter = remainingAmount)]
    pub fn remaining_amount(this: &Order) -> u32;
    /// Price of the order per unit of the resource the order is for.
    #[wasm_bindgen(method, getter)]
    pub fn price(this: &Order) -> f64;
}

/// A rust-native local representation of an order, which can be deserialized
/// across the memory boundary in one operation
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LocalOrder {
    /// The order ID, which can be used to retrieve the order, or execute a
    /// trade using [`MarketInfo::deal`].
    pub id: String,
    /// Tick of order creation, `None` for intershard orders.
    pub created: Option<u32>,
    /// Timestamp of order creation in milliseconds since epoch.
    pub created_timestamp: u64,
    /// The [`OrderType`] of the order (whether the owner is looking to buy or
    /// sell the given resource).
    #[serde(rename = "type")]
    pub order_type: OrderType,
    /// The resource type this order is for.
    pub resource_type: MarketResourceType,
    /// Room that owns the order, `None` for intershard orders.
    pub room_name: Option<RoomName>,
    /// The amount of resource currently ready to be traded (loaded in the
    /// terminal).
    pub amount: u32,
    /// The total remaining amount of the resource to be traded before this
    /// order has been completely filled and removed.
    pub remaining_amount: u32,
    /// Price of the order per unit of the resource the order is for.
    pub price: f64,
}

// todo docs
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen]
    pub type Transaction;
    #[wasm_bindgen(method, getter = transactionId)]
    pub fn transaction_id(this: &Transaction) -> JsString;
    #[wasm_bindgen(method, getter)]
    pub fn time(this: &Transaction) -> u32;
    /// The player who sent resources for this transaction, or `None` if it was
    /// an NPC terminal
    #[wasm_bindgen(method, getter)]
    pub fn sender(this: &Transaction) -> Option<Player>;
    /// The recipient of the resources for this transaction, or `None` if it was
    /// an NPC terminal
    #[wasm_bindgen(method, getter)]
    pub fn recipient(this: &Transaction) -> Option<Player>;
    #[wasm_bindgen(method, getter = resourceType)]
    pub fn resource_type(this: &Transaction) -> ResourceType;
    #[wasm_bindgen(method, getter)]
    pub fn amount(this: &Transaction) -> u32;
    /// The room that sent resources for this transaction
    #[wasm_bindgen(method, getter)]
    pub fn from(this: &Transaction) -> JsString;
    /// The room that received resources in this transaction
    #[wasm_bindgen(method, getter)]
    pub fn to(this: &Transaction) -> JsString;
    /// The description set in the sender's `StructureTerminal::send()` call, if
    /// any
    #[wasm_bindgen(method, getter)]
    pub fn description(this: &Transaction) -> Option<JsString>;
    /// Information about the market order that this transaction was fulfilling,
    /// if any
    #[wasm_bindgen(method, getter = order)]
    pub fn order(this: &Transaction) -> Option<TransactionOrder>;
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LocalTransaction {
    pub transaction_id: String,
    pub time: u32,
    /// The player who sent resources for this transaction, or `None` if it was
    /// an NPC terminal
    pub sender: Option<LocalPlayer>,
    /// The recipient of the resources for this transaction, or `None` if it was
    /// an NPC terminal
    pub recipient: Option<LocalPlayer>,
    pub resource_type: ResourceType,
    pub amount: u32,
    /// The room that sent resources for this transaction
    pub from: RoomName,
    /// The room that received resources in this transaction
    pub to: RoomName,
    /// The description set in the sender's `StructureTerminal::send()` call, if
    /// any
    pub description: Option<String>,
    /// Information about the market order that this transaction was fulfilling,
    /// if any
    pub order: Option<LocalTransactionOrder>,
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen]
    pub type Player;
    #[wasm_bindgen(method, getter)]
    pub fn username(this: &Player) -> JsString;
}

#[derive(Deserialize, Debug)]
pub struct LocalPlayer {
    pub username: String,
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen]
    pub type TransactionOrder;
    #[wasm_bindgen(method, getter)]
    pub fn id(this: &TransactionOrder) -> JsString;
    #[wasm_bindgen(method, getter = type)]
    pub fn order_type(this: &TransactionOrder) -> OrderType;
    #[wasm_bindgen(method, getter)]
    pub fn price(this: &TransactionOrder) -> f64;
}

#[derive(Deserialize, Debug)]
pub struct LocalTransactionOrder {
    pub id: String,
    #[serde(rename = "type")]
    pub order_type: OrderType,
    pub price: f64,
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen]
    pub type MyOrder;
    #[wasm_bindgen(method, getter)]
    pub fn id(this: &MyOrder) -> JsString;
    /// Tick of order creation, `None` for intershard orders
    #[wasm_bindgen(method, getter)]
    pub fn created(this: &MyOrder) -> Option<u32>;
    /// Timestamp of order creation in milliseconds since epoch
    #[wasm_bindgen(method, getter = createdTimestamp)]
    pub fn created_timestamp(this: &MyOrder) -> u64;
    #[wasm_bindgen(method, getter)]
    pub fn active(this: &MyOrder) -> bool;
    #[wasm_bindgen(method, getter = type)]
    pub fn order_type(this: &MyOrder) -> OrderType;
    #[wasm_bindgen(method, getter = resourceType)]
    pub fn resource_type(this: &MyOrder) -> MarketResourceType;
    /// Room that owns the order, `None` for intershard orders
    #[wasm_bindgen(method, getter = roomName)]
    pub fn room_name(this: &MyOrder) -> Option<JsString>;
    #[wasm_bindgen(method, getter)]
    pub fn amount(this: &MyOrder) -> u32;
    #[wasm_bindgen(method, getter = remainingAmount)]
    pub fn remaining_amount(this: &MyOrder) -> u32;
    #[wasm_bindgen(method, getter = totalAmount)]
    pub fn total_amount(this: &MyOrder) -> u32;
    #[wasm_bindgen(method, getter)]
    pub fn price(this: &MyOrder) -> f64;
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LocalMyOrder {
    pub id: String,
    /// Tick of order creation, `None` for intershard orders
    pub created: Option<u32>,
    /// Timestamp of order creation in milliseconds since epoch
    pub created_timestamp: u64,
    pub active: bool,
    #[serde(rename = "type")]
    pub order_type: OrderType,
    pub resource_type: MarketResourceType,
    /// Room that owns the order, `None` for intershard orders
    pub room_name: Option<RoomName>,
    pub amount: u32,
    pub remaining_amount: u32,
    pub total_amount: u32,
    pub price: f64,
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen]
    pub type OrderHistoryRecord;
    #[wasm_bindgen(method, getter = resourceType)]
    pub fn resource_type(this: &OrderHistoryRecord) -> MarketResourceType;
    /// Calendar date in string format, eg "2018-12-31"
    #[wasm_bindgen(method, getter)]
    pub fn date(this: &OrderHistoryRecord) -> JsString;
    /// Total number of transactions for this resource on this day
    #[wasm_bindgen(method, getter)]
    pub fn transactions(this: &OrderHistoryRecord) -> u32;
    /// Total volume of this resource bought and sold on this day
    #[wasm_bindgen(method, getter)]
    pub fn volume(this: &OrderHistoryRecord) -> u32;
    #[wasm_bindgen(method, getter = avgPrice)]
    pub fn avg_price(this: &OrderHistoryRecord) -> f64;
    #[wasm_bindgen(method, getter = stddevPrice)]
    pub fn stddev_price(this: &OrderHistoryRecord) -> f64;
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LocalOrderHistoryRecord {
    pub resource_type: MarketResourceType,
    /// Calendar date in string format, eg "2018-12-31"
    pub date: String,
    /// Total number of transactions for this resource on this day
    pub transactions: u32,
    /// Total volume of this resource bought and sold on this day
    pub volume: u32,
    pub avg_price: f64,
    pub stddev_price: f64,
}

// use std::{borrow::Cow, collections::HashMap, str::FromStr};

// use parse_display::FromStr;
// use serde::{
//     de::{Deserializer, Error as _, Unexpected},
//     Deserialize,
// };
// use serde_repr::{Deserialize_repr ,Serialize_repr};

// use crate::{
//     constants::{MarketResourceType, ResourceType, ReturnCode},
//     local::RoomName,
//     traits::TryInto,
// };

// /// Translates the `ORDER_SELL` and `ORDER_BUY` constants.
// ///
// /// *Note:* This constant's `TryFrom<Value>`, `Serialize` and `Deserialize`
// /// implementations only operate on made-up integer constants. If you're ever
// /// using these impls manually, use the `__order_type_num_to_str` and
// /// `__order_type_str_to_num` JavaScript functions,
// /// [`FromStr`][std::str::FromStr] or [`OrderType::deserialize_from_str`].
// ///
// /// `OrderType`'s `FromStr`, `Display` and `ToString` representations accurately
// /// represent the strings the game constant uses.
// ///
// /// See the [constants module's documentation][crate::constants] for more
// /// details.
// #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize_repr, Deserialize_repr, FromStr)]
// #[repr(u8)]
// pub enum OrderType {
//     #[display("sell")]
//     Sell = 0,
//     #[display("buy")]
//     Buy = 1,
// }

// impl OrderType {
//     /// Helper function for deserializing from a string rather than from an
//     /// integer.
//     pub fn deserialize_from_str<'de, D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
//         let s: Cow<'de, str> = Cow::deserialize(d)?;
//         Self::from_str(&s)
//             .map_err(|_| D::Error::invalid_value(Unexpected::Str(&s), &r#""buy" or "sell""#))
//     }
// }

// // impl OrderType {
// //     fn as_string(&self) -> String {
// //         match self {
// //             OrderType::Sell => String::from("sell"),
// //             OrderType::Buy => String::from("buy")
// //         }
// //     }
// // }

// #[derive(Deserialize, Debug)]
// pub struct Player {
//     pub username: String,
// }
// js_deserializable!(Player);

// #[derive(Deserialize, Debug)]
// pub struct TransactionOrder {
//     pub id: String,
//     #[serde(rename = "type", deserialize_with = "OrderType::deserialize_from_str")]
//     pub order_type: OrderType,
//     pub price: f64,
// }
// js_deserializable!(TransactionOrder);

// #[derive(Deserialize, Debug)]
// #[serde(rename_all = "camelCase")]
// pub struct Transaction {
//     pub transaction_id: String,
//     pub time: u32,
//     /// The player who sent resources for this transaction, or `None` if it was
//     /// an NPC terminal
//     pub sender: Option<Player>,
//     /// The recipient of the resources for this transaction, or `None` if it was
//     /// an NPC terminal
//     pub recipient: Option<Player>,
//     #[serde(deserialize_with = "ResourceType::deserialize_from_str")]
//     pub resource_type: ResourceType,
//     pub amount: u32,
//     /// The room that sent resources for this transaction
//     pub from: RoomName,
//     /// The room that received resources in this transaction
//     pub to: RoomName,
//     /// The description set in the sender's `StructureTerminal::send()` call, if
//     /// any
//     pub description: Option<String>,
//     /// Information about the market order that this transaction was fulfilling,
//     /// if any
//     pub order: Option<TransactionOrder>,
// }
// js_deserializable!(Transaction);

// #[derive(Deserialize, Debug)]
// #[serde(rename_all = "camelCase")]
// pub struct Order {
//     pub id: String,
//     /// Tick of order creation, `None` for intershard orders
//     pub created: Option<u32>,
//     /// Timestamp of order creation in milliseconds since epoch
//     pub created_timestamp: u64,
//     #[serde(rename = "type", deserialize_with = "OrderType::deserialize_from_str")]
//     pub order_type: OrderType,
//     #[serde(deserialize_with = "MarketResourceType::deserialize_from_str")]
//     pub resource_type: MarketResourceType,
//     /// Room that owns the order, `None` for intershard orders
//     pub room_name: Option<RoomName>,
//     pub amount: u32,
//     pub remaining_amount: u32,
//     pub price: f64,
// }
// js_deserializable!(Order);

// #[derive(Deserialize, Debug)]
// #[serde(rename_all = "camelCase")]
// pub struct MyOrder {
//     pub id: String,
//     /// Tick of order creation, `None` for intershard orders
//     pub created: Option<u32>,
//     /// Timestamp of order creation in milliseconds since epoch
//     pub created_timestamp: u64,
//     pub active: bool,
//     #[serde(rename = "type", deserialize_with = "OrderType::deserialize_from_str")]
//     pub order_type: OrderType,
//     #[serde(deserialize_with = "MarketResourceType::deserialize_from_str")]
//     pub resource_type: MarketResourceType,
//     /// Room that owns the order, `None` for intershard orders
//     pub room_name: Option<RoomName>,
//     pub amount: u32,
//     pub remaining_amount: u32,
//     pub total_amount: u32,
//     pub price: f64,
// }
// js_deserializable!(MyOrder);

// #[derive(Deserialize, Debug)]
// #[serde(rename_all = "camelCase")]
// pub struct OrderHistoryRecord {
//     #[serde(deserialize_with = "MarketResourceType::deserialize_from_str")]
//     pub resource_type: MarketResourceType,
//     /// Calendar date in string format, eg "2018-12-31"
//     pub date: String,
//     /// Total number of transactions for this resource on this day
//     pub transactions: u32,
//     /// Total volume of this resource bought and sold on this day
//     pub volume: u32,
//     pub avg_price: f64,
//     pub stddev_price: f64,
// }
// js_deserializable!(OrderHistoryRecord);

// pub fn credits() -> f64 {
//     js_unwrap!(Game.market.credits)
// }

// pub fn incoming_transactions() -> Vec<Transaction> {
//     js_unwrap!(Game.market.incomingTransactions)
// }

// pub fn outgoing_transactions() -> Vec<Transaction> {
//     js_unwrap!(Game.market.outgoingTransactions)
// }

// /// Get a `HashMap` of the player's currently-listed market orders
// pub fn orders() -> HashMap<String, MyOrder> {
//     js_unwrap!(Game.market.orders)
// }

// pub fn calc_transaction_cost(amount: u32, room1: RoomName, room2: RoomName) -> f64 {
//     js_unwrap!(Game.market.calcTransactionCost(@{amount}, @{room1.to_string()}, @{room2.to_string()}))
// }

// pub fn cancel_order(order_id: &str) -> ReturnCode {
//     js_unwrap!(Game.market.cancelOrder(@{order_id}))
// }

// pub fn change_order_price(order_id: &str, new_price: f64) -> ReturnCode {
//     js_unwrap!(Game.market.changeOrderPrice(@{order_id}, @{new_price}))
// }

// pub fn create_order(
//     order_type: OrderType,
//     resource_type: MarketResourceType,
//     price: f64,
//     total_amount: u32,
//     room: Option<RoomName>,
// ) -> ReturnCode {
//     let resource_num = match resource_type {
//         MarketResourceType::Resource(ty) => ty as u32,
//         MarketResourceType::IntershardResource(ty) => ty as u32,
//     };
//     match room {
//         Some(room_name) => {
//             js_unwrap! {
//                 Game.market.createOrder({
//                     type: __order_type_num_to_str(@{order_type as u32}),
//                     resourceType: __resource_type_num_to_str(@{resource_num}),
//                     price: @{price},
//                     totalAmount: @{total_amount},
//                     roomName: @{room_name.to_string()}
//                 })
//             }
//         }
//         None => {
//             js_unwrap! {
//                 Game.market.createOrder({
//                     type: __order_type_num_to_str(@{order_type as u32}),
//                     resourceType: __resource_type_num_to_str(@{resource_num}),
//                     price: @{price},
//                     totalAmount: @{total_amount}
//                 })
//             }
//         }
//     }
// }

// /// Execute a market trade
// ///
// /// `target_room` is your owned room whose terminal will send or receive
// /// resources in this transaction, or `None` if this is an order for an
// /// intershard resource type
// pub fn deal(order_id: &str, amount: u32, target_room: Option<RoomName>) -> ReturnCode {
//     match target_room {
//         Some(target_room_name) => {
//             js_unwrap!(Game.market.deal(@{order_id}, @{amount}, @{target_room_name.to_string()}))
//         }
//         None => js_unwrap!(Game.market.deal(@{order_id}, @{amount})),
//     }
// }

// pub fn extend_order(order_id: &str, add_amount: u32) -> ReturnCode {
//     js_unwrap!(Game.market.extendOrder(@{order_id}, @{add_amount}))
// }

// /// Get all orders from the market
// ///
// /// Full filtering support is not available, but filtering by resource type
// /// is available and will reduce the CPU cost compared to getting all orders
// pub fn get_all_orders(resource: Option<MarketResourceType>) -> Vec<Order> {
//     match resource {
//         Some(resource_type) => {
//             let resource_num = match resource_type {
//                 MarketResourceType::Resource(ty) => ty as u32,
//                 MarketResourceType::IntershardResource(ty) => ty as u32,
//             };
//             js_unwrap! {
//                 Game.market.getAllOrders({
//                     resourceType: __resource_type_num_to_str(@{resource_num})
//                 })
//             }
//         }
//         None => js_unwrap!(Game.market.getAllOrders()),
//     }
// }

// /// Provides historical information on the price of each resource over the last
// /// 14 days
// ///
// /// Provide a resource type to get history for using `Some(ResourceType)`, or
// /// get data for all resources by passing `None`
// pub fn get_history(resource: Option<MarketResourceType>) -> Vec<OrderHistoryRecord> {
//     match resource {
//         Some(resource_type) => {
//             match resource_type {
//                 // workaround: Game.market.getHistory returns `{}` instead of `[]` when querying a resource type
//                 // that has no history records
//                 // Verify records are present otherwise return an empty array to prevent panics
//                 MarketResourceType::Resource(ty) => js!(
//                     const history = Game.market.getHistory(__resource_type_num_to_str(@{ty as u32}));
//                     if (history && history.length > 0) {
//                         return history;
//                     } else {
//                         return [];
//                     }
//                 ).try_into().unwrap(),
//                 MarketResourceType::IntershardResource(ty) => js!(
//                     const history = Game.market.getHistory(__resource_type_num_to_str(@{ty as u32}));
//                     if (history && history.length > 0) {
//                         return history;
//                     } else {
//                         return [];
//                     }
//                 ).try_into().unwrap(),
//             }
//         }
//         None => js_unwrap!(Game.market.getHistory()),
//     }
// }

// pub fn get_order(id: &str) -> Option<Order> {
//     let order = js! {
//         return Game.market.getOrderById(@{id});
//     };
//     order.try_into().ok()
// }
