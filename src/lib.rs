mod crdt;
mod util;

use std::sync::atomic::{Ordering};
use rand::Rng;

use sqlite_loadable::{api, define_scalar_function, prelude::*, Result as SQLiteResult};
use yrs::{self, ReadTxn, Transact};
use crate::crdt::*;
use crate::util::to_sqlite_error;


pub fn create_ydoc(context: *mut sqlite3_context, values: &[*mut sqlite3_value]) -> SQLiteResult<()> {
    let ver = api::value_int(values.get(0).ok_or("Version parameter is missing")?);
    let doc = create_doc_with_id();
    let tnx = doc.transact();
    let state = match ver {
        1 => tnx.encode_state_as_update_v1(&tnx.state_vector()),
        2 => tnx.encode_state_as_update_v2(&tnx.state_vector()),
        _ => return Err(sqlite_loadable::Error::new_message("Invalid version"))
    };

    api::result_blob(context, state.as_slice());
    Ok(())
}

pub fn ydoc_client_id(context: *mut sqlite3_context, values: &[*mut sqlite3_value]) -> SQLiteResult<()>  {
    let old_id = if values.len() >= 1 {
        let id_val = values.get(0).ok_or("ID must be passed as first parameter")?;
        ID.swap(api::value_int64(id_val) as u64, Ordering::SeqCst)
    } else {
        ID.load(Ordering::Relaxed)
    };

    api::result_int64(context, old_id as i64);
    Ok(())
}

pub fn ydoc_merge_update(context: *mut sqlite3_context, values: &[*mut sqlite3_value]) -> SQLiteResult<()>  {
    let ver = api::value_int(values.get(2).ok_or("Version parameter is missing")?);
    let ser = {
        let l_doc = values.get(0).ok_or("Target document parameter missing")?;
        let mut left = decode_update(
            api::value_blob(l_doc),
            ver
        ).map_err(to_sqlite_error)?;

        let r_doc = values.get(1).ok_or("Source document parameter missing")?;
        let right = decode_update(
            api::value_blob(r_doc),
            ver
        ).map_err(to_sqlite_error)?;

        left.merge(right);
        encode(&left, ver).map_err(to_sqlite_error)?
    };

    api::result_blob(context, ser.as_slice());
    Ok(())
}

#[sqlite_entrypoint]
pub fn sqlite3_sqliteycrdt_init(db: *mut sqlite3) -> SQLiteResult<()> {
    let mut rng = rand::thread_rng();
    ID.store(rng.gen(), Ordering::Relaxed);

    define_scalar_function(db, "ydoc", 1, create_ydoc, FunctionFlags::UTF8 | FunctionFlags::DETERMINISTIC)?;
    define_scalar_function(db, "ydoc_merge_update", 3, ydoc_merge_update, FunctionFlags::UTF8 | FunctionFlags::DETERMINISTIC)?;
    define_scalar_function(db, "ydoc_client_id", -1, ydoc_client_id, FunctionFlags::UTF8)?;
    Ok(())
}

#[cfg(test)]
mod test {
    use std::sync::atomic::Ordering;
    use crate::{create_doc_with_id, ID};

    #[test]
    pub fn test_doc_created_with_id() {
        ID.store(1337, Ordering::Relaxed);
        let doc = create_doc_with_id();
        assert_eq!(1337, doc.client_id());
    }
}