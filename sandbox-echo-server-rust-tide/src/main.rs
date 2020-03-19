use serde::{Deserialize, Serialize};
use tide::Request;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Message {
    msg: String,
}

#[async_std::main]
async fn main() -> std::io::Result<()> {
    let mut app = tide::new();
    app.at("/").get(|req: Request<()>| async move {
        if let Ok(query) = req.query::<Message>() {
            format!("{}", query.msg)
        } else {
            format!("no message")
        }
    });
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}
