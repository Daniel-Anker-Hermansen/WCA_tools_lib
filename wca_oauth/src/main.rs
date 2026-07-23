use wca_oauth::Wcif;

fn main() {
	let wcif = include_str!("../sydals2.json");
	let wcif: Wcif = serde_json::from_str(wcif).unwrap();
}
