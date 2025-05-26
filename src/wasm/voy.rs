use crate::utils::set_panic_hook;
use crate::{engine, Neighbor, NumberOfResult, Query, Resource, SearchResult, SerializedIndex};

use js_sys::{Function, Object, Reflect};
use wasm_bindgen::prelude::*;

pub struct Options {
    pub on_init: Option<Function>,
    pub on_index: Option<Function>,
    pub on_add: Option<Function>,
    pub on_remove: Option<Function>,
    pub on_search: Option<Function>,
    pub on_clear: Option<Function>,
    pub on_serialize: Option<Function>,
    pub on_deserialize: Option<Function>,
}

/// Convert a JS object (the `options` arg to the constructor / deserialize)
/// into our strongly‑typed `Options` struct. Any property that cannot be
/// down‑cast to `Function` is ignored.
fn reify(js_options: Option<Object>) -> Option<Options> {
    let get_fn = |name: &str, obj: &Object| -> Option<Function> {
        match Reflect::get(obj, &JsValue::from_str(name)) {
            Ok(cb) if cb.is_function() => cb.dyn_into::<Function>().ok(),
            _ => None,
        }
    };

    js_options.map(|obj| Options {
        on_init:        get_fn("onInit",        &obj),
        on_index:       get_fn("onIndex",       &obj),
        on_add:         get_fn("onAdd",         &obj),
        on_remove:      get_fn("onRemove",      &obj),
        on_search:      get_fn("onSearch",      &obj),
        on_clear:       get_fn("onClear",       &obj),
        on_serialize:   get_fn("onSerialize",   &obj),
        on_deserialize: get_fn("onDeserialize", &obj),
    })
}

#[wasm_bindgen]
pub struct Voy {
    index:   engine::Index,
    options: Option<Options>,
}

#[wasm_bindgen]
impl Voy {

    #[wasm_bindgen(constructor)]
    pub fn new(resource: Option<Resource>, options: Option<Object>) -> Voy {
        set_panic_hook();

        let resource = resource.unwrap_or(Resource { embeddings: vec![] });
        let index = engine::index(resource).expect("index build failed");
        let opts  = reify(options);

        if let Some(cb) = opts.as_ref().and_then(|o| o.on_init.as_ref()) {
            cb.call0(&JsValue::UNDEFINED).ok();
        }

        Voy { index, options: opts }
    }

    pub fn serialize(&self) -> SerializedIndex {
        if let Some(cb) = self.options.as_ref().and_then(|o| o.on_serialize.as_ref()) {
            cb.call0(&JsValue::UNDEFINED).ok();
        }
        serde_json::to_string(&self.index).unwrap()
    }

    #[wasm_bindgen(js_name = "deserialize")]
    pub fn deserialize_js(serialized_index: SerializedIndex, options: Option<Object>) -> Voy {
        let index: engine::Index = serde_json::from_str(&serialized_index).unwrap();
        let opts = reify(options);

        if let Some(cb) = opts.as_ref().and_then(|o| o.on_deserialize.as_ref()) {
            cb.call0(&JsValue::UNDEFINED).ok();
        }

        Voy { index, options: opts }
    }

    pub fn index(&mut self, resource: Resource) {
        self.index = engine::index(resource).unwrap();
        if let Some(cb) = self.options.as_ref().and_then(|o| o.on_index.as_ref()) {
            cb.call0(&JsValue::UNDEFINED).ok();
        }
    }

    pub fn add(&mut self, resource: Resource) {
        engine::add(&mut self.index, &resource);
        if let Some(cb) = self.options.as_ref().and_then(|o| o.on_add.as_ref()) {
            cb.call0(&JsValue::UNDEFINED).ok();
        }
    }

    pub fn remove(&mut self, resource: Resource) {
        engine::remove(&mut self.index, &resource);
        if let Some(cb) = self.options.as_ref().and_then(|o| o.on_remove.as_ref()) {
            cb.call0(&JsValue::UNDEFINED).ok();
        }
    }

    pub fn clear(&mut self) {
        engine::clear(&mut self.index);
        if let Some(cb) = self.options.as_ref().and_then(|o| o.on_clear.as_ref()) {
            cb.call0(&JsValue::UNDEFINED).ok();
        }
    }

    pub fn search(&self, query: Query, k: NumberOfResult) -> SearchResult {
        let q: engine::Query = engine::Query::Embeddings(query);
        let neighbors_raw = engine::search(&self.index, &q, k).unwrap();
        let neighbors: Vec<Neighbor> = neighbors_raw
            .into_iter()
            .map(|n| Neighbor { id: n.id, title: n.title, url: n.url })
            .collect();

        if let Some(cb) = self.options.as_ref().and_then(|o| o.on_search.as_ref()) {
            cb.call0(&JsValue::UNDEFINED).ok();
        }

        SearchResult { neighbors }
    }

    pub fn size(&self) -> usize {
        engine::size(&self.index)
    }
}
