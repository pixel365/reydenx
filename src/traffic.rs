use std::error::Error;

use crate::{
    client::Requests,
    model::{result::StandardResult, traffic::Traffic},
};

/// Traffic statistics by country
///
/// ```rust,no_run
/// use reydenx::{
///     client::{Auth, Client},
///     traffic,
/// };
/// fn main() {
///     let mut client = Client::new(
///         String::from("USERNAME"),
///         String::from("PASSWORD"),
///     );
///     if let Ok(client) = client.auth() {
///         let res = traffic::countries(client);
///         println!("{:#?}", res);
///     }
/// }
/// ```
pub fn countries(c: &impl Requests) -> Result<StandardResult<Vec<Traffic>>, Box<dyn Error>> {
    let resp = c.get("/traffic/countries/")?;
    let res: StandardResult<Vec<Traffic>> = serde_json::from_str(&resp)?;
    Ok(res)
}

/// Traffic statistics by language
///
/// ```rust,no_run
/// use reydenx::{
///     client::{Auth, Client},
///     traffic,
/// };
/// fn main() {
///     let mut client = Client::new(
///         String::from("USERNAME"),
///         String::from("PASSWORD"),
///     );
///     if let Ok(client) = client.auth() {
///         let res = traffic::languages(client);
///         println!("{:#?}", res);
///     }
/// }
/// ```
pub fn languages(c: &impl Requests) -> Result<StandardResult<Vec<Traffic>>, Box<dyn Error>> {
    let resp = c.get("/traffic/languages/")?;
    let res: StandardResult<Vec<Traffic>> = serde_json::from_str(&resp)?;
    Ok(res)
}
