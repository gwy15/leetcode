struct Solution;

use std::collections::HashMap;
#[allow(unused)]
impl Solution {
    fn find<'a>(x: &'a str, f: &mut HashMap<&'a str, &'a str>) -> &'a str {
        f.entry(x).or_insert(x);
        if x != f[x] {
            let root = Self::find(f[x], f);
            f.insert(x, root);
        }
        f[x]
    }

    pub fn truly_most_popular(names: Vec<String>, synonyms: Vec<String>) -> Vec<String> {
        let mut f: HashMap<&str, &str> = HashMap::new();
        for syn in synonyms.iter() {
            let split: Vec<&str> = syn[1..syn.len() - 1].split(',').collect();
            let (a, b) = (split[0], split[1]);
            let ra = Self::find(a, &mut f);
            let rb = Self::find(b, &mut f);
            if ra == rb {
                continue;
            } else if ra < rb {
                f.insert(rb, ra);
            } else {
                f.insert(ra, rb);
            }
        }
        // stat and sort
        let mut freq: HashMap<&str, i32> = HashMap::new();
        for info in names.iter() {
            let info: Vec<&str> = info[..info.len() - 1].split('(').collect();
            let (name, n): (_, i32) = (info[0], info[1].parse().unwrap());
            let name = Self::find(name, &mut f);
            *freq.entry(name).or_insert(0) += n;
        }
        //
        let mut names: Vec<String> = freq.keys().map(|&s| s.into()).collect();
        names.sort_by_key(|n| freq[&n[..]]);
        names
            .into_iter()
            .map(|name| format!("{}({})", name, freq[&name[..]]))
            .collect()
    }
}

#[test]
fn test_solution() {
    macro_rules! test {
        ($n:tt, $s:tt, $ans:tt) => {
            let f = |v: Vec<&str>| v.into_iter().map(|s| s.to_owned()).collect();
            assert_eq!(
                Solution::truly_most_popular(f(vec!$n), f(vec!$s)),
                vec!$ans
            );
        }
    };
    test!(
        [
            "John(15)",
            "Jon(12)",
            "Chris(13)",
            "Kris(4)",
            "Christopher(19)"
        ],
        [
            "(Jon,John)",
            "(John,Johnny)",
            "(Chris,Kris)",
            "(Chris,Christopher)"
        ],
        ["John(27)", "Chris(36)"]
    );
}
