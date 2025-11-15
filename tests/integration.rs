use innocence_db::db::{DbBuilder, DbError};

#[test]
fn simple_gets_inserts_gets() {
    let db = DbBuilder::new().build_for_tests().unwrap();

    for i in 0i32..10_000 {
        assert!(matches!(
            db.get(&i.to_be_bytes().into()).unwrap_err(),
            DbError::KeyNotFound
        ));
        db.insert(i.to_be_bytes().into(), i.to_be_bytes().into())
            .unwrap();
        assert_eq!(db.get(&i.to_be_bytes().into()).unwrap(), &i.to_be_bytes());
    }
}
