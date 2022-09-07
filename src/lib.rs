#[derive(Debug)]
pub struct TableRow {
    pub field: String,
    pub col_type: String,
    pub null: String,
    pub key: String,
}

#[derive(Debug)]
pub struct Tables {
    pub parent: String,
    pub children: Vec<TableRow>,
}

#[derive(Debug,Clone)]
pub struct Port {
    pub parent: String,
    pub field: String,
    pub port_name: String,
}

pub fn create_diagram(tables: Vec<Tables>) -> String { 
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

pub fn create_ports (tables: &Vec<Tables>) -> Vec<(Port, Port)> {
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

pub fn create_table_row(table_row: &TableRow, port_name: &String ) -> String {
    let mut prt = port_name.clone();
    if prt.len() > 0 {
        prt = format!(r#" port="{}""#, prt);
    }
    let x = format!("<tr><td{}>{}:{}</td></tr>", prt, (*table_row).field.trim_matches('\''), (*table_row).col_type.trim_matches('\''));
    x
}
