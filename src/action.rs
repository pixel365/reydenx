use std::error::Error;

use crate::{
    client::Requests,
    model::{order::LaunchParams, result::ActionResult, task::TaskStatus},
};

fn patch(
    c: &impl Requests,
    path: &str,
    payload: Option<String>,
) -> Result<ActionResult, Box<dyn Error>> {
    let resp = c.patch(path, payload)?;
    let res: ActionResult = serde_json::from_str(&resp)?;
    Ok(res)
}

/// Run order
///
/// ```rust, no_run
/// use reydenx::{
///     action::run,
///     client::{Auth, Client},
/// };
///
/// fn main() {
///     let mut client = Client::new(String::from("EMAIL"), String::from("PASSWORD"));
///     if let Ok(client) = client.auth() {
///         let res = run(client, 12345);
///         println!("{:#?}", res);
///     }
/// }
/// ```
pub fn run(c: &impl Requests, order_id: u32) -> Result<ActionResult, Box<dyn Error>> {
    self::patch(c, &format!("/orders/{}/action/run/", order_id), None)
}

/// Stop order
///
/// ```rust, no_run
/// use reydenx::{
///     action::stop,
///     client::{Auth, Client},
/// };
///
/// fn main() {
///     let mut client = Client::new(String::from("EMAIL"), String::from("PASSWORD"));
///     if let Ok(client) = client.auth() {
///         let res = stop(client, 12345);
///         println!("{:#?}", res);
///     }
/// }
/// ```
pub fn stop(c: &impl Requests, order_id: u32) -> Result<ActionResult, Box<dyn Error>> {
    self::patch(c, &format!("/orders/{}/action/stop/", order_id), None)
}

/// Cancel order
///
/// ```rust, no_run
/// use reydenx::{
///     action::cancel,
///     client::{Auth, Client},
/// };
///
/// fn main() {
///     let mut client = Client::new(String::from("EMAIL"), String::from("PASSWORD"));
///     if let Ok(client) = client.auth() {
///         let res = cancel(client, 12345);
///         println!("{:#?}", res);
///     }
/// }
/// ```
pub fn cancel(c: &impl Requests, order_id: u32) -> Result<ActionResult, Box<dyn Error>> {
    self::patch(c, &format!("/orders/{}/action/cancel/", order_id), None)
}

/// Change online viewers for order
///
/// ```rust, no_run
/// use reydenx::{
///     action::change_online,
///     client::{Auth, Client},
/// };
///
/// fn main() {
///     let mut client = Client::new(String::from("EMAIL"), String::from("PASSWORD"));
///     if let Ok(client) = client.auth() {
///         let res = change_online(client, 12345, 50);
///         println!("{:#?}", res);
///     }
/// }
/// ```
pub fn change_online(
    c: &impl Requests,
    order_id: u32,
    value: u32,
) -> Result<ActionResult, Box<dyn Error>> {
    self::patch(
        c,
        &format!("/orders/{}/action/change/online/{}/", order_id, value),
        None,
    )
}

/// Change increase time for order
///
/// ```rust, no_run
/// use reydenx::{
///     action::change_increase_time,
///     client::{Auth, Client},
/// };
///
/// fn main() {
///     let mut client = Client::new(String::from("EMAIL"), String::from("PASSWORD"));
///     if let Ok(client) = client.auth() {
///         let res = change_increase_time(client, 12345, 50);
///         println!("{:#?}", res);
///     }
/// }
/// ```
pub fn change_increase_time(
    c: &impl Requests,
    order_id: u32,
    value: u32,
) -> Result<ActionResult, Box<dyn Error>> {
    self::patch(
        c,
        &format!("/orders/{}/action/increase/change/{}/", order_id, value),
        None,
    )
}

/// Enable increase time for order
///
/// ```rust, no_run
/// use reydenx::{
///     action::enable_increase_of_viewers,
///     client::{Auth, Client},
/// };
///
/// fn main() {
///     let mut client = Client::new(String::from("EMAIL"), String::from("PASSWORD"));
///     if let Ok(client) = client.auth() {
///         let res = enable_increase_of_viewers(client, 12345, 50);
///         println!("{:#?}", res);
///     }
/// }
/// ```
pub fn enable_increase_of_viewers(
    c: &impl Requests,
    order_id: u32,
    value: u32,
) -> Result<ActionResult, Box<dyn Error>> {
    self::patch(
        c,
        &format!("/orders/{}/action/increase/on/{}/", order_id, value),
        None,
    )
}

/// Disable increase time for order
///
/// ```rust, no_run
/// use reydenx::{
///     action::disable_increase_of_viewers,
///     client::{Auth, Client},
/// };
///
/// fn main() {
///     let mut client = Client::new(String::from("EMAIL"), String::from("PASSWORD"));
///     if let Ok(client) = client.auth() {
///         let res = disable_increase_of_viewers(client, 12345);
///         println!("{:#?}", res);
///     }
/// }
/// ```
pub fn disable_increase_of_viewers(
    c: &impl Requests,
    order_id: u32,
) -> Result<ActionResult, Box<dyn Error>> {
    self::patch(
        c,
        &format!("/orders/{}/action/increase/off/", order_id),
        None,
    )
}

/// Add views to order
///
/// ```rust, no_run
/// use reydenx::{
///     action::add_views,
///     client::{Auth, Client},
/// };
///
/// fn main() {
///     let mut client = Client::new(String::from("EMAIL"), String::from("PASSWORD"));
///     if let Ok(client) = client.auth() {
///         let res = add_views(client, 12345, 50);
///         println!("{:#?}", res);
///     }
/// }
/// ```
pub fn add_views(
    c: &impl Requests,
    order_id: u32,
    value: u32,
) -> Result<ActionResult, Box<dyn Error>> {
    self::patch(
        c,
        &format!("/orders/{}/action/add/views/{}/", order_id, value),
        None,
    )
}

/// Check task status
///
/// ```rust, no_run
/// use reydenx::{
///     action::task_status,
///     client::{Auth, Client},
/// };
///
/// fn main() {
///     let mut client = Client::new(String::from("EMAIL"), String::from("PASSWORD"));
///     if let Ok(client) = client.auth() {
///         let res = task_status(client, 12345, &String::from("TASK_ID"));
///         println!("{:#?}", res);
///     }
/// }
/// ```
pub fn task_status(
    c: &impl Requests,
    order_id: u32,
    task_id: &str,
) -> Result<TaskStatus, Box<dyn Error>> {
    let resp = c.get(&format!("/orders/{}/task/{}/status/", order_id, task_id))?;
    let res: TaskStatus = serde_json::from_str(&resp)?;
    Ok(res)
}

/// Change launch mode
///
/// ```rust, no_run
/// use reydenx::{
///     action::change_launch_mode,
///     client::{Auth, Client},
///     model::order::{LaunchMode, LaunchParams},
/// };
///
/// fn main() {
///     let mut client = Client::new(String::from("EMAIL"), String::from("PASSWORD"));
///     if let Ok(client) = client.auth() {
///         let res = change_launch_mode(
///             client,
///             12345,
///             &LaunchParams {
///                 mode: LaunchMode::Delay,
///                 delay_time: 15,
///             },
///         );
///         println!("{:#?}", res);
///     }
/// }
/// ```
pub fn change_launch_mode(
    c: &impl Requests,
    order_id: u32,
    params: &LaunchParams,
) -> Result<ActionResult, Box<dyn Error>> {
    let payload = serde_json::to_string(params)?;
    self::patch(
        c,
        &format!("/orders/{}/action/change/launch/", order_id),
        Some(payload),
    )
}
