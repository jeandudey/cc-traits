#[cfg(feature = "heapless")]
mod heapless;

#[cfg(feature = "ijson")]
mod ijson;

#[cfg(feature = "serde_json")]
mod serde_json;

#[cfg(feature = "slab")]
mod slab;

#[cfg(feature = "smallvec")]
mod smallvec;

#[cfg(not(feature = "nostd"))]
mod std_collections;

