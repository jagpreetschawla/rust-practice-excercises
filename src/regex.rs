pub struct Pattern {}

impl Pattern {
    pub fn new(pattern: &str) -> Self {
        todo!("add string to pattern constructor here")
    }

    pub fn find_matches<'s, 'i>(&'s self, inp: &'i str) -> Vec<&'i str> {
        todo!("implement this")
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
