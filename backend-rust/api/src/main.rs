use poem::{
    get, handler,
    listener::TcpListener,
    web::{Json, Path},
    Route, Server,
};
use serde::{Deserialize, Serialize};

#[handler]
fn hello(Path(name): Path<String>) -> String {
    format!("hello: {}", name)
}

#[derive(Serialize, Deserialize)]
pub struct UserSignInResponse {
    id: String,
    token: String,
}

#[handler]
fn signin_user() -> Json<UserSignInResponse> {
    let response = UserSignInResponse {
        id: String::from("id"),
        token: String::from("token"),
    };

    Json(response)
}

#[tokio::main(flavor = "multi_thread", worker_threads = 10)]
async fn main() -> Result<(), std::io::Error> {
    let app = Route::new()
        .at("/hello/:name", get(hello))
        .at("/user/signin", get(signin_user));

    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(app)
        .await
}
