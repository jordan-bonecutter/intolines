struct IntoLines {
    str: String,
    head: usize,
}

impl IntoLines {
    fn next(&mut self) -> Option<&str> {
        if self.head >= self.str.len() {
            return None;
        }

        self.head += 1;
        let start = if self.head == 1 {0} else {self.head};
        while self.head < self.str.len() && self.str.as_bytes()[self.head] as char != '\n' {
            self.head += 1;
        }

        return unsafe {
             Some(std::str::from_utf8_unchecked(&self.str.as_bytes()[start..self.head]))
        }
    }
}

impl From<String> for IntoLines {
    fn from(value: String) -> Self {
        Self{str: value, head: 0}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let str = "hello
this is a string
it's so cool".to_owned();
        let mut lines: IntoLines = str.into();
        let mut idx = 0;
        loop {
            let line = match lines.next() {
                Some(l) => l,
                _ => break,
            };
            assert_eq!(idx < 3, true);
            let expected = match idx {
                0 => "hello",
                1 => "this is a string",
                2 => "it's so cool",
                _ => unreachable!(),
            };
            assert_eq!(expected, line);
            idx += 1;
        }
    }
}
