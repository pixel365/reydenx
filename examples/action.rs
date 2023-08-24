use reydenx::{
    action::{
        add_views, cancel, change_increase_time, change_online, disable_increase_of_viewers,
        enable_increase_of_viewers, run, stop, task_status,
    },
    client::{Auth, Client},
};

fn main() {
    let mut client = Client::new(String::from("EMAIL"), String::from("PASSWORD"));

    if let Ok(client) = client.auth() {
        let res = run(client, 12345);
        println!("{:#?}", res);

        let res = stop(client, 12345);
        println!("{:#?}", res);

        let res = cancel(client, 12345);
        println!("{:#?}", res);

        let res = change_online(client, 12345, 50);
        println!("{:#?}", res);

        let res = change_increase_time(client, 12345, 50);
        println!("{:#?}", res);

        let res = enable_increase_of_viewers(client, 12345, 50);
        println!("{:#?}", res);

        let res = disable_increase_of_viewers(client, 12345);
        println!("{:#?}", res);

        let res = add_views(client, 12345, 50);
        println!("{:#?}", res);

        let res = task_status(client, 12345, &String::from("TASK_ID"));
        println!("{:#?}", res);
    }
}
