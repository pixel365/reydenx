use std::error::Error;

use crate::{
    client::Requests,
    model::user::{Balance, User},
};

/// Get user balance
///
/// ```rust,no_run
/// use reydenx::{
///     client::{Auth, Client},
///     user::balance,
/// };
/// fn main() {
///     let mut client = Client::new(String::from("EMAIL"), String::from("PASSWORD"));
///     if let Ok(client) = client.auth() {
///         let res = balance(client);
///         println!("{:#?}", res);
///     }
/// }
/// ```
pub fn balance(c: &impl Requests) -> Result<Balance, Box<dyn Error>> {
    let resp = c.get("/user/balance/")?;
    let res: Balance = serde_json::from_str(&resp)?;
    Ok(res)
}

/// Get user account
///
/// ```rust,no_run
/// use reydenx::{
///     client::{Auth, Client},
///     user::account,
/// };
/// fn main() {
///     let mut client = Client::new(String::from("EMAIL"), String::from("PASSWORD"));
///     if let Ok(client) = client.auth() {
///         let res = account(client);
///         println!("{:#?}", res);
///     }
/// }
/// ```
pub fn account(c: &impl Requests) -> Result<User, Box<dyn Error>> {
    let resp = c.get("/user/")?;
    let res: User = serde_json::from_str(&resp)?;
    Ok(res)
}
