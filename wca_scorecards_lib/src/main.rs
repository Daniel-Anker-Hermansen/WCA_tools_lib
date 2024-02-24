fn main() {
    let mut args = std::env::args().skip(1);
    let wcif_path = args.next().unwrap();
    let stations = args.next().unwrap().parse().unwrap();
    wca_scorecards_lib::blank_for_subsequent_rounds(&wcif_path, stations);
}
