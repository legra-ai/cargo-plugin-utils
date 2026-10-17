//! Recognize ConPTY's cursor-position query across arbitrary read boundaries.

#[derive(Default)]
pub(crate) struct CursorQuery {
    matched: usize,
}

impl CursorQuery {
    pub(crate) fn consume(&mut self, bytes: &[u8]) -> usize {
        const QUERY: &[u8] = b"\x1b[6n";
        let mut queries = 0;
        for &byte in bytes {
            if byte == QUERY[self.matched] {
                self.matched += 1;
                if self.matched == QUERY.len() {
                    queries += 1;
                    self.matched = 0;
                }
            } else {
                self.matched = usize::from(byte == QUERY[0]);
            }
        }
        queries
    }
}

#[cfg(test)]
mod tests {
    use super::CursorQuery;

    #[test]
    fn queries_survive_every_split_and_ignore_other_escape_sequences() {
        let bytes = b"noise\x1b[31m\x1b\x1b[6nremaining\x1b[6n";
        for split in 0..=bytes.len() {
            let mut query = CursorQuery::default();
            assert_eq!(
                query.consume(&bytes[..split]) + query.consume(&bytes[split..]),
                2
            );
        }
    }
}
