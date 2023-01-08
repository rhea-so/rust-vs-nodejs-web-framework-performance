mod routes;

#[macro_use]
extern crate rocket;

#[launch]
fn rocket() -> _ {
  println!("Server started!");
  rocket::build().mount("/", routes![routes::index::get])
}
