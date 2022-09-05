use std::env::*;
use mysql::*;
use mysql::prelude::*;
// use dot_generator::*;
use graphviz_rust::{exec, parse};
use graphviz_rust::cmd::{CommandArg, Format};
use graphviz_rust::printer::{PrinterContext,DotPrinter};
use graphviz_rust::attributes::*;

#[derive(Debug)]
struct TableRow {
    field: String,
    col_type: String,
    null: String,
    key: String,
}

#[derive(Debug)]
struct Tables {
    parent: String,
    children: Vec<TableRow>,
}

#[derive(Debug,Clone)]
struct Port {
    parent: String,
    field: String,
    port_name: String,
}

fn create_diagram(tables: Vec<Tables>) -> String { 
    let mut diag = String::new();
    let ports = create_ports(&tables);
    tables.iter().for_each(|table| {
        let mut table_children = String::new();
        table.children.iter().for_each(|child| {
            let mut port_name = String::new();
            if child.key == "'MUL'" {
                let x = ports.iter().find(|p| { p.1.parent == table.parent && p.1.field == child.field.trim_matches('\'') });
                if x.is_some() {
                    port_name = (&x.unwrap().1.port_name).to_string();
                }
            } else if child.key == "'PRI'" {
                let x = ports.iter().find(|p| { p.0.parent == table.parent && p.0.field == child.field.trim_matches('\'') });
                if x.is_some() {
                    port_name = (&x.unwrap().0.port_name).to_string();
                }
            }
            table_children.push_str(&create_table_row(&child, &port_name));
        });
        let table_str = format!(r#"{}[shape=plain label=<<table border="0" cellborder="1" cellspacing="0" cellpadding="10"><tr><td><b>{}</b></td></tr>{}</table>>]"#, table.parent, table.parent, table_children);
        diag.push_str(&table_str);
        diag.push('\n');
    });
    ports.iter().for_each(|p| {
        let port_str = format!("{}:{} -> {}:{}[dir=forward]\n", p.0.parent, p.0.port_name, p.1.parent, p.1.port_name);
        diag.push_str(&port_str);
    });
    format!(r#"strict graph t {{
            rankdir=LR
            {}
            }}"#, diag)
}

fn create_ports (tables: &Vec<Tables>) -> Vec<(Port, Port)> {
    let mut ports:Vec<Port> = Vec::new();
    let mut port_pairs:Vec<(Port, Port)> = Vec::new();
    let mut count = 0;
    tables.iter().for_each( |t| {
        t.children.iter().for_each(|c| {
            count += 1;
            let par = t.parent.clone().trim_matches('\'').to_string();
            let fld = c.field.clone().trim_matches('\'').to_string();
            if c.key == "'MUL'" {
                ports.push(Port {
                    parent: par,
                    field: fld,
                    port_name: format!("p{}_MUL", count)
                });
            } else if c.key == "'PRI'" {
                ports.push(Port {
                    parent: par,
                    field: fld,
                    port_name: format!("p{}_PRI", count)
                });
            }
        })
    });
    ports.iter().for_each(|port| {
        if port.port_name.ends_with("PRI") {
            ports.iter().for_each(|p| {
                if p.field == port.field && p.port_name.ends_with("MUL") {
                    port_pairs.push((port.clone(), p.clone()));
                };
            });
        }
    });
    port_pairs
}

fn create_table_row(table_row: &TableRow, port_name: &String ) -> String {
    let mut prt = port_name.clone();
    if prt.len() > 0 {
        prt = format!(r#" port="{}""#, prt);
    }
    let x = format!("<tr><td{}>{}:{}</td></tr>", prt, (*table_row).field.trim_matches('\''), (*table_row).col_type.trim_matches('\''));
    x
}

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
