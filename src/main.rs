use mysql::*;
use mysql::prelude::*;


fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let url = "mysql://root@localhost:3306";
    let opts = OptsBuilder::new().user(Some("root")).db_name(Some("my_guitar_shop"));
    let mut conn = Conn::new(opts)?;
    let mut orders = conn.query_iter("SHOW COLUMNS from orders").unwrap();
    for ords in orders.iter() {
        ords.for_each(|o| println!("{:?}", o.unwrap()) )
    }
    Ok(())
}
