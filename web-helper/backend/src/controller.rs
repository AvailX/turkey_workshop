use std::str::FromStr;
use actix_web::{
    get, post,
    web::{Data, Json},
    HttpResponse, Responder,
};
use bson::{doc, Document};
use futures::TryStreamExt;
use mongodb::Client;
use rand::seq::SliceRandom;
use serde_json::json;
use snarkvm::prelude::PrivateKey;
use snarkvm_console_network::Testnet3;

use crate::{
    get_key_collection,
    goose::{create_goose, get_secret_code},
    models::{Key, Keys},
    use_key,
};

#[get("/")]
async fn get_code_and_pk(client: Data<Client>) -> impl Responder {
    let collection = get_key_collection::<Key>(client.get_ref());

    let filter = doc! { "used": false };

    let mut cursor = match collection.find(filter, None).await {
        Ok(cursor) => cursor,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    let mut keys = vec![];
    while let Some(key) = match cursor.try_next().await {
        Ok(Some(doc)) => Some(doc),
        Ok(None) => None,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    } {
        keys.push(key);
    }

    let mut rng = rand::thread_rng();

    let random_key = match keys.choose(&mut rng) {
        Some(key) => key,
        None => return HttpResponse::InternalServerError().finish(),
    };

    use_key(client.get_ref(), random_key).await;

    let private_key = match PrivateKey::<Testnet3>::from_str(&random_key.pk) {
        Ok(private_key) => private_key,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    let code = create_goose(get_secret_code(&private_key).unwrap().as_str()).unwrap();

    HttpResponse::Ok().json(json!({
        "pk": random_key.pk,
        "code": code,
    }))
}

#[post("/")]
async fn insert_code_and_pk(doc: Json<Keys>, client: Data<Client>) -> impl Responder {
    let docs = doc.into_inner();

    let new_docs = docs
        .keys
        .iter()
        .map(|key| doc! { "pk": &key, "used": false })
        .collect::<Vec<Document>>();

    let insert_result = match get_key_collection::<Document>(&client)
        .insert_many(new_docs, None)
        .await
    {
        Ok(result) => result,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    HttpResponse::Ok().json(insert_result)
}
