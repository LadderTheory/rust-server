#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

use rocket_contrib::json::{Json,JsonValue};

#[get("/")]
fn index() -> String {
    format!("{{\"apple\":8,\"eggs\":\"yea we got em\"}}")
}

#[get("/h?<id>&<cow>")]
fn h(id: Option<String>, cow: Option<String>) -> String {
    let id_str = id.map(|x| format!("id: {}", x))
        .unwrap_or_else(|| format!(""));
    
    let cow_str = cow.map(|x| format!("cow: {}", x))
        .unwrap_or_else(|| format!(""));

    let rtn = format!("{} {}\n", id_str, cow_str);

    rtn
}

#[get("/json")]
fn json() -> JsonValue {
    json!({
        "apple": 8,
        "eggs": "yea we got em",
    })
}

fn main() {
    let routes = routes![
        index,
        h,
        json,
    ];

    let roket = rocket::ignite().mount("/", routes);

    roket.launch();
}