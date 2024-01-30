use reydenx::{
    client::{Auth, Client},
    traffic,
};

fn main() {
    let mut client = Client::new(String::from("EMAIL"), String::from("PASSWORD"));

    if let Ok(client) = client.auth() {
        let res = traffic::countries(client);
        println!("{:#?}", res);

        let res = traffic::languages(client);
        println!("{:#?}", res);

        let res = traffic::devices(client);
        println!("{:#?}", res);
    }
}
