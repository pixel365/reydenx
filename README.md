# REYDEN-X

###### Reyden-X is an automated service for promoting live broadcasts on external sites with integrated system of viewers and views management.

- [Website](https://reyden-x.com/en)

- [API Documentation](https://api.reyden-x.com/docs)

### Quickstart

```rust
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
```
