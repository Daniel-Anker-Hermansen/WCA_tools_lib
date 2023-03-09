pub use wca_oauth::noauth::*;

#[tokio::main]
async fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("bruhh");
    let result = BaseOAuthBuilder
        .with_secret("A1w1yHnpVmiAQ0Kv6AjDd3B015uNWceZn8efutMUd0U".to_owned(),
            "_1wtjDno1RIIamtxoL8ygzXvkXLA1SHM4kg_xhxLvhw".to_owned(),
            "urn:ietf:wg:oauth:2.0:oob".to_owned())
        .with_manage_competition_scope()
        .authenticate_explicit(input)
        .await
        .unwrap()
        //.wcif("dsfgeneralforsamlingen2023")
        .me()
        .await;

    println!("{result}");
}
