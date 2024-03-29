mod controller;
mod goose;
mod models;

use actix_cors::Cors;
use actix_governor::{Governor, GovernorConfigBuilder};
use actix_web::http;
use actix_web::middleware::Logger;
use actix_web::{web::Data, App, HttpServer};
use anyhow::Result;
use models::Key;
use mongodb::bson::doc;
use mongodb::{
    options::{ClientOptions, ResolverConfig},
    Client,
};

use std::env;
use tracing::subscriber;
use tracing::{info, subscriber::set_global_default};

use crate::controller::{get_code_and_pk, insert_code_and_pk};

const DB_NAME: &str = "avail";
const COLLECTION_NAME: &str = "keys";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let client_uri = env::var("MONGODB_URI").unwrap();
    let port: u16 = env::var("PORT").unwrap().parse().unwrap();

    let subscriber = tracing_subscriber::fmt::Subscriber::builder()
        .with_max_level(tracing::Level::INFO)
        .finish();

    let _ = set_global_default(subscriber);

    let options =
        ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
            .await
            .unwrap();

    let governor_conf = GovernorConfigBuilder::default()
        .per_second(600)
        .burst_size(2)
        .finish()
        .unwrap();

    info!("Server listening on port {}", port);

    HttpServer::new(move || {
        let client = Client::with_options(options.clone()).unwrap();
        let cors = Cors::permissive();

        App::new()
            // .wrap(Governor::new(&governor_conf))
            .wrap(cors)
            .wrap(Logger::new("%a %{User-Agent}i"))
            .app_data(Data::new(client.clone()))
            .service(get_code_and_pk)
            .service(insert_code_and_pk)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}

fn get_key_collection<T>(client: &Client) -> mongodb::Collection<T> {
    client.database(DB_NAME).collection(COLLECTION_NAME)
}

async fn use_key(client: &Client, key: &Key) {
    let collection = get_key_collection::<Key>(client);

    let filter = doc! { "pk": &key.pk };
    let update = doc! { "$set": { "used": true } };

    collection.update_one(filter, update, None).await.unwrap();
}
