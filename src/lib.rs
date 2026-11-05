#[derive(Debug, Default)]
pub struct DuckDbEngine { queries: Vec<String> }
impl DuckDbEngine {
    pub fn new() -> Self { Self::default() }
    pub fn execute(&mut self, sql: impl Into<String>) { self.queries.push(sql.into()); }
    pub fn count(&self) -> usize { self.queries.len() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_query() {
        let mut e = DuckDbEngine::new();
        e.execute("SELECT 1;");
        assert_eq!(e.count(), 1);
    }
}
