pub use wca_oauth::noauth::*;

#[tokio::main]
async fn main() {
    let result = BaseOAuthBuilder
        .with_manage_competition_scope()
        .with_secret("A1w1yHnpVmiAQ0Kv6AjDd3B015uNWceZn8efutMUd0U".to_owned(),
            "_1wtjDno1RIIamtxoL8ygzXvkXLA1SHM4kg_xhxLvhw".to_owned(),
            "urn:ietf:wg:oauth:2.0:oob".to_owned())
        .authenticate_explicit("uzeXn-pWXfTIBCAK259ZNkwRMBFiceQKYRHA0pEza6o".to_owned())
        .await
        .unwrap()
        .wcif("dsfgeneralforsamlingen2023")
        .await;

    println!("{result}");
}
