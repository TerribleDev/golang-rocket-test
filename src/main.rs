#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

// #[launch]
// fn rocket() -> _ {
//     // let cfg =  rocket::config::Config
//     // .address("0.0.0.0")
//     // .port(port)   
//     // .unwrap();
//     // rocket::build()
//     //.configure(cfg)
//     // .mount("/", routes![index])
//     // .launch()
//     rocket::ignite().mount("/", routes![index]).launch();
// }
#[launch]
fn rocket() -> _ {
    let port = std::env::var("PORT").unwrap_or("8000".to_string());
    let port_as_int = port.parse::<u16>().unwrap();
    let figmant = rocket::Config::figment().merge(("port", port_as_int)).merge(("address", "0.0.0.0"));

    rocket::custom(figmant).mount("/", routes![index])
}