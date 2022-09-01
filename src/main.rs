use mysql::*;
use mysql::prelude::*;

#[derive(Debug)]
struct TableRow {
    field: String,
    col_type: String,
    null: String,
    key: String,
}

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let url = "mysql://root@localhost:3306";
    let opts = OptsBuilder::new().user(Some("root")).db_name(Some("my_guitar_shop"));
    let mut conn = Conn::new(opts)?;
    let cols:Vec<TableRow> = conn.query_map("SHOW COLUMNS from Orders", |row:Row| {
        TableRow {
            field: row[0].as_sql(false),
            col_type: row[1].as_sql(false),
            null: row[2].as_sql(false),
            key: row[3].as_sql(false),
        }
    })?;
    
    println!("{}", cols[0].field.trim_matches('\'').to_string());
    Ok(())
}
