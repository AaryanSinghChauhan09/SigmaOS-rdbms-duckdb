use sigmaos_rdbms_duckdb::{DuckDb, Table, Column, ColumnType};

#[test]
fn duckdb_integration_aaa() {
    let mut db = DuckDb::new("mydb", true);
    let t = Table { name: "users".into(), columns: vec![Column { name: "id".into(), column_type: ColumnType::Integer }] };
    db.create_table(t).unwrap();
    assert_eq!(db.table_count(), 1);
}
