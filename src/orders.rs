use std::error::Error;

use crate::{
    client::Requests,
    model::{
        order::{
            DateAndQuantity, IdAndQuantity, Identifiers, OnlineStats, Order, Payment, SiteStats,
            TwitchPayload, YouTubePayload,
        },
        platform::Platform,
        result::{ActionResult, StandardResult},
    },
};
use crate::model::order::KickPayload;

/// Return list of orders
///
/// By default it returns 50 items
/// ```rust,no_run
/// use reydenx::{
///     client::{Auth, Client},
///     orders::all_orders,
/// };
//
/// fn main() {
///     let mut client = Client::new(String::from("EMAIL"), String::from("PASSWORD"));
///     if let Ok(client) = client.auth() {
///         let res = all_orders(client, None);
///         println!("{:#?}", res);
///     }
/// }
/// ```
///
/// If you need to get all orders, check cursor
/// ```rust,no_run
/// use reydenx::{
///     client::{Auth, Client},
///     model::order::Order,
///     orders::all_orders,
/// };
///
/// fn main() {
///     let mut client = Client::new(String::from("EMAIL"), String::from("PASSWORD"));
///     if let Ok(client) = client.auth() {
///         let mut next = true;
///         let mut cursor: Option<String> = None;
///         let mut orders: Vec<Order> = Vec::new();
///         while next {
///             match all_orders(client, cursor.clone()) {
///                 Ok(res) => {
///                     next = res.has_next();
///                     cursor = res.cursor;
///                     for order in res.result {
///                         orders.push(order)
///                     }
///                 }
///                 Err(_) => next = false,
///             }
///         }
///         println!("{:#?}", orders)
///     }
/// }
/// ```
pub fn all_orders(
    c: &impl Requests,
    cursor: Option<String>,
) -> Result<StandardResult<Vec<Order>>, Box<dyn Error>> {
    let url = match cursor {
        Some(cur) => format!("/orders/?cursor={}", cur),
        None => String::from("/orders/"),
    };
    let resp = c.get(&url)?;
    let res: StandardResult<Vec<Order>> = serde_json::from_str(&resp)?;
    Ok(res)
}

/// Order details by id
///
/// ```rust,no_run
/// use reydenx::{
///     client::{Auth, Client},
///     orders::order_details,
/// };
/// fn main() {
///     let mut client = Client::new(
///         String::from("USERNAME"),
///         String::from("PASSWORD"),
///     );
///     if let Ok(client) = client.auth() {
///         let res = order_details(client, 12345);
///         println!("{:#?}", res);
///     }
/// }
/// ```
pub fn order_details(
    c: &impl Requests,
    order_id: u32,
) -> Result<StandardResult<Order>, Box<dyn Error>> {
    let resp = c.get(&format!("/orders/{}/", order_id))?;
    let res: StandardResult<Order> = serde_json::from_str(&resp)?;
    Ok(res)
}

/// Detailed information about users online
///
/// ```rust,no_run
/// use reydenx::{
///     client::{Auth, Client},
///     orders::online_stats,
/// };
/// fn main() {
///     let mut client = Client::new(
///         String::from("USERNAME"),
///         String::from("PASSWORD"),
///     );
///     if let Ok(client) = client.auth() {
///         let res = online_stats(client, 12345);
///         println!("{:#?}", res);
///     }
/// }
/// ```
pub fn online_stats(
    c: &impl Requests,
    order_id: u32,
) -> Result<StandardResult<Vec<OnlineStats>>, Box<dyn Error>> {
    let resp = c.get(&format!("/orders/{}/statistics/online/", order_id))?;
    let res: StandardResult<Vec<OnlineStats>> = serde_json::from_str(&resp)?;
    Ok(res)
}

/// Detailed information about clicks
///
/// ```rust,no_run
/// use reydenx::{
///     client::{Auth, Client},
///     orders::clicks_stats,
/// };
/// fn main() {
///     let mut client = Client::new(
///         String::from("USERNAME"),
///         String::from("PASSWORD"),
///     );
///     if let Ok(client) = client.auth() {
///         let res = clicks_stats(client, 12345);
///         println!("{:#?}", res);
///     }
/// }
/// ```
pub fn clicks_stats(
    c: &impl Requests,
    order_id: u32,
) -> Result<StandardResult<Vec<DateAndQuantity>>, Box<dyn Error>> {
    let resp = c.get(&format!("/orders/{}/statistics/clicks/", order_id))?;
    let res: StandardResult<Vec<DateAndQuantity>> = serde_json::from_str(&resp)?;
    Ok(res)
}

/// Detailed information about views
///
/// ```rust,no_run
/// use reydenx::{
///     client::{Auth, Client},
///     orders::views_stats,
/// };
/// fn main() {
///     let mut client = Client::new(
///         String::from("USERNAME"),
///         String::from("PASSWORD"),
///     );
///     if let Ok(client) = client.auth() {
///         let res = views_stats(client, 12345);
///         println!("{:#?}", res);
///     }
/// }
/// ```
pub fn views_stats(
    c: &impl Requests,
    order_id: u32,
) -> Result<StandardResult<Vec<DateAndQuantity>>, Box<dyn Error>> {
    let resp = c.get(&format!("/orders/{}/statistics/views/", order_id))?;
    let res: StandardResult<Vec<DateAndQuantity>> = serde_json::from_str(&resp)?;
    Ok(res)
}

/// Detailed information about sites
///
/// ```rust,no_run
/// use reydenx::{
///     client::{Auth, Client},
///     orders::sites_stats,
/// };
/// fn main() {
///     let mut client = Client::new(
///         String::from("USERNAME"),
///         String::from("PASSWORD"),
///     );
///     if let Ok(client) = client.auth() {
///         let res = sites_stats(client, 12345);
///         println!("{:#?}", res);
///     }
/// }
/// ```
pub fn sites_stats(
    c: &impl Requests,
    order_id: u32,
) -> Result<StandardResult<Vec<SiteStats>>, Box<dyn Error>> {
    let resp = c.get(&format!("/orders/{}/statistics/sites/", order_id))?;
    let res: StandardResult<Vec<SiteStats>> = serde_json::from_str(&resp)?;
    Ok(res)
}

/// Return list of order payments
///
/// By default it returns 50 items
/// ```rust,no_run
/// use reydenx::{
///     client::{Auth, Client},
///     orders::payments,
/// };
//
/// fn main() {
///     let mut client = Client::new(String::from("EMAIL"), String::from("PASSWORD"));
///     if let Ok(client) = client.auth() {
///         let res = payments(client, 12345, None);
///         println!("{:#?}", res);
///     }
/// }
/// ```
///
/// If you need to get all payments, check cursor
/// ```rust,no_run
/// use reydenx::{
///     client::{Auth, Client},
///     model::order::Payment,
///     orders::payments,
/// };
///
/// fn main() {
///     let mut client = Client::new(String::from("EMAIL"), String::from("PASSWORD"));
///     if let Ok(client) = client.auth() {
///         let mut next = true;
///         let mut cursor: Option<String> = None;
///         let mut payments_vec: Vec<Payment> = Vec::new();
///         while next {
///             match payments(client, 12345, cursor.clone()) {
///                 Ok(res) => {
///                     next = res.has_next();
///                     cursor = res.cursor;
///                     for payment in res.result {
///                         payments_vec.push(payment)
///                     }
///                 }
///                 Err(_) => next = false,
///             }
///         }
///         println!("{:#?}", payments_vec)
///     }
/// }
/// ```
pub fn payments(
    c: &impl Requests,
    order_id: u32,
    cursor: Option<String>,
) -> Result<StandardResult<Vec<Payment>>, Box<dyn Error>> {
    let url = match cursor {
        Some(cur) => format!("/orders/{}/payments/?cursor={}", order_id, cur),
        None => format!("/orders/{}/payments/", order_id),
    };
    let resp = c.get(&url)?;
    let res: StandardResult<Vec<Payment>> = serde_json::from_str(&resp)?;
    Ok(res)
}

/// Create new order for Twitch stream
///
/// ```rust,no_run
/// use reydenx::{
///     client::{Auth, Client},
///     model::order::{SmoothGain, TwitchPayload},
///     orders::create_twitch_stream,
/// };
///
/// fn main() {
///     let mut client = Client::new(
///         String::from("USERNAME"),
///         String::from("PASSWORD"),
///     );
///     if let Ok(client) = client.auth() {
///         let res = create_twitch_stream(
///             client,
///             &TwitchPayload {
///                 price_id: 123,
///                 number_of_views: 1000,
///                 number_of_viewers: 100,
///                 launch_mode: String::from("auto"),
///                 smooth_gain: SmoothGain {
///                     enabled: false,
///                     minutes: 0,
///                 },
///                 delay_time: 0,
///                 twitch_id: 123456789,
///                 fixed_allocation: 0,
///                 on_overflow: false,
///             },
///         );
///         println!("{:#?}", res);
///     }
/// }
/// ```
pub fn create_twitch_stream(
    c: &impl Requests,
    payload: &TwitchPayload,
) -> Result<ActionResult, Box<dyn Error>> {
    let payload = serde_json::to_string(payload)?;
    let resp = c.post(
        &format!("/orders/create/{}/stream/", Platform::Twitch),
        payload,
    )?;
    let res: ActionResult = serde_json::from_str(&resp)?;
    Ok(res)
}

/// Create new order for YouTube stream
///
/// ```rust,no_run
/// use reydenx::{
///     client::{Auth, Client},
///     model::order::{SmoothGain, YouTubePayload},
///     orders::create_youtube_stream,
/// };
///
/// fn main() {
///     let mut client = Client::new(
///         String::from("USERNAME"),
///         String::from("PASSWORD"),
///     );
///     if let Ok(client) = client.auth() {
///         let res = create_youtube_stream(
///             client,
///             &YouTubePayload {
///                 price_id: 123,
///                 number_of_views: 1000,
///                 number_of_viewers: 100,
///                 launch_mode: String::from("auto"),
///                 smooth_gain: SmoothGain {
///                     enabled: false,
///                     minutes: 0,
///                 },
///                 delay_time: 0,
///                 channel_url: String::from(
///                     "https://www.youtube.com/channel/UCtI0Hodo5o5dUb67FeUjDeA",
///                 ),
///                 fixed_allocation: 0,
///                 on_overflow: false,
///             },
///         );
///         println!("{:#?}", res);
///     }
/// }
/// ```
pub fn create_youtube_stream(
    c: &impl Requests,
    payload: &YouTubePayload,
) -> Result<ActionResult, Box<dyn Error>> {
    let payload = serde_json::to_string(payload)?;
    let resp = c.post(
        &format!("/orders/create/{}/stream/", Platform::YouTube),
        payload,
    )?;
    let res: ActionResult = serde_json::from_str(&resp)?;
    Ok(res)
}

/// Create new order for Kick stream
///
/// ```rust,no_run
/// use reydenx::{
///     client::{Auth, Client},
///     model::order::{SmoothGain, YouTubePayload},
///     orders::create_kick_stream,
/// };
///
/// fn main() {
///     use reydenx::model::order::KickPayload;
/// let mut client = Client::new(
///         String::from("USERNAME"),
///         String::from("PASSWORD"),
///     );
///     if let Ok(client) = client.auth() {
///         let res = create_kick_stream(
///             client,
///             &KickPayload {
///                 price_id: 123,
///                 number_of_views: 1000,
///                 number_of_viewers: 100,
///                 launch_mode: String::from("auto"),
///                 smooth_gain: SmoothGain {
///                     enabled: false,
///                     minutes: 0,
///                 },
///                 delay_time: 0,
///                 channel_url: String::from(
///                     "https://kick.com/channel",
///                 ),
///                 fixed_allocation: 0,
///                 on_overflow: false,
///             },
///         );
///         println!("{:#?}", res);
///     }
/// }
/// ```
pub fn create_kick_stream(
    c: &impl Requests,
    payload: &KickPayload,
) -> Result<ActionResult, Box<dyn Error>> {
    let payload = serde_json::to_string(payload)?;
    let resp = c.post(
        &format!("/orders/create/{}/stream/", Platform::Kick),
        payload,
    )?;
    let res: ActionResult = serde_json::from_str(&resp)?;
    Ok(res)
}

/// Views statistics for multiple orders
/// 
/// ```rust,no_run
/// use reydenx::{
///     client::{Auth, Client},
///     model::order::Identifiers,
///     orders::multiple_views_stats,
/// };
///
/// fn main() {
///     let mut client = Client::new(String::from("USERNAME"), String::from("PASSWORD"));
///     if let Ok(client) = client.auth() {
///         let res = multiple_views_stats(
///             client,
///             Identifiers {
///                 identifiers: vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
///             },
///         );
///         println!("{:#?}", res);
///     }
/// } 
/// ```
pub fn multiple_views_stats(
    c: &impl Requests,
    identifiers: Identifiers,
) -> Result<StandardResult<Vec<IdAndQuantity>>, Box<dyn Error>> {
    let payload = serde_json::to_string(&identifiers)?;
    let resp = c.post("/orders/multiple/views/", payload)?;
    let res: StandardResult<Vec<IdAndQuantity>> = serde_json::from_str(&resp)?;
    Ok(res)
}

/// Click-through statistics for multiple orders
/// 
/// ```rust,no_run
/// use reydenx::{
///     client::{Auth, Client},
///     model::order::Identifiers,
///     orders::multiple_clicks_stats,
/// };
///
/// fn main() {
///     let mut client = Client::new(String::from("USERNAME"), String::from("PASSWORD"));
///     if let Ok(client) = client.auth() {
///         let res = multiple_clicks_stats(
///             client,
///             Identifiers {
///                 identifiers: vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
///             },
///         );
///         println!("{:#?}", res);
///     }
/// } 
/// ```
pub fn multiple_clicks_stats(
    c: &impl Requests,
    identifiers: Identifiers,
) -> Result<StandardResult<Vec<IdAndQuantity>>, Box<dyn Error>> {
    let payload = serde_json::to_string(&identifiers)?;
    let resp = c.post("/orders/multiple/clicks/", payload)?;
    let res: StandardResult<Vec<IdAndQuantity>> = serde_json::from_str(&resp)?;
    Ok(res)
}
