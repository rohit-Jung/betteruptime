use poem::{get, handler, listener::TcpListener, post, web::Path, Route, Server};

pub mod routes;
pub mod types;

use routes::user;

use crate::routes::user::{signin_user, signup_user};

#[handler]
fn hello(Path(name): Path<String>) -> String {
    format!("hello: {}", name)
}

#[tokio::main(flavor = "multi_thread", worker_threads = 10)]
async fn main() -> Result<(), std::io::Error> {
    let app = Route::new()
        .at("/user/signin", get(signin_user))
        .at("/user/signup", post(signup_user))
        .at("/hello/:name", get(hello));

    println!("Server running on port 3000");
    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(app)
        .await
}
