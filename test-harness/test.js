const sqlite = require('better-sqlite3');
const Y = require('yjs');
const assert = require('assert');

describe('YCRDT functions v1', () => {
    let db = sqlite(':memory:', {});
    beforeEach(() => {
        db = sqlite(':memory:', {});
        db.exec("CREATE TABLE dstore(k string PRIMARY KEY , v BLOB)")
            .loadExtension("../target/debug/libsqlite_y_crdt");
    });

    function insert_doc(k, doc) {
        const data = Y.encodeStateAsUpdate(doc);
        return db.prepare("INSERT OR REPLACE INTO dstore(k, v) VALUES (?, ?) RETURNING k, v").get(k, data);
    }

    function merge_doc(k, doc) {
        const data = Y.encodeStateAsUpdate(doc);
        return db.prepare("UPDATE dstore SET v = ydoc_merge_update(v, ?, 1) WHERE k = ?").run(data, k);
    }

    function get_doc(k) {
        const {v} = db.prepare("SELECT v FROM dstore WHERE k = ?").get(k);
        const doc = new Y.Doc();
        Y.applyUpdate(doc, v);
        return doc;
    }

    it('should be able to merge with empty doc using v1', () => {
        const doc = new Y.Doc();
        doc.getMap('map').set('test', 'Lorem Ipsum');

        const row = db.prepare('SELECT ydoc_merge_update(ydoc(1), ?, 1) as yd').get(
            Y.encodeStateAsUpdate(doc)
        );

        const read_doc = new Y.Doc();
        Y.applyUpdate(read_doc, row["yd"]);
        assert(read_doc.getMap('map').get('test') === doc.getMap('map').get('test'));
    });

    it('should be able to merge with empty doc using v2', () => {
        const doc = new Y.Doc();
        doc.getMap('map').set('test', 'Lorem Ipsum');

        const row = db.prepare('SELECT ydoc_merge_update(ydoc(2), ?, 2) as yd').get(
            Y.encodeStateAsUpdateV2(doc)
        );

        const read_doc = new Y.Doc();
        Y.applyUpdateV2(read_doc, row["yd"]);
        assert(read_doc.getMap('map').get('test') === doc.getMap('map').get('test'));
    });

    it('Bad version fails gracefully for v2', () => {
        const doc = new Y.Doc();
        doc.getMap('map').set('test', 'Lorem Ipsum');

        try {
            db.prepare('SELECT ydoc_merge_update(ydoc(2), ?, 2) as yd').get(
                Y.encodeStateAsUpdate(doc)
            );
            assert(false);
        } catch (e) {
        }
    });

    it('should merge stored value with document updates', () => {
        let doc = new Y.Doc();
        doc.getMap('map').set('test1', 'A');
        insert_doc("key", doc);

        doc = new Y.Doc();
        doc.getMap('map').set('test2', 'B');
        merge_doc("key", doc);

        doc = get_doc("key");
        const dict = doc.getMap('map').toJSON();
        assert(dict.test1 === 'A');
        assert(dict.test2 === 'B');
    });
});



