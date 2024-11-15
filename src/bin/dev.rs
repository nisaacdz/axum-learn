use axum_learn::User;
use httpc_test::new_client;
use serde_json::json;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let hc = new_client("http://localhost:8080").unwrap();

    // Serialize `CreateUser` and send it as JSON in the request
    let users_response = hc
        .do_post("/users", json!({ "username": "Kofi"}))
        .await
        .unwrap();

    // Parse the JSON response into a `User` instance
    let user = users_response.json_body_as::<User>().unwrap();
    println!("{:?}", user);

    hc.do_post(
        "/api/login",
        json!({"username": "isaacJ", "password": "idontknow"}),
    )
    .await
    .unwrap()
    .print()
    .await
    .unwrap();
    Ok(())
}
