#![forbid(unsafe_code)]
/// Tableau workbook datasource and dashboard model for SigmaOS BI integration.
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct DataSource {
    pub name: String,
    pub connection_type: String,
    pub fields: Vec<String>,
}

impl DataSource {
    pub fn new(name: &str, conn_type: &str) -> Self {
        Self {
            name: name.into(),
            connection_type: conn_type.into(),
            fields: Vec::new(),
        }
    }

    pub fn add_field(&mut self, f: &str) {
        self.fields.push(f.into());
    }
    pub fn field_count(&self) -> usize {
        self.fields.len()
    }
    pub fn has_field(&self, f: &str) -> bool {
        self.fields.iter().any(|x| x == f)
    }
}

#[derive(Debug, Clone)]
pub struct Worksheet {
    pub name: String,
    pub datasource: String,
    pub rows_field: Option<String>,
    pub cols_field: Option<String>,
}

impl Worksheet {
    pub fn new(name: &str, datasource: &str) -> Self {
        Self {
            name: name.into(),
            datasource: datasource.into(),
            rows_field: None,
            cols_field: None,
        }
    }

    pub fn set_rows(&mut self, f: &str) {
        self.rows_field = Some(f.into());
    }
    pub fn set_cols(&mut self, f: &str) {
        self.cols_field = Some(f.into());
    }
    pub fn is_configured(&self) -> bool {
        self.rows_field.is_some() || self.cols_field.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_datasource_has_field_aaa() {
        let mut ds = DataSource::new("Orders", "postgresql");
        ds.add_field("order_date");
        assert!(ds.has_field("order_date"));
    }

    #[test]
    fn test_datasource_field_count_aaa() {
        let mut ds = DataSource::new("Sales", "csv");
        ds.add_field("region");
        ds.add_field("revenue");
        assert_eq!(ds.field_count(), 2);
    }

    #[test]
    fn test_worksheet_configured_after_rows_set_aaa() {
        let mut ws = Worksheet::new("Sales Trend", "Orders");
        ws.set_rows("SUM(Revenue)");
        assert!(ws.is_configured());
    }

    #[test]
    fn test_worksheet_not_configured_by_default_aaa() {
        let ws = Worksheet::new("Empty", "ds");
        assert!(!ws.is_configured());
    }
}
