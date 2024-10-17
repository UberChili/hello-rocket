#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world! We are trying to learn Rocket."
}

#[get("/world")]
fn world() -> &'static str {
    "hello, world!"
}

#[get("/hello/<name>")]
fn hello(name: &str) -> String {
    format!("hello, {}!", name)
}

#[launch]
fn rocket() -> _ {
    //rocket::build().mount("/", routes![index])
    //rocket::build().mount("/", routes![index, world])

    // Rocket example, but why?
    //rocket::build()
    //    .mount("/hello", routes![world])
    //    .mount("/hi", routes![world])

    // Working example
    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![world])
        .mount("/", routes![hello])
}
