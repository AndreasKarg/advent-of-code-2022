use std::sync::Arc;

use reqwest::Url;

pub fn fetch(advent_day: u8, session_id: &str) -> String {
    let puzzle_input_url = format!("https://adventofcode.com/2022/day/{advent_day}/input");
    let base_url = "https://adventofcode.com/2022".parse::<Url>().unwrap();

    let cookie_jar = reqwest::cookie::Jar::default();

    let session_cookie = format!("session={session_id}");
    cookie_jar.add_cookie_str(&session_cookie, &base_url);

    let client = reqwest::blocking::ClientBuilder::new()
        .cookie_provider(Arc::new(cookie_jar))
        .build()
        .unwrap();
    let resp = client.get(puzzle_input_url).send().unwrap();
    let resp = resp.error_for_status().unwrap();
    resp.text().unwrap()
}
