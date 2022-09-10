#[cfg(test)]
mod tests {
    use lib::*;
    #[test]
    fn create_table_row_from_node_without_ports () {
        // pass
        let table_row = TableRow {
            field: "'my_id'".to_string(),
            col_type: "'int'".to_string(),
            null: "''".to_string(),
            key: "''".to_string(),
        };
        assert_eq!("<tr><td>my_id:int</td></tr>", create_table_row(&table_row, &"".to_string()));
        
    }
    #[test]
    fn create_table_row_from_node_with_ports () {
        // pass
        let table_row = TableRow {
            field: "'my_id'".to_string(),
            col_type: "'int'".to_string(),
            null: "''".to_string(),
            key: "'PRI'".to_string(),
        };
        assert_eq!(r#"<tr><td port="p0">my_id:int</td></tr>"#, create_table_row(&table_row, &"p0".to_string()));
    }
    #[test]
    fn create_ports_from_tables_no_ports () {
        // pass
        let fields = vec!["my_id", "order_id", "item", "customer_name"];
        let table_rows:Vec<TableRow> = fields.iter().map(|x| {
            return TableRow {
                field: x.to_string(),
                col_type: "'int'".to_string(),
                null: "''".to_string(),
                key: "''".to_string(),
            };
        }).collect();
        let table = Tables {
            parent: "field_table".to_string(),
            children: table_rows,
        };
        let tables = vec![table];
        assert_eq!(0, create_ports(&tables).len());
    }
    #[test]
    fn create_ports_from_table_with_ports () {
        // pass
        let fields = vec!["my_id", "order_id", "item", "customer_name"];
        let table_rows:Vec<TableRow> = fields.iter().map(|x| {
            let mut key = "'PRI'".to_string();
            if !x.ends_with("id") {
                key = "'MUL'".to_string();
            }
            return TableRow {
                field: x.to_string(),
                col_type: "'int'".to_string(),
                null: "''".to_string(),
                key,
            };
        }).collect();
        let table_one = Tables {
            parent: "field_one_table".to_string(),
            children: table_rows,
        };
        let table_two = Tables {
            parent: "field_two_table".to_string(),
            children: vec![
                TableRow {
                    field: "my_id".to_string(),
                    col_type: "'int'".to_string(),
                    null: "''".to_string(),
                    key: "'MUL'".to_string(),
                }
            ],
        };
        let tables = vec![table_one, table_two];
        let ports = vec!["p1_PRI".to_string(), "p5_MUL".to_string()];
        let test = create_ports(&tables);
        println!("{:?}", test);
        // test out length
        assert_eq!(1, test.len());
        // test each port relation
        test.iter().for_each(|t| {
            assert_eq!(ports[0], t.0.port_name);
            assert_eq!(ports[1], t.1.port_name);
        });
    }
    #[test]
    fn create_diagram_test () {
        //pass
    }

}
