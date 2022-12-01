use std::str::Lines;

/// Many Advent of Code inputs are formatted as
///
/// a
/// b
///
/// c
///
/// d
/// e
///
/// where each group of lines separated from each other by only a single '\n'
/// form a logical group.  Each group is separated from the next group
/// by a single empty line.
pub fn grouped_lines(s: &str) -> GroupedLines {
    GroupedLines { lines: s.lines() }
}

pub struct GroupedLines<'a> {
    lines: Lines<'a>,
}

impl<'a> Iterator for GroupedLines<'a> {
    type Item = Vec<&'a str>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.lines.next() {
            Some(line) if !line.is_empty() => {
                let mut lines = vec![line];
                loop {
                    match self.lines.next() {
                        Some(line) if !line.is_empty() => {
                            lines.push(line);
                        }
                        _ => break,
                    }
                }
                Some(lines)
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multiple_groups() {
        let s = "foo\nbar\nbaz\n\nfoo\nbar";
        let groups: Vec<_> = grouped_lines(s).collect();
        assert_eq!(groups, vec![vec!["foo", "bar", "baz"], vec!["foo", "bar"]]);
    }

    #[test]
    fn multiple_groups_trailing_newline() {
        let s = "foo\nbar\nbaz\n\nfoo\nbar\n";
        let groups: Vec<_> = grouped_lines(s).collect();
        assert_eq!(groups, vec![vec!["foo", "bar", "baz"], vec!["foo", "bar"]]);
    }

    #[test]
    fn single_group() {
        let s = "foo\nbar";
        let groups: Vec<_> = grouped_lines(s).collect();
        assert_eq!(groups, vec![vec!["foo", "bar"]]);
    }

    #[test]
    fn empty() {
        let mut groups = grouped_lines("");
        assert_eq!(None, groups.next());
    }

    #[test]
    fn just_newline() {
        let mut groups = grouped_lines("\n");
        assert_eq!(None, groups.next());
    }
}