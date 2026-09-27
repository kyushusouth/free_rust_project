use chrono::Utc;
use free::pipeline;
use free::request::{Request, Location, Condition};

fn main() {
    let request = Request {
        request_number: 1,
        user_name: "Alice".to_string(),
        query: "美味しいラーメン".to_string(),
        location: Location { latitude: 35.4437, longitude: 139.6380 },
        time: Utc::now(),
        condition: Condition::Cloudy,
    };

    let response = pipeline::run(&request);
    println!("{:#?}", response);
}
