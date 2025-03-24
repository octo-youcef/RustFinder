use regex::Regex;

pub trait Searches {
    fn search<'a>(&'a self, contents: &'a str) -> Vec<String>;
}

pub struct Searcher<'a> {
    query: &'a str,
    case_insensitive: bool,
}

pub struct ReSearcher {
    pattern: Regex,
}

impl Searcher<'_> {
    pub fn new<'a>(query: &'a str, case_insensitive: bool) -> Searcher<'a> {
        Searcher {
            query: query,
            case_insensitive: case_insensitive,
        }
    }

    fn _sensitive_search<'a>(&'a self, contents: &'a str) -> Vec<String> {
        let mut results = Vec::new();
        let mut rownum = 1;

        for line in contents.lines() {
            if line.contains(self.query) {
                results.push(format!("{}: {}", rownum, line));
            }
            rownum += 1;
        }

        results
    }

    fn _insensitive_search<'a>(&'a self, contents: &'a str) -> Vec<String> {
        let mut results = Vec::new();
        let query = self.query.to_lowercase();
        let mut rownum = 1;

        for line in contents.lines() {
            if line.to_lowercase().contains(&query) {
                results.push(format!("{}: {}", rownum, line));
            }
            rownum += 1;
        }

        results
    }
}

impl ReSearcher {
    pub fn new(pattern: &str) -> ReSearcher {
        ReSearcher {
            pattern: Regex::new(pattern).unwrap(),
        }
    }
}

impl Searches for Searcher<'_> {
    fn search<'a>(&'a self, contents: &'a str) -> Vec<String> {
        if self.case_insensitive {
            self._insensitive_search(contents)
        } else {
            self._sensitive_search(contents)
        }
    }
}

impl Searches for ReSearcher {
    fn search<'a>(&'a self, contents: &'a str) -> Vec<String> {
        let mut results = Vec::new();
        let mut rownum = 1;

        for line in contents.lines() {
            if self.pattern.is_match(line) {
                results.push(format!("{}: {}", rownum, line));
            }
            rownum += 1;
        }

        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Error;
    static CONTENTS: &str = "line one\nLINE TWO";

    #[test]
    fn test_case_sensitive() -> Result<(), Error> {
        let searcher = Searcher::new("line", false);

        let observed_result = searcher.search(CONTENTS);
        let mut expected_result = Vec::new();

        expected_result.push("1: line one");

        assert_eq!(observed_result, expected_result);

        Ok(())
    }

    #[test]
    fn test_case_insensitive() -> Result<(), Error> {
        let searcher = Searcher::new("line", true);

        let observed_result = searcher.search(CONTENTS);
        let mut expected_result = Vec::new();

        expected_result.push("1: line one");
        expected_result.push("2: LINE TWO");

        assert_eq!(observed_result, expected_result);

        Ok(())
    }

    #[test]
    fn test_regex_match() -> Result<(), Error> {
        let re_searcher = ReSearcher::new("[a-z]+");

        let observed_result = re_searcher.search(CONTENTS);
        let mut expected_result = Vec::new();

        expected_result.push("1: line one");

        assert_eq!(observed_result, expected_result);

        Ok(())
    }
}
