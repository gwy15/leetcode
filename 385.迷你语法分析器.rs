/*
 * @lc app=leetcode.cn id=385 lang=rust
 *
 * [385] 迷你语法分析器
 */

#[derive(Debug, PartialEq, Eq)]
pub enum NestedInteger {
    Int(i32),
    List(Vec<NestedInteger>),
}

struct Solution;

// @lc code=start

type It<'a> = std::iter::Peekable<std::str::Chars<'a>>;

impl Solution {
    pub fn deserialize(s: String) -> NestedInteger {
        let mut chars = s.chars().peekable();
        Self::de(&mut chars).unwrap()
    }

    fn de(chars: &mut It) -> Option<NestedInteger> {
        let &first = chars.peek()?;

        if first == '[' {
            Self::de_list(chars)
        } else {
            Self::de_int(chars)
        }
    }

    fn de_list(chars: &mut It) -> Option<NestedInteger> {
        let first = chars.next()?;
        assert_eq!(first, '[');
        let mut list = Vec::new();
        loop {
            while let Some(&' ') = chars.peek() {
                chars.next();
            }
            if let Some(']') = chars.peek() {
                break;
            }
            let elem = Self::de(chars)?;
            list.push(elem);
            if chars.peek() == Some(&',') {
                chars.next();
            }
        }
        assert_eq!(chars.next()?, ']');
        Some(NestedInteger::List(list))
    }

    fn de_int(chars: &mut It) -> Option<NestedInteger> {
        let first = chars.next()?;
        if first == '-' {
            let elem = Self::de_int(chars)?;
            match elem {
                NestedInteger::Int(i) => return Some(NestedInteger::Int(-i)),
                _ => unreachable!(),
            }
        }
        let mut num = first.to_digit(10)?;
        while let Some(next) = chars.peek() {
            if next.is_digit(10) {
                num = num * 10 + next.to_digit(10)?;
                chars.next();
            } else {
                break;
            }
        }
        Some(NestedInteger::Int(num as i32))
    }
}
// @lc code=end
#[test]
fn test_solution() {
    use NestedInteger as N;

    macro_rules! test {
        ($args:tt, $ans:expr) => {
            assert_eq!(Solution::deserialize($args.to_string()), $ans)
        };
    }
    test!("324", N::Int(324));

    test!(
        "[123,[456,[789]]]",
        N::List(vec![
            N::Int(123),
            N::List(vec![N::Int(456), N::List(vec![N::Int(789)])]),
        ])
    );

    test!(
        "[123,[456,[-789, -10, 0, [-0]]]]",
        N::List(vec![
            N::Int(123),
            N::List(vec![
                N::Int(456),
                N::List(vec![
                    N::Int(-789),
                    N::Int(-10),
                    N::Int(0),
                    N::List(vec![N::Int(0)]),
                ]),
            ]),
        ])
    );

    test!(
        "[123,456,[788,799,833],[[]],10,[]]",
        N::List(vec![
            N::Int(123),
            N::Int(456),
            N::List(vec![N::Int(788), N::Int(799), N::Int(833),]),
            N::List(vec![N::List(vec![])]),
            N::Int(10),
            N::List(vec![]),
        ])
    )
}
