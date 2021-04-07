//! See [https://docs.screeps.com/api/#Game-market]
//!
//! [https://docs.screeps.com/api/#Game-market]: https://docs.screeps.com/api/#Game-market
use std::{borrow::Cow, collections::HashMap, str::FromStr};

use parse_display::FromStr;
use serde::{
    de::{Deserializer, Error as _, Unexpected},
    Deserialize,
};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::{
    constants::{MarketResourceType, ResourceType, ReturnCode},
    local::RoomName,
    traits::TryInto,
};

/// Translates the `ORDER_SELL` and `ORDER_BUY` constants.
///
/// *Note:* This constant's `TryFrom<Value>`, `Serialize` and `Deserialize`
/// implementations only operate on made-up integer constants. If you're ever
/// using these impls manually, use the `__order_type_num_to_str` and
/// `__order_type_str_to_num` JavaScript functions,
/// [`FromStr`][std::str::FromStr] or [`OrderType::deserialize_from_str`].
///
/// `OrderType`'s `FromStr`, `Display` and `ToString` representations accurately
/// represent the strings the game constant uses.
///
/// See the [constants module's documentation][crate::constants] for more
/// details.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize_repr, Deserialize_repr, FromStr)]
#[repr(u8)]
pub enum OrderType {
    #[display("sell")]
    Sell = 0,
    #[display("buy")]
    Buy = 1,
}

impl OrderType {
    /// Helper function for deserializing from a string rather than from an
    /// integer.
    pub fn deserialize_from_str<'de, D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let s: Cow<'de, str> = Cow::deserialize(d)?;
        Self::from_str(&s)
            .map_err(|_| D::Error::invalid_value(Unexpected::Str(&s), &r#""buy" or "sell""#))
    }
}

// impl OrderType {
//     fn as_string(&self) -> String {
//         match self {
//             OrderType::Sell => String::from("sell"),
//             OrderType::Buy => String::from("buy")
//         }
//     }
// }

#[derive(Deserialize, Debug)]
pub struct Player {
    pub username: String,
}
js_deserializable!(Player);

#[derive(Deserialize, Debug)]
pub struct TransactionOrder {
    pub id: String,
    #[serde(rename = "type", deserialize_with = "OrderType::deserialize_from_str")]
    pub order_type: OrderType,
    pub price: f64,
}
js_deserializable!(TransactionOrder);

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    pub transaction_id: String,
    pub time: u32,
    /// The player who sent resources for this transaction, or `None` if it was
    /// an NPC terminal
    pub sender: Option<Player>,
    /// The recipient of the resources for this transaction, or `None` if it was
    /// an NPC terminal
    pub recipient: Option<Player>,
    #[serde(deserialize_with = "ResourceType::deserialize_from_str")]
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
    pub order: Option<TransactionOrder>,
}
js_deserializable!(Transaction);

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    pub id: String,
    /// Tick of order creation, `None` for intershard orders
    pub created: Option<u32>,
    /// Timestamp of order creation in milliseconds since epoch
    pub created_timestamp: u64,
    #[serde(rename = "type", deserialize_with = "OrderType::deserialize_from_str")]
    pub order_type: OrderType,
    #[serde(deserialize_with = "MarketResourceType::deserialize_from_str")]
    pub resource_type: MarketResourceType,
    /// Room that owns the order, `None` for intershard orders
    pub room_name: Option<RoomName>,
    pub amount: u32,
    pub remaining_amount: u32,
    pub price: f64,
}
js_deserializable!(Order);

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MyOrder {
    pub id: String,
    /// Tick of order creation, `None` for intershard orders
    pub created: Option<u32>,
    /// Timestamp of order creation in milliseconds since epoch
    pub created_timestamp: u64,
    pub active: bool,
    #[serde(rename = "type", deserialize_with = "OrderType::deserialize_from_str")]
    pub order_type: OrderType,
    #[serde(deserialize_with = "MarketResourceType::deserialize_from_str")]
    pub resource_type: MarketResourceType,
    /// Room that owns the order, `None` for intershard orders
    pub room_name: Option<RoomName>,
    pub amount: u32,
    pub remaining_amount: u32,
    pub total_amount: u32,
    pub price: f64,
}
js_deserializable!(MyOrder);

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderHistoryRecord {
    #[serde(deserialize_with = "MarketResourceType::deserialize_from_str")]
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
js_deserializable!(OrderHistoryRecord);

pub fn credits() -> f64 {
    js_unwrap!(Game.market.credits)
}

pub fn incoming_transactions() -> Vec<Transaction> {
    js_unwrap!(Game.market.incomingTransactions)
}

pub fn outgoing_transactions() -> Vec<Transaction> {
    js_unwrap!(Game.market.outgoingTransactions)
}

/// Get a `HashMap` of the player's currently-listed market orders
pub fn orders() -> HashMap<String, MyOrder> {
    js_unwrap!(Game.market.orders)
}

pub fn calc_transaction_cost(amount: u32, room1: RoomName, room2: RoomName) -> f64 {
    js_unwrap!(Game.market.calcTransactionCost(@{amount}, @{room1.to_string()}, @{room2.to_string()}))
}

pub fn cancel_order(order_id: &str) -> ReturnCode {
    js_unwrap!(Game.market.cancelOrder(@{order_id}))
}

pub fn change_order_price(order_id: &str, new_price: f64) -> ReturnCode {
    js_unwrap!(Game.market.changeOrderPrice(@{order_id}, @{new_price}))
}

pub fn create_order(
    order_type: OrderType,
    resource_type: MarketResourceType,
    price: f64,
    total_amount: u32,
    room: Option<RoomName>,
) -> ReturnCode {
    let resource_num = match resource_type {
        MarketResourceType::Resource(ty) => ty as u32,
        MarketResourceType::IntershardResource(ty) => ty as u32,
    };
    match room {
        Some(room_name) => {
            js_unwrap! {
                Game.market.createOrder({
                    type: __order_type_num_to_str(@{order_type as u32}),
                    resourceType: __resource_type_num_to_str(@{resource_num}),
                    price: @{price},
                    totalAmount: @{total_amount},
                    roomName: @{room_name.to_string()}
                })
            }
        }
        None => {
            js_unwrap! {
                Game.market.createOrder({
                    type: __order_type_num_to_str(@{order_type as u32}),
                    resourceType: __resource_type_num_to_str(@{resource_num}),
                    price: @{price},
                    totalAmount: @{total_amount}
                })
            }
        }
    }
}

/// Execute a market trade
///
/// `target_room` is your owned room whose terminal will send or receive
/// resources in this transaction, or `None` if this is an order for an
/// intershard resource type
pub fn deal(order_id: &str, amount: u32, target_room: Option<RoomName>) -> ReturnCode {
    match target_room {
        Some(target_room_name) => {
            js_unwrap!(Game.market.deal(@{order_id}, @{amount}, @{target_room_name.to_string()}))
        }
        None => js_unwrap!(Game.market.deal(@{order_id}, @{amount})),
    }
}

pub fn extend_order(order_id: &str, add_amount: u32) -> ReturnCode {
    js_unwrap!(Game.market.extendOrder(@{order_id}, @{add_amount}))
}

/// Get all orders from the market
///
/// Full filtering support is not available, but filtering by resource type
/// is available and will reduce the CPU cost compared to getting all orders
pub fn get_all_orders(resource: Option<MarketResourceType>) -> Vec<Order> {
    match resource {
        Some(resource_type) => {
            let resource_num = match resource_type {
                MarketResourceType::Resource(ty) => ty as u32,
                MarketResourceType::IntershardResource(ty) => ty as u32,
            };
            js_unwrap! {
                Game.market.getAllOrders({
                    resourceType: __resource_type_num_to_str(@{resource_num})
                })
            }
        }
        None => js_unwrap!(Game.market.getAllOrders()),
    }
}

/// Provides historical information on the price of each resource over the last
/// 14 days
///
/// Provide a resource type to get history for using `Some(ResourceType)`, or
/// get data for all resources by passing `None`
pub fn get_history(resource: Option<MarketResourceType>) -> Vec<OrderHistoryRecord> {
    match resource {
        Some(resource_type) => match resource_type {
            MarketResourceType::Resource(ty) => js!(
                const history = Game.market.getHistory(__resource_type_num_to_str(@{ty as u32}));
                if (history && history.length > 0) {
                    return history;
                } else {
                    return [];
                }
            )
            .try_into()
            .unwrap(),
            MarketResourceType::IntershardResource(ty) => js!(
                const history = Game.market.getHistory(__resource_type_num_to_str(@{ty as u32}));
                if (history && history.length > 0) {
                    return history;
                } else {
                    return [];
                }
            )
            .try_into()
            .unwrap(),
        },
        None => js_unwrap!(Game.market.getHistory()),
    }
}

pub fn get_order(id: &str) -> Option<Order> {
    let order = js! {
        return Game.market.getOrderById(@{id});
    };
    order.try_into().ok()
}
