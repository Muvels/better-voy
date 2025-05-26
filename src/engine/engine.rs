use crate::Resource;
use kiddo::float::{distance::squared_euclidean, kdtree::KdTree};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, convert::TryInto};

use super::hash;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]

pub struct Document {
    pub id: String,
    pub title: String,
    pub url: String,
}

// New struct to include the similarity score
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SearchResult {
    pub document: Document,
    pub similarity_score: f32, // Using f32 for the score
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Query {
    // TODO: support query in string
    // Phrase(String)
    Embeddings(Vec<f32>),
}

// Wasm has a 4GB memory limit. Should make sure the bucket size and capacity
// doesn't exceed it and cause stack overflow.
// More detail: https://v8.dev/blog/4gb-wasm-memory
const BUCKET_SIZE: usize = 32;

pub type Tree = KdTree<f32, u64, 768, BUCKET_SIZE, u16>;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Index {
    // "IDX" is set to u16 to optimize CPU cache.
    // Read more: https://github.com/sdd/kiddo/blob/7a0bb6ecce39963b27ffdca913c6be7a265e3523/src/types.rs#L35
    pub tree: Tree,
    pub data: HashMap<u64, Document>,
}

pub fn index(resource: Resource) -> anyhow::Result<Index> {
    let data_vec: Vec<(u64, Document)> = resource
        .embeddings
        .iter()
        .map(|resource_item| Document {
            id: resource_item.id.to_owned(),
            title: resource_item.title.to_owned(),
            url: resource_item.url.to_owned(),
        })
        .map(|document| (hash(&document), document))
        .collect();

    let data: HashMap<u64, Document> = data_vec.clone().into_iter().collect();

    let mut tree: Tree = KdTree::new();

    resource
        .embeddings
        .iter()
        .zip(data_vec.iter())
        .for_each(|(resource_item, data_entry)| {
            let mut embeddings = resource_item.embeddings.clone();
            embeddings.resize(768, 0.0);

            let query_embedding: &[f32; 768] = &embeddings.try_into().unwrap_or_else(|_| panic!("Failed to convert embeddings to fixed size array during indexing for ID: {}", resource_item.id));
            tree.add(query_embedding, data_entry.0);
        });

    Ok(Index { tree, data })
}

// Modified search function to return Vec<SearchResult>
pub fn search<'a>(index: &'a Index, query: &'a Query, k: usize) -> anyhow::Result<Vec<SearchResult>> {
    let mut query_vec: Vec<f32> = match query {
        Query::Embeddings(q) => q.to_owned(),
    };
    query_vec.resize(768, 0.0);

    let query_embedding: &[f32; 768] = &query_vec.try_into().unwrap_or_else(|_| panic!("Failed to convert query embeddings to fixed size array during search."));
    // The `nearest_n` method returns a Vec of `NearestNeighbour` structs,
    // which include `distance` and `item`.
    let neighbors = index.tree.nearest_n(query_embedding, k, &squared_euclidean);

    let mut results: Vec<SearchResult> = vec![];

    for neighbor in &neighbors {
        let doc = index.data.get(&neighbor.item);
        if let Some(document) = doc {
            // The `neighbor.distance` is the squared Euclidean distance.
            let similarity_score = 1.0 / (1.0 + neighbor.distance); // Or any other transformation

            results.push(SearchResult {
                document: document.to_owned(),
                similarity_score,
            });
        }
    }

    Ok(results)
}

pub fn add<'a>(index: &'a mut Index, resource: &'a Resource) {
    for item in &resource.embeddings {
        let mut embeddings = item.embeddings.clone();
        embeddings.resize(768, 0.0);

        let query_embedding: &[f32; 768] = &embeddings.try_into().unwrap_or_else(|_| panic!("Failed to convert embeddings to fixed size array during add for ID: {}", item.id));
        let doc = Document {
            id: item.id.to_owned(),
            title: item.title.to_owned(),
            url: item.url.to_owned(),
        };
        let id_hash = hash(&doc);
        index.data.insert(id_hash, doc);
        index.tree.add(query_embedding, id_hash);
    }
}

pub fn remove<'a>(index: &'a mut Index, resource: &'a Resource) {
    for item in &resource.embeddings {
        let mut embeddings = item.embeddings.clone();
        embeddings.resize(768, 0.0);

        let query_embedding: &[f32; 768] = &embeddings.try_into().unwrap_or_else(|_| panic!("Failed to convert embeddings to fixed size array during remove for ID: {}", item.id));
        let id_hash = hash(&Document {
            id: item.id.to_owned(),
            title: item.title.to_owned(),
            url: item.url.to_owned(),
        });

        // Look into Note: Kiddo's remove might not be perfectly efficient or might have limitations
        // for removing by value if multiple items have the exact same embedding.
        // However, removing by (embedding, item_id) should be specific.
        // The current API `tree.remove(&[f32; K], item: B)` seems to require both.
        index.tree.remove(query_embedding, id_hash);
        index.data.remove(&id_hash);
    }
}

pub fn clear<'a>(index: &'a mut Index) {
    // simply assign a new tree and data because traversing the nodes to perform removal is the only alternative.
    // Kiddo provides only basic removal. See more: https://github.com/sdd/kiddo/issues/76
    index.tree = KdTree::new();
    index.data = HashMap::new();
}

pub fn size<'a>(index: &'a Index) -> usize {
    index.data.len()
}