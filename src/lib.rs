use regex::Regex;
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct FileRecord {
    pub file: String,
    pub stmt_count: u32,
    pub miss_count: u32,
    pub covered_percent: u32,
    pub uncovered_sections: Vec<(u32, u32)>,
}

impl fmt::Display for FileRecord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "SF:{}", self.file)?;
        for section in &self.uncovered_sections {
            for lnum in section.0..=section.1 {
                writeln!(f, "DA:{},0", lnum)?;
            }
        }
        write!(f, "end_of_record")?;
        Ok(())
    }
}

impl FileRecord {
    #[allow(dead_code)]
    pub fn from_string(line: String) -> Option<FileRecord> {
        let parts: Vec<&str> = line.split('%').collect();
        let sections: Vec<(u32, u32)> =
            if parts.len() > 1 && !parts[1].trim().is_empty() {
                parts[1]
                    .split(',')
                    .into_iter()
                    .map(|s| {
                        let lnums: Vec<&str> = s.split('-').collect();
                        let lnum_start = lnums[0].trim().parse().unwrap();
                        (
                            lnum_start,
                            match lnums.len() > 1 {
                                true => lnums[1].trim().parse().unwrap(),
                                false => lnum_start,
                            },
                        )
                    })
                    .collect()
            } else {
                vec![]
            };
        let re = Regex::new(r"\s*(.+)\s+(\d+)\s+(\d+)\s+(\d+)\s*").unwrap();
        let cap = re.captures(parts[0])?;
        Some(FileRecord {
            file: cap[1].trim().to_string(),
            stmt_count: cap[2].parse().unwrap(),
            miss_count: cap[3].parse().unwrap(),
            covered_percent: cap[4].parse().unwrap(),
            uncovered_sections: sections,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filerecord_from_string_non_coverage_string() {
        let tests : Vec<(&str, Option<FileRecord>)> = vec![
            ("", None),
            ("%", None),
            ("% 4, 10-12", None),
            ("--------------------------------------------------------------", None),
            ("Name                             Stmts   Miss  Cover   Missing", None),
            // ("TOTAL                              817    172    79%", None),
        ];
        for t in tests {
            match FileRecord::from_string(t.0.to_string()) {
                Some(_) => {
                    panic!("should get `none`")
                }
                None => {}
            }
        }
    }

    #[test]
    fn test_filerecord_from_string_one_section() {
        let s = "ftplugin/lcov.vim                   30      8    73%   5-13";
        match FileRecord::from_string(s.to_string()) {
            Some(r) => {
                assert_eq!(
                    r,
                    FileRecord {
                        file: "ftplugin/lcov.vim".to_string(),
                        stmt_count: 30,
                        miss_count: 8,
                        covered_percent: 73,
                        uncovered_sections: vec![(5, 13)],
                    }
                )
            }
            None => {
                panic!("should not get `none`")
            }
        }
    }

    #[test]
    fn test_filerecord_from_string_section_with_single_line() {
        let s = "autoload/lcov/signcolors.vim        45     11    76%   37-45, 60, 73";
        match FileRecord::from_string(s.to_string()) {
            Some(r) => {
                assert_eq!(
                    r,
                    FileRecord {
                        file: "autoload/lcov/signcolors.vim".to_string(),
                        stmt_count: 45,
                        miss_count: 11,
                        covered_percent: 76,
                        uncovered_sections: vec![(37, 45), (60, 60), (73, 73)],
                    }
                )
            }
            None => {
                panic!("should not get `none`")
            }
        }
    }

    #[test]
    fn test_filerecord_from_string_with_no_uncovered() {
        let s = "test/parser-tests.vim              101      0   100%";
        match FileRecord::from_string(s.to_string()) {
            Some(r) => {
                assert_eq!(
                    r,
                    FileRecord {
                        file: "test/parser-tests.vim".to_string(),
                        stmt_count: 101,
                        miss_count: 0,
                        covered_percent: 100,
                        uncovered_sections: vec![],
                    }
                )
            }
            None => {
                panic!("should not get `none`")
            }
        }
    }

    #[test]
    fn test_filerecord_from_string_edge_cases() {
        let s = "test/parser-tests.vim              101      0   100%";
        match FileRecord::from_string(s.to_string()) {
            Some(r) => {
                assert_eq!(
                    r,
                    FileRecord {
                        file: "test/parser-tests.vim".to_string(),
                        stmt_count: 101,
                        miss_count: 0,
                        covered_percent: 100,
                        uncovered_sections: vec![],
                    }
                )
            }
            None => {
                panic!("should not get `none`")
            }
        }
    }
}
