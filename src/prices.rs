use std::error::Error;

use crate::{
    client::Requests,
    model::{platform::Platform, price::Price, result::StandardResult},
};

/// Returns all available rates for a specific platform
///
/// ```rust,no_run
/// use reydenx::{
///     client::{Auth, Client},
///     model::platform::Platform,
///     prices::get_prices,
/// };
/// fn main() {
///     let mut client = Client::new(
///         String::from("EMAIL"),
///         String::from("PASSWORD"),
///     );

///     if let Ok(client) = client.auth() {
///         let res = get_prices(client, Platform::Twitch);
///         println!("{:#?}", res);
///     }
/// }
/// ```
pub fn get_prices(
    c: &impl Requests,
    platform: Platform,
) -> Result<StandardResult<Vec<Price>>, Box<dyn Error>> {
    let resp = c.get(&format!("/prices/{}/", platform.slug()))?;
    let res: StandardResult<Vec<Price>> = serde_json::from_str(&resp)?;
    Ok(res)
}
