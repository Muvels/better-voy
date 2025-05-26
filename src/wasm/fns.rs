use crate::{
    engine, utils::set_panic_hook, Neighbor, NumberOfResult, Query, Resource, SearchResult,
    SerializedIndex,
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn index(resource: Resource) -> SerializedIndex {
    set_panic_hook();

    let index = engine::index(resource);
    match index {
        Ok(idx) => serde_json::to_string(&idx).unwrap(),
        // Consider more robust error handling than returning an empty string
        _ => "".to_owned(),
    }
}

#[wasm_bindgen]
pub fn search(index: SerializedIndex, query: Query, k: NumberOfResult) -> SearchResult {
    set_panic_hook();

    let index_data: engine::Index = serde_json::from_str(&index).unwrap(); // Renamed to avoid shadowing module
    let engine_query: engine::Query = engine::Query::Embeddings(query); // Renamed to avoid shadowing type

    // Assuming engine::search now returns Vec<engine::SearchResultItem>
    // where engine::SearchResultItem { document: engine::Document, similarity_score: f32 }
    let core_search_results = engine::search(&index_data, &engine_query, k).unwrap();

    let neighbors: Vec<Neighbor> = core_search_results
        .into_iter()
        .map(|item_with_score| Neighbor { // item_with_score is an engine::SearchResultItem
            id: item_with_score.document.id,
            title: item_with_score.document.title,
            url: item_with_score.document.url,
            similarity_score: item_with_score.similarity_score, // Pass the score
        })
        .collect();

    SearchResult { neighbors }
}

#[wasm_bindgen]
pub fn add(index: SerializedIndex, resource: Resource) -> SerializedIndex {
    set_panic_hook();

    let mut index_data: engine::Index = serde_json::from_str(&index).unwrap(); // Renamed
    engine::add(&mut index_data, &resource);

    serde_json::to_string(&index_data).unwrap()
}

#[wasm_bindgen]
pub fn remove(index: SerializedIndex, resource: Resource) -> SerializedIndex {
    set_panic_hook();

    let mut index_data: engine::Index = serde_json::from_str(&index).unwrap(); // Renamed
    engine::remove(&mut index_data, &resource);

    serde_json::to_string(&index_data).unwrap()
}

#[wasm_bindgen]
pub fn clear(index: SerializedIndex) -> SerializedIndex {
    set_panic_hook();

    let mut index_data: engine::Index = serde_json::from_str(&index).unwrap(); // Renamed
    engine::clear(&mut index_data);

    serde_json::to_string(&index_data).unwrap()
}

#[wasm_bindgen]
pub fn size(index: SerializedIndex) -> usize {
    set_panic_hook();

    let index_data: engine::Index = serde_json::from_str(&index).unwrap(); // Renamed

    engine::size(&index_data)
}