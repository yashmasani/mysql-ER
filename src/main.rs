use std::env::*;
use mysql::*;
use mysql::prelude::*;
use graphviz_rust::{exec, parse};
use graphviz_rust::cmd::{CommandArg, Format};
use graphviz_rust::printer::{PrinterContext,DotPrinter};
use graphviz_rust::attributes::*;

use lib::*;
#[cfg(test)]
mod test;

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let args:Vec<String> = args().collect();
    let mut port = 3306;
    let mut user = "root";
    let mut db = "";
    let mut is_p = false;
    let mut is_user = false;
    let mut is_db = false;
    for arg in &args {
        if *arg == "-p".to_string() {
            is_p = true;
        } else if is_p {
            port = arg.parse()?;
            is_p = false;
        } else if *arg == "-u".to_string() {
            is_user = true;
        } else if is_user {
            user = arg;
            is_user = false;
        } else if *arg == "-db".to_string() {
            is_db = true;
        } else if is_db {
            db = arg;
            is_db = false;
        }
    }
    let opts = OptsBuilder::new().user(Some(user)).db_name(Some(db)).tcp_port(port);
    let mut conn = Conn::new(opts)?;
    let tables:Vec<Row>= conn.query("SHOW TABLES")?;
    let tot:Vec<_> = tables.iter().map(|r| {
        let table_name = &r[0];
        let x = (*table_name).as_sql(false).clone();
        let mut query:String = String::from("SHOW COLUMNS from ");
        query.push_str(x.trim_matches('\''));
        let table:Vec<TableRow> = conn.query_map(query, |table_row:Row| {
            TableRow {
                field: table_row[0].as_sql(false),
                col_type: table_row[1].as_sql(false),
                null: table_row[2].as_sql(false),
                key: table_row[3].as_sql(false),
            }
        }).unwrap();
        Tables {
            parent: x.trim_matches('\'').to_string(),
            children: table,
        }
    }).collect();

    let g = parse(create_diagram(tot).as_str())?;
    
    exec(g, &mut PrinterContext::default(), vec![
        CommandArg::Format(Format::Svg),
        CommandArg::Output("example.svg".to_string()),
    ]).unwrap();

    Ok(())
}
