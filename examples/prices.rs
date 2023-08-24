use reydenx::{
    client::{Auth, Client},
    model::platform::Platform,
    prices::get_prices,
};

fn main() {
    let mut client = Client::new(
        String::from("EMAIL"),
        String::from("PASSWORD"),
    );

    if let Ok(client) = client.auth() {
        let res = get_prices(client, Platform::Twitch);
        println!("{:#?}", res);
    }
}
