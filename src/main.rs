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

fn create_diagram(tables: Vec<Tables>) -> String { 
    let diag = String::new();
    /* tables.iter().for_each(|x| {
         
    })*/
    let x = &tables[0];
    format!(r#"strict graph t {{
        aa[shape=square]
        bb[shape=plain label=<<table border="0" cellborder="1" cellspacing="0" cellpadding="10"><tr><td>{}</td></tr>{}</table>>]
    }}"#, x.parent, create_table_row(&x.children[0]))
}

fn create_table_row(table_row: &TableRow ) -> String {
    
    let x = (*table_row).field.clone();
    let y = (*table_row).col_type.clone();
    
    let row = format!("<tr><td>{} = {}</td></tr>", x.trim_matches('\''), y.trim_matches('\''));
    row 
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

    
    /* let mut g = graph!(id!("id");
        node!("nod"),
        subgraph!("sb";
            edge!(node_id!("a") => subgraph!(;
               node!("n1";
               NodeAttributes::color(color_name::black), NodeAttributes::shape(shape::box_))
           )),
            edge!(node_id!("b") => subgraph!(;
               node!("n2";
               NodeAttributes::color(color_name::black), NodeAttributes::shape(shape::box_))
           ))
       ),
       edge!(node_id!("a1") => node_id!(esc "a2"))
    );*/
    
    let mut g = parse(create_diagram(tot).as_str())?;

    let graph_svg = exec(g, &mut PrinterContext::default(), vec![
        CommandArg::Format(Format::Svg),
        CommandArg::Output("temp.svg".to_string()),
    ]).unwrap();

    Ok(())
}
