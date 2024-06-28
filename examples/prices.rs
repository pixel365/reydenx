use reydenx::{
    client::{Auth, Client},
    model::platform::Platform,
    prices::{get_categories, get_prices},
};

fn main() {
    let mut client = Client::new(String::from("EMAIL"), String::from("PASSWORD"));

    if let Ok(client) = client.auth() {
        let res = get_prices(client, Platform::Twitch);
        println!("{:#?}", res);

        let res = get_categories(client);
        println!("{:#?}", res);
    }
}
