use regex::Regex;
use std::fmt::{self, Debug};
use std::ops::RangeInclusive;

/**
 * FileCov represents one line of a file coverage data in coverage report
 */
#[derive(Debug, PartialEq)]
pub struct FileCov {
    pub file: String,
    pub stmt_count: usize,
    pub miss_count: usize,
    pub covered_percent: usize,
    pub missed_sections: Vec<RangeInclusive<usize>>,
}

/**
 * Outputs in the `lcov` format for the file represented by FileCov record
 */
impl fmt::Display for FileCov {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "SF:{}", self.file)?;
        for section in &self.missed_sections {
            for lnum in section.to_owned() {
                writeln!(f, "DA:{},0", lnum)?;
            }
        }
        write!(f, "end_of_record")?;
        Ok(())
    }
}

impl FileCov {
    /**
     * parse a line from coverage data into an option of `FileCov` struct
     */
    pub fn parse(line: &str) -> Option<Self> {
        let parts: Vec<&str> = line.split('%').collect();
        let sections: Vec<RangeInclusive<usize>> =
            if parts.len() > 1 && !parts[1].trim().is_empty() {
                parts[1]
                    .split(',')
                    .into_iter()
                    .map(|s| {
                        let lnums: Vec<&str> = s.split('-').collect();
                        let lnum_start = lnums[0].trim().parse().unwrap();

                        lnum_start..=match lnums.len() > 1 {
                            true => lnums[1].trim().parse().unwrap(),
                            false => lnum_start,
                        }
                    })
                    .collect()
            } else {
                vec![]
            };
        let re = Regex::new(r"\s*(.+)\s+(\d+)\s+(\d+)\s+(\d+)\s*").unwrap();
        let cap = re.captures(parts[0])?;
        Some(FileCov {
            file: cap[1].trim().to_string(),
            stmt_count: cap[2].parse().unwrap(),
            miss_count: cap[3].parse().unwrap(),
            covered_percent: cap[4].parse().unwrap(),
            missed_sections: sections,
        })
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
            match FileCov::parse(t) {
                Some(_) => {
                    panic!("should get `None`")
                }
                None => {}
            }
        }
    }

    #[test]
    fn test_filecov_one_section() {
        let s = "ftplugin/lcov.vim                   30      8    73%   5-13";
        match FileCov::parse(s) {
            Some(r) => {
                assert_eq!(
                    r,
                    FileCov {
                        file: "ftplugin/lcov.vim".to_string(),
                        stmt_count: 30,
                        miss_count: 8,
                        covered_percent: 73,
                        missed_sections: vec![(5usize..=13usize)],
                    }
                )
            }
            None => {
                panic!("should not get `none`")
            }
        }
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
        let fc = FileCov::parse(s).unwrap();
        assert_eq!(format!("{}", fc), expected_output);
    }

    #[test]
    fn test_filecov_section_with_single_line() {
        let s = "autoload/lcov/signcolors.vim        45     11    76%   37-45, 60, 73";
        match FileCov::parse(s) {
            Some(r) => {
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
                )
            }
            None => {
                panic!("should not get `none`")
            }
        }
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
        let fc = FileCov::parse(s).unwrap();
        assert_eq!(format!("{}", fc), expected_output);
    }

    #[test]
    fn test_filecov_100percent_covered() {
        let s = "test/parser-tests.vim              101      0   100%";
        match FileCov::parse(s) {
            Some(r) => {
                assert_eq!(
                    r,
                    FileCov {
                        file: "test/parser-tests.vim".to_string(),
                        stmt_count: 101,
                        miss_count: 0,
                        covered_percent: 100,
                        missed_sections: vec![],
                    }
                )
            }
            None => {
                panic!("should not get `none`")
            }
        }
        let expected_output = r#"SF:test/parser-tests.vim
end_of_record"#;
        let fc = FileCov::parse(s).unwrap();
        assert_eq!(format!("{}", fc), expected_output);
    }
}
