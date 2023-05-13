
pub struct Searcher<'a> {
    query: &'a str,
    contents: &'a str
}

impl Searcher<'_> {
    pub fn new<'a>(query: &'a str, contents: &'a str) -> Searcher<'a> {
        Searcher {
            query: query,
            contents: contents
        }
    }

    fn _sensitive_search(&self) -> Vec<&str> {
        let mut results = Vec::new();

        for line in self.contents.lines() {
            if line.contains(self.query) {
                results.push(line);
            }
        }

        results
    }

    fn _insensitive_search(&self) -> Vec<&str> {
        let mut results = Vec::new();
        let query = self.query.to_lowercase();

        for line in self.contents.lines() {
            if line.to_lowercase().contains(&query) {
                results.push(line);
            }
        }

        results
    }

    pub fn search(&self, case_sensitive: bool) -> Vec<&str> {
        if case_sensitive {
            self._sensitive_search()
        } else {
            self._insensitive_search()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Error;

    #[test]
    fn test_case_sensitive() -> Result<(), Error> {
        let searcher = Searcher::new("line","line one\nLINE TWO");

        let observed_result = searcher.search(true);
        let mut expected_result = Vec::new();
        
        expected_result.push("line one");
        
        assert_eq!(observed_result, expected_result);

        Ok(())
    }
    
    #[test]
    fn test_case_insensitive() -> Result<(), Error> {
        let searcher = Searcher::new("line","line one\nLINE TWO");

        let observed_result = searcher.search(false);
        let mut expected_result = Vec::new();
        
        expected_result.push("line one");
        expected_result.push("LINE TWO");
        
        assert_eq!(observed_result, expected_result);

        Ok(())
    }
}

