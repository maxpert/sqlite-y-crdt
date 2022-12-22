# SQLite Y-CRDTs Extension

This repo is WIP for wrapping Y-CRDT for SQLite. 

## Why?

[Yjs](https://yjs.dev/) provides a great way to implement CRDTs. This repository is exploratory PoC of building blocks
that takes the implementation from [Y-CRDT](https://github.com/y-crdt/y-crdt), and exposes them for usage inside
SQLite as functions and operators (just like JSON). This will let developers build:

 - Inplace document creation, merging, and modification.
 - Identify useful set of operations required for comprehensive usage of Y-CRDTs.

## Usage

Build:

```bash
cargo build
```

Once built load and use in sqlite3 by:

```
sqlite3
.load target/debug/libsqlite_y_crdt.dylib 
select ydoc_merge_update(ydoc(1), ydoc(1), 1);
```

## Functions

 - `ydoc(version)`: creates an empty YDOC with and serializes it to given `version` as BLOB
 - `ydoc_merge_update(target, source, version)`: Decodes and merges updates from `source` into `target`. Encoding 
   and decoding version is passed as `version` parameter.
 - `ydoc_client_id([id])`: Everytime creating a document the passed `id` will be used as document ID. Calling this
   function without parameters returns current ID. It's initialized as random value when extension is loaded.