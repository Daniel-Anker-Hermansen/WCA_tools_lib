fn main() {
    let data = include_str!("json");
    dbg!(wca_oauth::Competition::from_json(&data));
}
