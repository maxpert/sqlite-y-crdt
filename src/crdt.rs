use std::sync::atomic::{AtomicU64, Ordering};
use lib0::error::Error;

use yrs::{self, Update, updates::decoder::Decode};
use yrs::updates::encoder::Encode;

pub static ID: AtomicU64 = AtomicU64::new(0);

pub fn create_doc_with_id() -> yrs::Doc {
    let id = ID.load(Ordering::Relaxed);
    yrs::Doc::with_client_id(id)
}


pub fn decode_update(blob: &[u8], version: i32) -> Result<Update, Error> {
    match version {
        1 =>  Update::decode_v1(blob),
        2 =>  Update::decode_v2(blob),
        _ => Err(Error::Other("Invalid version".to_string()))
    }
}

pub fn encode(upd: &impl Encode, version: i32) -> Result<Vec<u8>, Error> {
    match version {
        1 =>  Ok(upd.encode_v1()),
        2 =>  Ok(upd.encode_v2()),
        _ => Err(Error::Other("Invalid version".to_string()))
    }
}