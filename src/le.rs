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
    #[test]
    fn empty() {
        let empty = "".as_bytes();
        let stats = super::count_line_endings(empty);
        assert_eq!(stats, super::LineEndingStats { lf: vec![], crlf: vec![] })
    }
}
