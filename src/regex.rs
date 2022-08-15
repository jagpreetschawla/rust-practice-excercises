use std::collections::HashSet;

enum MatchCountStatus {
    Under,
    Over,
    MaxMatch,
    InRange,
}

impl MatchCountStatus {
    fn get_status(min: &u8, max: &u8, to_check: &u8) -> Self {
        if to_check < min {
            Self::Under
        } else if to_check > max {
            Self::Over
        } else if to_check == max {
            Self::MaxMatch
        } else {
            Self::InRange
        }
    }
}

#[derive(Debug)]
enum SubPattern {
    Set {
        one_of: HashSet<u8>,
        min_count: u8,
        max_count: u8,
    },
    Range {
        between: (u8, u8),
        min_count: u8,
        max_count: u8,
    },
}

impl SubPattern {
    fn matches(&self, c: u8) -> bool {
        match self {
            SubPattern::Set { one_of, .. } => one_of.contains(&c),
            SubPattern::Range { between, .. } => (between.0..=between.1).contains(&c),
        }
    }

    fn allows_zero_length(&self) -> bool {
        let min_allowed = match self {
            SubPattern::Set { min_count, .. } => min_count,
            SubPattern::Range { min_count, .. } => min_count,
        };
        *min_allowed == 0u8
    }

    fn check_count(&self, count: u8) -> MatchCountStatus {
        match self {
            SubPattern::Set {
                min_count,
                max_count,
                ..
            } => MatchCountStatus::get_status(min_count, max_count, &count),
            SubPattern::Range {
                min_count,
                max_count,
                ..
            } => MatchCountStatus::get_status(min_count, max_count, &count),
        }
    }
}

#[derive(Debug)]
enum StateStatus<'a> {
    PendingMinimumMatches(PatternState<'a>),
    MatchInRange(PatternState<'a>),
    ExhaustedMatchLimit(Option<PatternState<'a>>),
    MatchFailed,
}

#[derive(Debug)]
struct PatternState<'a> {
    pattern: &'a Pattern,
    subpattern_index: usize,
    matched_count: u8,
}

impl<'a> PatternState<'a> {
    fn root_state(pattern: &'a Pattern) -> Self {
        Self {
            pattern,
            subpattern_index: 0,
            matched_count: 0,
        }
    }

    fn next_sub_pattern(&self) -> Option<Self> {
        let next_index = self.subpattern_index + 1;
        if next_index == self.pattern.subpatterns.len() {
            None
        } else {
            Some(Self {
                pattern: self.pattern,
                subpattern_index: next_index,
                matched_count: 0,
            })
        }
    }

    fn remaining_can_be_zero_len(&self) -> bool {
        self.subpattern_index >= self.pattern.opt_suffix_start_idx
    }

    fn update(mut self, c: u8) -> StateStatus<'a> {
        let subpattern = &self.pattern.subpatterns[self.subpattern_index];
        if subpattern.matches(c) {
            self.matched_count += 1;
            match subpattern.check_count(self.matched_count) {
                MatchCountStatus::Under => StateStatus::PendingMinimumMatches(self),
                MatchCountStatus::MaxMatch => StateStatus::ExhaustedMatchLimit(self.next_sub_pattern()),
                MatchCountStatus::Over => panic!("code should never reach here, possible bug"),
                MatchCountStatus::InRange => StateStatus::MatchInRange(self),
            }
        } else {
            StateStatus::MatchFailed
        }
    }
}

#[derive(Debug)]
pub struct Pattern{
    subpatterns: Vec<SubPattern>,
    opt_suffix_start_idx: usize
}

impl Pattern {
    pub fn new(pattern: &str) -> Self {
        let mut subpatterns = Vec::new();
        let mut pattern = pattern.as_bytes();

        // extracted common code used within this function into an internal function for reuse.
        fn extract_counts(mut pattern: &[u8], num_start_pos: usize) -> (u8, u8, usize) {
            pattern = &pattern[num_start_pos..];
            let num_sep_pos = pattern
                .iter()
                .position(|x| *x == b',')
                .expect("expected comma separator for number range, invalid pattern");
            let brack_close_pos = pattern[(num_sep_pos + 1)..]
                .iter()
                .position(|x| *x == b'}')
                .expect("expected } at end, invalid pattern")
                + num_sep_pos
                + 1;
            let min_count = std::str::from_utf8(&pattern[..num_sep_pos])
                .unwrap()
                .parse()
                .expect("expected valid number");
            let max_count = std::str::from_utf8(&pattern[(num_sep_pos + 1)..brack_close_pos])
                .unwrap()
                .parse()
                .expect("expected valid number");
            (min_count, max_count, num_start_pos + brack_close_pos)
        }

        // convert input into a vec of PatternType by extracting 1 pattern at a time
        while pattern.len() > 0 {
            assert_eq!(pattern[0], b'[', "invalid pattern");
            pattern = if pattern[2] == b'-' {
                // range pattern
                assert!(pattern[4] == b']' && pattern[5] == b'{', "invalid pattern");
                let (min_count, max_count, end_pos) = extract_counts(pattern, 6);
                subpatterns.push(SubPattern::Range {
                    between: (pattern[1], pattern[3]),
                    min_count,
                    max_count,
                });
                &pattern[(end_pos + 1)..]
            } else {
                // set pattern
                assert!(pattern[2] == b',' || pattern[2] == b']', "invalid pattern");
                let mut char_set = HashSet::new();
                let mut p_iter = pattern.iter().enumerate();
                p_iter.next(); // ignore first [
                let num_start_pos = loop {
                    let (_, c) = p_iter.next().expect("pattern unexpectedly ended");
                    char_set.insert(*c);
                    let (ni, nc) = p_iter
                        .next()
                        .filter(|(_, x)| **x == b',' || **x == b']')
                        .expect("expected , or ] after char, invalid pattern");
                    if *nc == b']' {
                        break ni + 2;
                    }
                };
                let (min_count, max_count, end_pos) = extract_counts(pattern, num_start_pos);
                subpatterns.push(SubPattern::Set {
                    one_of: char_set,
                    min_count,
                    max_count,
                });
                &pattern[(end_pos + 1)..]
            }
        }
        let opt_suffix_count = subpatterns.iter().rev().take_while(|s| s.allows_zero_length()).count();
        let opt_suffix_start_idx = subpatterns.len() - opt_suffix_count;
        Pattern{ subpatterns, opt_suffix_start_idx }
    }

    pub fn find_matches<'s, 'i>(&'s self, inp: &'i str) -> Vec<&'i str> {
        let mut matches = Vec::new();

        // extracted common code used within this function into an internal function for reuse.
        fn check_status_n_update_next<'a>(
            s: PatternState<'a>,
            c: u8,
            next_to_check: &mut Vec<PatternState<'a>>,
        ) -> bool {
            let mut pattern_completed = false;
            let next_to_consider = match s.update(c) {
                StateStatus::PendingMinimumMatches(s) => {
                    next_to_check.push(s);
                    None
                },
                StateStatus::MatchInRange(s) => {
                    let nxt = s.next_sub_pattern();
                    next_to_check.push(s);
                    Some(nxt)
                },
                StateStatus::ExhaustedMatchLimit(next) => Some(next),
                StateStatus::MatchFailed => None,
            };
            
            if let Some(nxt) = next_to_consider {
                match nxt {
                    Some(ns) => {
                        if ns.remaining_can_be_zero_len() {
                            pattern_completed = true;
                            let mut n = Some(ns);
                            while let Some(i) = n {
                                n = i.next_sub_pattern();
                                next_to_check.push(i);
                            }
                        } else {
                            next_to_check.push(ns);
                        }
                    }
                    None => pattern_completed = true,
                }   
            }
            pattern_completed
        }

        let mut start_idx = 0;
        let inp_bytes = inp.as_bytes();
        while start_idx < inp_bytes.len() {
            let mut match_end_idx = None;
            let mut active_states: Vec<PatternState> =
                vec![PatternState::root_state(self)];
            for (i, c) in inp_bytes.iter().enumerate().skip(start_idx) {
                let mut new_states = Vec::new();
                for s in active_states.into_iter() {
                    if check_status_n_update_next(s, *c, &mut new_states) {
                        match_end_idx = Some(i);
                    }
                }
                active_states = new_states;
                if active_states.is_empty() {
                    break;
                }
            }
            if let Some(end_idx) = match_end_idx {
                matches.push(&inp[start_idx..=end_idx]);
                start_idx = end_idx;
            }
            start_idx += 1;
        }
        matches
    }
}

#[cfg(test)]
mod test {
    use super::Pattern;
    struct PatternTest {
        pattern: &'static str,
        inp: &'static str,
        expected: Vec<&'static str>,
    }

    impl PatternTest {
        fn test(self) {
            let pattern = Pattern::new(self.pattern);
            let out = pattern.find_matches(self.inp);
            assert!(
                is_both_equal(&out, &self.expected),
                "test failed. Got output: {:?}, expected: {:?}",
                &out,
                &self.expected
            );
        }
    }

    fn is_both_equal<T: PartialEq>(l1: &[T], l2: &[T]) -> bool {
        let mut i1 = l1.into_iter();
        let mut i2 = l2.into_iter();
        loop {
            let (c1, c2) = (i1.next(), i2.next());
            if c1 != c2 {
                break false;
            } else if c1.is_none() && c2.is_none() {
                break true;
            }
        }
    }

    #[test]
    fn single_list_pattern() {
        PatternTest {
            pattern: "[a,z,x,?]{3,4}",
            inp: "teststringazzx zxabla bluxaz?",
            expected: vec!["azzx", "zxa", "xaz?"],
        }
        .test();
    }

    #[test]
    fn single_list_pattern_duplicates() {
        PatternTest {
            pattern: "[a,z,x,?,a]{3,4}",
            inp: "teststringazzx zxabla bluxaz?",
            expected: vec!["azzx", "zxa", "xaz?"],
        }
        .test();
    }

    #[test]
    fn single_list_single_char() {
        PatternTest {
            pattern: "[a]{1,1}",
            inp: "teststringazzx zxabla bluxaz?",
            expected: vec!["a", "a", "a", "a"],
        }
        .test();
    }

    #[test]
    fn single_list_pattern_exactly_one() {
        PatternTest {
            pattern: "[1,2,?]{1,1}",
            inp: "12?11?2",
            expected: vec!["1", "2", "?", "1", "1", "?", "2"],
        }
        .test();
    }

    #[test]
    fn single_list_pattern_one_or_none() {
        PatternTest {
            pattern: "[1,2,?]{0,1}",
            inp: "12?11?2",
            expected: vec!["1", "2", "?", "1", "1", "?", "2"],
        }
        .test();
    }

    #[test]
    fn single_list_pattern_overlap() {
        PatternTest {
            pattern: "[1,2,?]{3,4}",
            inp: "12?11?2",
            expected: vec!["12?1", "1?2"],
        }
        .test();
    }

    #[test]
    fn single_range_pattern1() {
        PatternTest {
            pattern: "[a-z]{5,6}",
            inp: "test stringazzxzxablablBLABL STRING",
            expected: vec!["string", "azzxzx", "ablabl"],
        }
        .test();
    }

    #[test]
    fn single_range_pattern2() {
        PatternTest {
            pattern: "[A-Z]{5,6}",
            inp: "testSTRINGazzx zxa BLABL blabl string",
            expected: vec!["STRING", "BLABL"],
        }
        .test();
    }

    #[test]
    fn single_range_pattern3() {
        PatternTest {
            pattern: "[+-_]{5,6}",
            inp: "test string STRING TEST+ A-Z,/ 19^[] 10/343 testy",
            expected: vec!["STRING", "TEST+", "A-Z,/", "19^[]", "10/343"],
        }
        .test();
    }

    #[test]
    fn single_range_pattern_overlap() {
        PatternTest {
            pattern: "[a-z]{2,3}",
            inp: "acdab bcd",
            expected: vec!["acd", "ab", "bcd"],
        }
        .test();
    }

    #[test]
    fn single_range_pattern_exactly_one() {
        PatternTest {
            pattern: "[a-z]{1,1}",
            inp: "acdab",
            expected: vec!["a", "c", "d", "a", "b"],
        }
        .test();
    }

    #[test]
    fn single_range_pattern_one_or_none() {
        PatternTest {
            pattern: "[a-z]{0,1}",
            inp: "acdab",
            expected: vec!["a", "c", "d", "a", "b"],
        }
        .test();
    }

    #[test]
    fn multi_list_pattern1() {
        PatternTest {
            pattern: "[A,B,C]{2,4}[1,2,3]{1,2}",
            inp: "a1c12dabABClkjsdABC12fjBC1lsjflABCA2",
            expected: vec!["ABC12", "BC1", "ABCA2"],
        }
        .test();
    }

    #[test]
    fn multi_list_pattern2() {
        PatternTest {
            pattern: "[A,B,C]{2,3}[A,B,C,E]{1,3}",
            inp: "soidABCjfasdABCAnckjsABAA;oiwBCBEEjfoej",
            expected: vec!["ABC", "ABCA", "ABAA", "BCBEE"],
        }
        .test();
    }

    #[test]
    fn multi_range_pattern1() {
        PatternTest {
            pattern: "[A-Z]{2,3}[a-z]{1,2}",
            inp: "acdabAZAazlkjsdABCD12fjBCD1lsjABazflABCA2",
            expected: vec!["AZAaz", "ABaz"],
        }
        .test();
    }

    #[test]
    fn multi_range_pattern2() {
        PatternTest {
            pattern: "[A-Z]{2,3}[A-Z]{1,2}",
            inp: "iweAAuowABDZuofofABZoiAAAworrwZZZXXe",
            expected: vec!["ABDZ", "ABZ", "AAA", "ZZZXX"],
        }
        .test();
    }

    #[test]
    fn multi_mixed_pattern1() {
        PatternTest {
            pattern: "[A-Z]{3,4}[1,2,3]{1,1}[a-z]{2,2}",
            inp: "lkdjABF2flsjflAABBjsdflsAABjlf;jsADGGGdlfjslfsl",
            expected: vec!["ABF2fl"],
        }
        .test();
    }

    #[test]
    fn multi_mixed_pattern2() {
        PatternTest {
            pattern: "[1,2,3]{1,1}[a-z]{2,4}[D,E]{2,2}",
            inp: "kljsd2sssdEEldajl2ssdEEfjsdlf3ddwDDjsljf",
            expected: vec!["2sssdEE", "2ssdEE", "3ddwDD"],
        }
        .test();
    }

    #[test]
    fn multi_mixed_pattern_none_or_more_suffix1() {
        PatternTest {
            pattern: "[a]{1,1}[a-z]{0,5}",
            inp: "test aaa abcdef, aAaZa apple cat abcdefg lkjlkj",
            expected: vec!["aaa", "abcdef", "a", "a", "a", "apple", "at", "abcdef"],
        }
        .test();
    }

    #[test]
    fn multi_mixed_pattern_none_or_more_suffix2() {
        PatternTest {
            pattern: "[a]{1,1}[a-z]{0,5}[#]{0,1}",
            inp: "test aaa abcdef abcdef#, aAa#Za apple cat abcdefg lkjlkj",
            expected: vec!["aaa", "abcdef", "abcdef#", "a", "a#", "a", "apple", "at", "abcdef"],
        }
        .test();
    }

    #[test]
    fn multi_pattern_match_overlap() {
        PatternTest {
            pattern: "[a-d]{2,3}[d-e]{1,1}",
            inp: "abdde", // both abdd and bdde match. But we will select one which starts first on overlap
            expected: vec!["abdd"],
        }
        .test();
    }

    #[test]
    fn multi_no_match() {
        PatternTest {
            pattern: "[1,2,3]{1,1}[a-z]{2,3}[D,E]{2,2}",
            inp: "kljsdldajl2ssEfjsdlf3ddwDjsljf",
            expected: vec![],
        }
        .test();
    }
}
