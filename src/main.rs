//use std::io;
use chrono::prelude::*;
use actix_web::{web, App, HttpResponse, HttpServer};

#[actix_web::main]
async fn main() {
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(get_index))
            .route("/age", web::post().to(post_age))
    });

    println!("Serving on http://localhost:3000...");
    server
        .bind("127.0.0.1:3000").expect("error binding server to address")
        .run()
        .await
        .expect("error running server");
}

async fn get_index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(
            r#"
                <title>GCD Calculator</title>
                <form action="/age" method="post">
                <head># Age Calculator  </head>
                <button type="submit"> Compute Age </button>
                <br><br>
                <input type="text" name="name">: Name</input><br><br>
                <input type="text" name="dob">: Date of Birth(yyyy.mm.dd)</input><br><br>
                
                </form>
            "#,
        )
}

use serde::Deserialize;
#[derive(Deserialize)]
struct Agecalc {
    name: String,
    dob: String,
}

async fn post_age(form: web::Form<Agecalc>) -> HttpResponse {
    
    let today = Local::now().naive_local().date();
    // Parse the date of birth
    let dob = NaiveDate::parse_from_str(&form.dob.trim(), "%Y.%m.%d")
        .expect("Please enter a valid date");

    let response =
        format!("Name: {}<br>Date of Birth: {}<br>Age: {}",
                form.name, form.dob, age(today, dob));

    HttpResponse::Ok()
        .content_type("text/html")
        .body(response)
}

fn age(now: NaiveDate, dob: NaiveDate) -> u32 {
    // Calculate age
    let age = now.year() as u32 - dob.year() as u32;

    // Check if the birthday has occurred this year
    let has_birthday_occurred = now.month() > dob.month()
        || (now.month() == dob.month() && now.day() >= dob.day());

    // Adjust age based on whether the birthday has occurred this year
    let final_age = if has_birthday_occurred { age } else { age - 1 };

    final_age
}
