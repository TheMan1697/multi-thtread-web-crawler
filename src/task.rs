// task.rs

use super::model::Pokemon;
use super::api::fetch_pokemon;

use std::collections::BTreeMap;

const MAX_POKEMON: u32 = 151;

pub fn create_fetch_tasks() -> Vec<tokio::task::JoinHandle<Option<Pokemon>>> {
    (1..=MAX_POKEMON).map(|id| {
        tokio::spawn(async move {
            fetch_pokemon(id).await.ok()
        })
    }).collect()
}

pub async fn await_tasks(tasks: Vec<tokio::task::JoinHandle<Option<Pokemon>>>) -> BTreeMap<u32, Pokemon> {
    let mut results = BTreeMap::new();
    for (id, task) in (1..).zip(tasks) {
        if let Ok(Some(pokemon)) = task.await {
            results.insert(id, pokemon);
        }
    }
    results
}