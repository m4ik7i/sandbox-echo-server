use serde::{Deserialize, Serialize};
use tide::{IntoResponse, Request, Response};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Message {
    msg: String,
}

#[async_std::main]
async fn main() -> std::io::Result<()> {
    let mut app = tide::new();
    app.at("/").get(|req: Request<()>| async move {
        let message = match req.query::<Message>() {
            Ok(ok) => ok,
            Err(err) => return err.into_response(),
        };
        Response::new(200).body_string(format!("{}", message.msg))
    });
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}
