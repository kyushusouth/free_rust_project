use chrono::Utc;
use free::item::Item;
use free::pipeline;
use free::request::{Condition, Location, Request};
use free::stages::retrieval;

fn request_with(request_number: u32, user_name: &str) -> Request {
    Request {
        request_number,
        user_name: user_name.to_string(),
        query: "query".to_string(),
        location: Location {
            latitude: 0.0,
            longitude: 0.0,
        },
        time: Utc::now(),
        condition: Condition::Sunny,
    }
}

fn expected_item(id: &str, name: &str, score: f64, user_name: &str) -> Item {
    let mut item = retrieval::item(id, name, score);
    item.explanation = Some(format!(
        "{} さんの検索意図に合致するため推薦されました",
        user_name
    ));
    item
}

#[test]
fn run_returns_items_sorted_and_annotated_for_request_number_1() {
    let request = request_with(1, "Alice");
    let actual = pipeline::run(&request);

    let expected = vec![
        expected_item("1", "Item A", 0.5, "Alice"),
        expected_item("2", "Item B", 0.3, "Alice"),
    ];

    assert_eq!(actual.items, expected);
}

#[test]
fn run_returns_items_sorted_and_annotated_for_request_number_2() {
    let request = request_with(2, "Alice");
    let actual = pipeline::run(&request);

    let expected = vec![
        expected_item("3", "Item C", 0.9, "Alice"),
        expected_item("5", "Item E", 0.7, "Alice"),
        expected_item("4", "Item D", 0.1, "Alice"),
    ];

    assert_eq!(actual.items, expected);
}

#[test]
fn run_returns_empty_items_for_other_request_number() {
    let request = request_with(3, "Alice");
    let actual = pipeline::run(&request);

    assert!(actual.items.is_empty());
}

#[test]
fn run_reflects_user_name_in_explanation() {
    let request = request_with(1, "Bob");
    let actual = pipeline::run(&request);

    for item in actual.items {
        assert_eq!(
            item.explanation,
            Some("Bob さんの検索意図に合致するため推薦されました".to_string())
        );
    }
}
