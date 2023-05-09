pub use wca_oauth::noauth::*;

#[tokio::main]
async fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("bruhh");
    let result = BaseOAuthBuilder
        .staging()
        .with_secret("HK5EG0aL5co1dwtO6aBSeHqRMkbQgRq52xxG8ilqPes".to_owned(),
            "FcEvy_FI92tLWgmL5hy41x8vCc7Crfo-153m42AFNtI".to_owned(),
            "urn:ietf:wg:oauth:2.0:oob".to_owned())
        .with_manage_competition_scope()
        .authenticate_explicit(input)
        .await
        .unwrap()
        .me()
        .send().await;

    println!("{result:?}");
}
