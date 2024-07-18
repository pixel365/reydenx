use std::error::Error;

use crate::{
    client::Requests,
    model::{
        platform::Platform, price::Price, price_category::PriceCategory, result::StandardResult,
    },
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
    let resp = c.get(&format!("/prices/{}/", platform))?;
    let res: StandardResult<Vec<Price>> = serde_json::from_str(&resp)?;
    Ok(res)
}

/// Returns all price categories
///
/// ```rust,no_run
/// use reydenx::{
///     client::{Auth, Client},
///     prices::get_categories,
/// };
/// fn main() {
///     let mut client = Client::new(
///         String::from("EMAIL"),
///         String::from("PASSWORD"),
///     );

///     if let Ok(client) = client.auth() {
///         let res = get_categories(client);
///         println!("{:#?}", res);
///     }
/// }
/// ```
pub fn get_categories(
    c: &impl Requests,
) -> Result<StandardResult<Vec<PriceCategory>>, Box<dyn Error>> {
    let resp = c.get("/price-categories/")?;
    let res: StandardResult<Vec<PriceCategory>> = serde_json::from_str(&resp)?;
    Ok(res)
}
