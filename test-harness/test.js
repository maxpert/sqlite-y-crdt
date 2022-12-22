const sqlite = require('better-sqlite3')
const Y = require('yjs');
const assert = require('assert');

describe('YCRDT functions v1', () => {
    let db = sqlite(':memory:', {});
    beforeEach(() => {
        db = sqlite(':memory:', {});
        db.loadExtension("../target/debug/libsqlite_y_crdt");
    });

    it('should be able to merge with empty doc', () => {
        const doc = new Y.Doc();
        doc.getMap('map').set('test', 'Lorem Ipsum');

        const row = db.prepare('SELECT ydoc_merge_update(ydoc(1), ?, 1) as yd').get(
            Y.encodeStateAsUpdate(doc)
        );

        const read_doc = new Y.Doc();
        Y.applyUpdate(read_doc, row["yd"]);
        assert(read_doc.getMap('map').get('test') === doc.getMap('map').get('test'));
    });
});

describe('YCRDT functions v2', () => {
    let db = sqlite(':memory:', {});
    beforeEach(() => {
        db = sqlite(':memory:', {});
        db.loadExtension("../target/debug/libsqlite_y_crdt");
    });

    it('should be able to merge with empty doc', () => {
        const doc = new Y.Doc();
        doc.getMap('map').set('test', 'Lorem Ipsum');

        const row = db.prepare('SELECT ydoc_merge_update(ydoc(2), ?, 2) as yd').get(
            Y.encodeStateAsUpdateV2(doc)
        );

        const read_doc = new Y.Doc();
        Y.applyUpdateV2(read_doc, row["yd"]);
        assert(read_doc.getMap('map').get('test') === doc.getMap('map').get('test'));
    });
});



