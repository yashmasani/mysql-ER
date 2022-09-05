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
        let mut port_name = String::new();
        table.children.iter().for_each(|child| {
            if child.key == "'MUL'" {
                let x = ports.iter().find(|p| { p.1.field == child.field.trim_matches('\'') });
                if x.is_some() {
                    port_name = (&x.unwrap().1.port_name).to_string();
                }
            } else if child.key == "'PRI'" {
                let x = ports.iter().find(|p| { p.0.field == child.field.trim_matches('\'') });
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
        let port_str = format!("{}:{} -> {}:{}\n", p.0.parent, p.0.port_name, p.1.parent, p.1.port_name);
        println!("{}", port_str);
        diag.push_str(&port_str);
    });
    format!(r#"strict graph t {{ 
            {}
            }}"#, diag)
}

fn create_ports (tables: &Vec<Tables>) -> Vec<(Port, Port)> {
    let mut ports:Vec<Port> = Vec::new();
    let mut port_pairs:Vec<(Port, Port)> = Vec::new();
    tables.iter().for_each( |t| {
        t.children.iter().for_each(|c| {
            let par = t.parent.clone().trim_matches('\'').to_string();
            let fld = c.field.clone().trim_matches('\'').to_string();
            if c.key == "'MUL'" {
                ports.push(Port {
                    parent: par,
                    field: fld,
                    port_name: format!("{}_MUL", c.field.trim_matches('\''))
                });
            } else if c.key == "'PRI'" {
                ports.push(Port {
                    parent: par,
                    field: fld,
                    port_name: format!("{}_PRI", c.field.trim_matches('\''))
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
    // println!("{:?}", port_pairs);
    port_pairs
}

fn create_table_row(table_row: &TableRow, port_name: &String ) -> String {
    let mut prt = port_name.clone();
    if prt.len() > 0 {
        prt = format!(r#" port="{}""#, prt);
    }
    return format!("<tr><td{}>{}:{}</td></tr>", prt, (*table_row).field.trim_matches('\''), (*table_row).col_type.trim_matches('\''));
}

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let url = "mysql://root@localhost:3306";
    let opts = OptsBuilder::new().user(Some("root")).db_name(Some("my_guitar_shop"));
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

    let mut g = parse(create_diagram(tot).as_str())?;
    
    let graph_svg = exec(g, &mut PrinterContext::default(), vec![
        CommandArg::Format(Format::Svg),
        CommandArg::Output("example.svg".to_string()),
    ]).unwrap();

    Ok(())
}
