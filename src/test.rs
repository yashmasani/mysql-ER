#[cfg(test)]
mod tests {
    use lib::*;
    #[test]
    fn create_ports_from_nodes () {
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
    fn create_html_from_node_without_ports () {
        // pass
    }
    #[test]
    fn create_html_from_node_with_ports () {
        // pass
    }
    #[test]
    fn create_diagram_test () {
        //pass
    }

}
