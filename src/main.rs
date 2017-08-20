#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate serde_json;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate debug_rs;

extern crate juniper;
extern crate juniper_rocket;

use std::path::{Path, PathBuf};

use rocket::response::NamedFile;
use rocket::State;
use rocket::config::{Config, Environment};
use rocket::response::content;
use rocket_contrib::Template;

use juniper::{EmptyMutation, RootNode};

type Schema = RootNode<'static, None, EmptyMutation<None>>;

#[get("/graphiql")]
fn graphiql() -> content::Html<String> {
    juniper_rocket::graphiql_source("/graphql")
}

#[get("/graphql?<request>")]
fn graphql(
    context: State<None>,
    request: juniper_rocket::GraphQLRequest,
    schema: State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &context)
}


#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static").join(file)).ok()
}

fn main() {
    debug!("hello");
    rocket::Rocket::ignite()
        .manage(Schema::new(None, EmptyMutation::<None>::new()))
        .mount("/", routes![graphiql, graphql])
        .mount("/static", routes![files])
        .attach(Template::fairing())
        .launch();
}
