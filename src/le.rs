pub enum LineEndingType {
    LF,
    CRLF,
    MIXED,
}

#[derive(Debug, PartialEq)]
pub struct LineEndingStats {
    pub lf: Vec<usize>,
    pub crlf: Vec<usize>,
}

impl LineEndingStats {
    pub fn is_lf(&self) -> bool {
        self.lf.len() > 0 && self.crlf.is_empty()
    }

    pub fn is_crlf(&self) -> bool {
        self.lf.is_empty() && self.crlf.len() > 0
    }

    pub fn is_mixed(&self) -> bool {
        self.lf.len() > 0 && self.crlf.len() > 0
    }
}

pub fn count_line_endings(bytes: &[u8]) -> LineEndingStats {
    const LINE_FEED: u8 = 0x0A;
    const CARRIAGE_RETURN: u8 = 0x0D;

    let mut stats = LineEndingStats {
        lf: vec![],
        crlf: vec![],
    };
    let mut prev: u8 = 0;
    let mut line_number: usize = 1;

    for byte in bytes.into_iter() {
        if *byte == LINE_FEED {
            if prev == CARRIAGE_RETURN {
                stats.crlf.push(line_number);
            } else {
                stats.lf.push(line_number);
            }
            line_number += 1;
        }

        prev = *byte;
    }

    stats
}

#[cfg(test)]
mod tests {
    use super::{count_line_endings, LineEndingStats};

    #[test]
    fn empty() {
        let empty = "".as_bytes();
        let stats: LineEndingStats = count_line_endings(empty);

        assert_eq!(
            stats,
            LineEndingStats {
                lf: vec![],
                crlf: vec![]
            }
        );
        assert!(!stats.is_lf());
        assert!(!stats.is_crlf());
        assert!(!stats.is_mixed());
    }

    #[test]
    fn single_lf() {
        let lf = "\n".as_bytes();
        let stats: LineEndingStats = count_line_endings(lf);

        assert_eq!(
            stats,
            LineEndingStats {
                lf: vec![1],
                crlf: vec![]
            }
        );
        assert!(stats.is_lf());
        assert!(!stats.is_crlf());
        assert!(!stats.is_mixed());
    }

    #[test]
    fn single_crlf() {
        let crlf = "\r\n".as_bytes();
        let stats: LineEndingStats = count_line_endings(crlf);

        assert_eq!(
            stats,
            LineEndingStats {
                lf: vec![],
                crlf: vec![1]
            }
        );
        assert!(!stats.is_lf());
        assert!(stats.is_crlf());
        assert!(!stats.is_mixed());
    }

    #[test]
    fn multiple_lf() {
        let multiple_lf = "\n\n\n\n".as_bytes();
        let stats: LineEndingStats = count_line_endings(multiple_lf);

        assert_eq!(
            stats,
            LineEndingStats {
                lf: vec![1, 2, 3, 4],
                crlf: vec![]
            }
        );
        assert!(stats.is_lf());
        assert!(!stats.is_crlf());
        assert!(!stats.is_mixed());
    }

    #[test]
    fn multiple_crlf() {
        let multiple_crlf = "\r\n\r\n\r\n\r\n".as_bytes();
        let stats: LineEndingStats = count_line_endings(multiple_crlf);

        assert_eq!(
            stats,
            LineEndingStats {
                lf: vec![],
                crlf: vec![1, 2, 3, 4]
            }
        );
        assert!(!stats.is_lf());
        assert!(stats.is_crlf());
        assert!(!stats.is_mixed());
    }

    #[test]
    fn mixed() {
        let mixed = "\n\r\n\n\r\n".as_bytes();
        let stats: LineEndingStats = count_line_endings(mixed);

        assert_eq!(
            stats,
            LineEndingStats {
                lf: vec![1, 3],
                crlf: vec![2, 4]
            }
        );
        assert!(!stats.is_lf());
        assert!(!stats.is_crlf());
        assert!(stats.is_mixed());
    }
}
