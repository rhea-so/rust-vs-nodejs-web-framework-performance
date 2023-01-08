use rocket::serde::{json::Json, Serialize};

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct GreetingMessage {
  text: String,
}

#[get("/")]
pub fn get() -> Json<GreetingMessage> {
  let response = Json(GreetingMessage {
    text: String::from("Hello, World!"),
  });
  return response;
}
