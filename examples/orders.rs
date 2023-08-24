use reydenx::{
    client::{Auth, Client},
    model::order::{Identifiers, Order, Payment, SmoothGain, TwitchPayload, YouTubePayload},
    orders::{
        all_orders, clicks_stats, create_twitch_stream, create_youtube_stream,
        multiple_clicks_stats, multiple_views_stats, online_stats, order_details, payments,
        views_stats,
    },
};

fn main() {
    let mut client = Client::new(String::from("EMAIL"), String::from("PASSWORD"));

    if let Ok(client) = client.auth() {
        let res = all_orders(client, None);
        println!("{:#?}", res);

        let mut next = true;
        let mut cursor: Option<String> = None;
        let mut orders: Vec<Order> = Vec::new();

        while next {
            match all_orders(client, cursor.clone()) {
                Ok(res) => {
                    next = res.has_next();
                    cursor = res.cursor;
                    for order in res.result {
                        orders.push(order)
                    }
                }
                Err(_) => next = false,
            }
        }

        println!("{:#?}", orders);

        let res = order_details(client, 12345);
        println!("{:#?}", res);

        let res = online_stats(client, 12345);
        println!("{:#?}", res);

        let res = clicks_stats(client, 12345);
        println!("{:#?}", res);

        let res = views_stats(client, 12345);
        println!("{:#?}", res);

        let res = payments(client, 12345, None);
        println!("{:#?}", res);

        let mut next = true;
        let mut cursor: Option<String> = None;
        let mut payments_vec: Vec<Payment> = Vec::new();

        while next {
            match payments(client, 12345, cursor.clone()) {
                Ok(res) => {
                    next = res.has_next();
                    cursor = res.cursor;
                    for payment in res.result {
                        payments_vec.push(payment)
                    }
                }
                Err(_) => next = false,
            }
        }

        println!("{:#?}", payments_vec);

        let res = create_twitch_stream(
            client,
            &TwitchPayload {
                price_id: 123,
                number_of_views: 1000,
                number_of_viewers: 100,
                launch_mode: String::from("auto"),
                smooth_gain: SmoothGain {
                    enabled: false,
                    minutes: 0,
                },
                delay_time: 0,
                twitch_id: 123456789,
            },
        );
        println!("{:#?}", res);

        let res = create_youtube_stream(
            client,
            &YouTubePayload {
                price_id: 123,
                number_of_views: 1000,
                number_of_viewers: 100,
                launch_mode: String::from("auto"),
                smooth_gain: SmoothGain {
                    enabled: false,
                    minutes: 0,
                },
                delay_time: 0,
                channel_url: String::from(
                    "https://www.youtube.com/channel/UCtI0Hodo5o5dUb67FeUjDeA",
                ),
            },
        );
        println!("{:#?}", res);

        let res = multiple_views_stats(
            client,
            Identifiers {
                identifiers: vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            },
        );
        println!("{:#?}", res);

        let res = multiple_clicks_stats(
            client,
            Identifiers {
                identifiers: vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            },
        );
        println!("{:#?}", res);
    }
}
