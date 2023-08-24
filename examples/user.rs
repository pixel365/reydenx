use reydenx::{
    client::{Auth, Client},
    user::{account, balance},
};

fn main() {
    let mut client = Client::new(String::from("EMAIL"), String::from("PASSWORD"));

    if let Ok(client) = client.auth() {
        let res = balance(client);
        println!("{:#?}", res);

        let res = account(client);
        println!("{:#?}", res);
    }
}
