use regex::Regex;
use std::convert::TryFrom;
use std::fmt::{self, Debug};
use std::ops::RangeInclusive;

/// `FileCov` represents one line of a file coverage data in coverage report
///
/// An example line in coverage.py output:
///
/// `"src/lib/config.rs          45     11    76%   37-45, 60, 73"`
///
/// is represented as:
///
/// ```
/// let fc = coverage2lcov::FileCov {
///     file: String::from("src/lib/config.rs"),
///     stmt_count: 45,
///     miss_count: 11,
///     covered_percent: 76,
///     missed_sections: vec![37..=45, 60..=60, 73..=73]
/// };
/// ```
#[derive(Debug, PartialEq, Eq)]
pub struct FileCov {
    /// name of the tested source file as in the coverage report
    pub file: String,

    /// number of statements in the file
    pub stmt_count: usize,

    /// number of statements missed in testing
    pub miss_count: usize,

    /// percentage of statements covered by tests
    pub covered_percent: u8,

    /// a vector of sections of lines missed (not covered).
    /// each section is an inclusive range of usize representing the start and
    /// end line numbers
    pub missed_sections: Vec<RangeInclusive<usize>>,
}

/// Outputs data represented by `FileCov` record in the `lcov` format
impl fmt::Display for FileCov {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "SF:{}", self.file)?;
        for section in &self.missed_sections {
            for lnum in section.clone() {
                writeln!(f, "DA:{},0", lnum)?;
            }
        }
        write!(f, "end_of_record")?;
        Ok(())
    }
}

impl TryFrom<&str> for FileCov {
    type Error = String;

    /// Convert a line from coverage data into an option of `FileCov` struct
    ///
    /// # Example
    ///
    /// ```
    /// use coverage2lcov::FileCov;
    ///
    /// let s = "src/lib/config.rs       35     23    34%   6, 12, 21-23, 30-54";
    ///
    /// let fc = FileCov::try_from(s).unwrap();
    ///
    /// assert_eq!(fc, FileCov{
    ///     file: String::from("src/lib/config.rs"),
    ///     stmt_count: 35,
    ///     miss_count: 23,
    ///     covered_percent: 34,
    ///     missed_sections: vec![6..=6, 12..=12, 21..=23, 30..=54]
    /// });
    /// ```
    fn try_from(line: &str) -> Result<Self, Self::Error> {
        let parts: Vec<&str> = line.split('%').collect();
        let sections: Vec<RangeInclusive<usize>> =
            if parts.len() > 1 && !parts[1].trim().is_empty() {
                parts[1]
                    .split(',')
                    .into_iter()
                    .map(|s| {
                        let lnums: Vec<&str> = s.split('-').collect();
                        let lnum_start = lnums[0].trim().parse().unwrap();

                        lnum_start..=if lnums.len() > 1 {
                            lnums[1].trim().parse().unwrap()
                        } else {
                            lnum_start
                        }
                    })
                    .collect()
            } else {
                vec![]
            };
        let re = Regex::new(r"\s*(.+)\s+(\d+)\s+(\d+)\s+(\d+)\s*").unwrap();
        if let Some(cap) = re.captures(parts[0]) {
            Ok(FileCov {
                file: cap[1].trim().to_string(),
                stmt_count: cap[2].parse().unwrap(),
                miss_count: cap[3].parse().unwrap(),
                covered_percent: cap[4].parse().unwrap(),
                missed_sections: sections,
            })
        } else {
            Err(format!(
                "Error parsing line: {:?} with regex: {:?}",
                line, re
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filecov_non_coverage_string() {
        let tests: Vec<&str> = vec![
            "",
            "%",
            "% 4, 10-12",
            "--------------------------------------------------------------",
            "Name                             Stmts   Miss  Cover   Missing",
        ];
        for t in tests {
            assert!(FileCov::try_from(t).is_err(), "should have got an Error");
            let result: Result<FileCov, String> = t.try_into();
            assert!(result.is_err(), "should have got an Error");
        }
    }

    #[test]
    fn test_filecov_one_section() {
        let s = "ftplugin/lcov.vim                   30      8    73%   5-13";
        let expected_output = r#"SF:ftplugin/lcov.vim
DA:5,0
DA:6,0
DA:7,0
DA:8,0
DA:9,0
DA:10,0
DA:11,0
DA:12,0
DA:13,0
end_of_record"#;
        match FileCov::try_from(s) {
            Ok(r) => {
                assert_eq!(
                    r,
                    FileCov {
                        file: "ftplugin/lcov.vim".to_string(),
                        stmt_count: 30,
                        miss_count: 8,
                        covered_percent: 73,
                        missed_sections: vec![(5usize..=13usize)],
                    }
                );
                assert_eq!(format!("{}", r), expected_output);
            }
            Err(e) => {
                panic!("Unexpected error: {:?}", e);
            }
        }
    }

    #[test]
    fn test_filecov_section_with_single_line() {
        let s = "autoload/lcov/signcolors.vim        45     11    76%   37-45, 60, 73";
        let expected_output = r#"SF:autoload/lcov/signcolors.vim
DA:37,0
DA:38,0
DA:39,0
DA:40,0
DA:41,0
DA:42,0
DA:43,0
DA:44,0
DA:45,0
DA:60,0
DA:73,0
end_of_record"#;
        match FileCov::try_from(s) {
            Ok(r) => {
                assert_eq!(
                    r,
                    FileCov {
                        file: "autoload/lcov/signcolors.vim".to_string(),
                        stmt_count: 45,
                        miss_count: 11,
                        covered_percent: 76,
                        missed_sections: vec![
                            (37usize..=45usize),
                            (60usize..=60usize),
                            (73usize..=73usize)
                        ],
                    }
                );
                assert_eq!(format!("{}", r), expected_output);
            }
            Err(e) => {
                panic!("Unexpected error: {:?}", e);
            }
        }
    }

    #[test]
    fn test_filecov_100percent_covered() {
        let s = "test/parser-tests.vim              101      0   100%";
        let expected_output = r#"SF:test/parser-tests.vim
end_of_record"#;
        match FileCov::try_from(s) {
            Ok(r) => {
                assert_eq!(
                    r,
                    FileCov {
                        file: "test/parser-tests.vim".to_string(),
                        stmt_count: 101,
                        miss_count: 0,
                        covered_percent: 100,
                        missed_sections: vec![],
                    }
                );
                assert_eq!(format!("{}", r), expected_output);
            }
            Err(e) => {
                panic!("Unexpected error: {:?}", e);
            }
        }
    }
}
