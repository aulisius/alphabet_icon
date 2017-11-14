#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate cached;
#[macro_use]
extern crate lazy_static;
extern crate svg;
extern crate rocket;
extern crate rocket_cors;

use cached::SizedCache;
use rocket_cors::Cors;
use rocket::response::Redirect;
use svg::Document;
use svg::node::element::{Circle, Rectangle, Text};

#[derive(FromForm)]
struct Icon {
    name: String,
    square: Option<bool>,
}

fn get_initials(name: String) -> String {
    name.split_whitespace()
        .take(2)
        .map(|n| n.chars().nth(0).unwrap())
        .collect()
}

#[get("/avatar?<icon>")]
fn index(icon: Icon) -> Redirect {
    let name = get_initials(icon.name).to_uppercase();
    let shape = icon.square.unwrap_or(false);
    let url = format!("/api/{}/{}", name, shape);
    Redirect::to(url.as_str())
}

cached!{ DATA: SizedCache = SizedCache::with_capacity(100); >>
fn get_data_uri(name: String, square: bool) -> String = {
    let tmp_doc = Document::new().set("viewBox", (0, 0, 10, 10));

    let final_doc = if square {
        let square = Rectangle::new()
            .set("x", 0)
            .set("y", 0)
            .set("width", 10)
            .set("height", 10)
            .set("fill", "#000000");
        tmp_doc.add(square)
    } else {
        let circle = Circle::new()
            .set("r", 5)
            .set("cx", 5)
            .set("cy", 5)
            .set("fill", "#000000");
        tmp_doc.add(circle)
    };

    let (font_size, y) = match name.len() {
        1 => ("6px", 7),
        2 => ("5px", 7),
        _ => ("4px", 5)
    };

    let data = svg::node::Text::new(name);

    let text = Text::new()
        .set("fill", "#ffffff")
        .set("font-size", font_size)
        .set("font-family", "Arial")
        .set("font-style", "bold")
        .set("text-anchor", "middle")
        .set("x", 5)
        .set("y", y)
        .add(data);

    let document = final_doc.add(text);

    let svg = format!("data:image/svg+xml;utf8,{}", document);
    svg.replace("\n", "")
}}

#[get("/<name>/<square>")]
fn get_icon(name: String, square: bool) -> String {
    get_data_uri(name, square)
}

fn main() {
    rocket::ignite()
        .mount("/api", routes![index, get_icon])
        .attach(Cors::default())
        .launch();
}
