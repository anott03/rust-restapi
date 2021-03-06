use warp::{Filter, http};
use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;
use serde::{Serialize, Deserialize};

type Items = HashMap<String, i32>;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Item {
    name: String,
    quantity: i32
}

#[derive(Clone)]
struct Store {
  grocery_list: Arc<RwLock<Items>>
}

impl Store {
    fn new() -> Self {
        Store {
            grocery_list: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

fn json_body() -> impl Filter<Extract = (Item,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

async fn add_list_item(item: Item, store: Store) -> Result<impl warp::Reply, warp::Rejection> {
    store.grocery_list.write().insert(item.name, item.quantity);
    println!("{:?}", store.grocery_list);
    return Ok(warp::reply::with_status(
            "Added items to list",
            http::StatusCode::CREATED,
    ));
}

async fn get_list(store: Store) -> Result<impl warp::Reply, warp::Rejection> {
    let mut result = HashMap::new();
    let r = store.grocery_list.read();

    for (key, value) in r.iter() {
        result.insert(key, value);
    }

    return Ok(warp::reply::json(&result));
}

#[tokio::main]
async fn main() {
    let store = Store::new();
    let store_filter = warp::any().map(move || store.clone());

    let add_items = warp::post()
        .and(warp::path("v1"))
        .and(warp::path("groceries"))
        .and(warp::path::end())
        .and(json_body())
        .and(store_filter.clone())
        .and_then(add_list_item);

    let get_items = warp::post()
        .and(warp::path("v1"))
        .and(warp::path("groceries"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(get_list);

    let routes = add_items.or(get_items);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
