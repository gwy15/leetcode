struct Solution;

impl Solution {
    fn simple_match(n_pattern: usize, value: String) -> bool {
        let pattern = value.len() / n_pattern;
        let pattern = &value[..pattern];
        value == pattern.repeat(n_pattern)
    }
    fn n_match(n_a: usize, n_b: usize, pattern: String, value: String) -> bool {
        'loop_length: for l_a in 0..=value.len() / n_a {
            let l_b = (value.len() - n_a * l_a) / n_b;
            if value.len() != n_a * l_a + n_b * l_b {
                continue 'loop_length;
            }

            let (mut pat_a, mut pat_b) = (None, None);
            let mut i = 0;
            for pat in pattern.chars() {
                match pat {
                    'a' => {
                        if pat_a.is_none() {
                            pat_a = Some(&value[i..i + l_a]);
                        } else if &value[i..i + l_a] != pat_a.unwrap() {
                            continue 'loop_length;
                        }
                        i += l_a;
                    }
                    'b' => {
                        if pat_b.is_none() {
                            pat_b = Some(&value[i..i + l_b]);
                        } else if &value[i..i + l_b] != pat_b.unwrap() {
                            continue 'loop_length;
                        }
                        i += l_b;
                    }
                    _ => unreachable!(),
                }
            }
            // length must match
            return true;
        }
        false
    }
    pub fn pattern_matching(pattern: String, value: String) -> bool {
        // stats
        let (mut n_a, mut n_b) = (0, 0);
        for c in pattern.chars() {
            if c == 'a' {
                n_a += 1;
            } else {
                n_b += 1;
            }
        }
        match (n_a, n_b) {
            (0, 0) => value.is_empty(),
            (0, _) => {
                if value.len() % n_b != 0 {
                    return false;
                }
                Solution::simple_match(n_b, value)
            }
            (_, 0) => {
                if value.len() % n_a != 0 {
                    return false;
                }
                Solution::simple_match(n_a, value)
            }
            _ => {
                if value.is_empty() {
                    return false;
                }
                Solution::n_match(n_a, n_b, pattern, value)
            }
        }
    }
}

#[test]
fn test_solution() {
    macro_rules! test {
        ($pat:expr, $v:expr, $ans:expr) => {
            assert_eq!(Solution::pattern_matching($pat.into(), $v.into()), $ans);
        };
    };
    test!("abba", "dogcatcatdog", true);
    test!("abba", "dogcatcatfish", false);
    test!("aaaa", "dogcatcatdog", false);
    test!("abba", "dogdogdogdog", true);
    test!("aaa", "", true);
    test!("bbb", "", true);
    test!("ab", "", false);
    test!("", "", true);
    test!("", "aa", false);
    test!(
        "bbbbbbbbbbbbbbabbbbb",
        "ppppppppppppppjsftcleifftfthiehjiheyqkhjfkyfckbtwbelfcgihlrfkrwireflijkjyppppg",
        true
    );
}
