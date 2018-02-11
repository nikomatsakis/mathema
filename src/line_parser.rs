use ::std::io::{self, BufRead, BufReader, Read};

crate struct LineParser {
    lines: Box<dyn Iterator<Item = io::Result<String>>>,
    current_line: Option<String>,
    line_number: u64,
}

impl LineParser {
    crate fn new(reader: impl Read + 'static) -> io::Result<Self> {
        let buffered_reader = BufReader::new(reader);
        let mut lines = Box::new(buffered_reader.lines());
        let current_line = match lines.next() {
            Some(text) => Some(text?),
            None => None,
        };
        Ok(LineParser {
            lines: lines,
            current_line: current_line,
            line_number: 1,
        })
    }

    crate fn current_line(&self) -> &str {
        self.current_line.as_ref().map(|s| &s[..]).unwrap_or("")
    }

    crate fn current_line_is_blank(&self) -> bool {
        self.current_line().trim().is_empty()
    }

    crate fn eof(&self) -> bool {
        self.current_line.is_none()
    }

    crate fn line_number(&self) -> u64 {
        self.line_number
    }

    crate fn read_next_line(&mut self) -> io::Result<()> {
        if self.current_line.is_none() {
            return Ok(());
        }

        self.current_line = match self.lines.next() {
            Some(text) => Some(text?),
            None => None,
        };

        self.line_number += 1;

        Ok(())
    }
}
