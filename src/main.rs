use warp::{Filter, http};
use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;

type items = HashMap<String, i32>;

#[derive(Debug, Deserialize, Serialize, Clone)]
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

async fn add_list_item(item: Item, store: Store) -> Result<impl warp::Reply, warp:: Rejection> {
    store.grocery_list.write().insert(item.name, item.quantity);
    Ok(warp::reply::with_status(
            "Added items to list",
            http::StatusCode::CREATED,
    ))
}

#[tokio::main]
async fn main() {
    let hello = warp::path!("hello" / String).map(|name| format!("Hello, {}!", name));
    warp::serve(hello).run(([127, 0, 0, 1], 3030)).await;
}
