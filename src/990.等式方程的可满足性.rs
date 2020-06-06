/*
 * @lc app=leetcode.cn id=990 lang=rust
 *
 * [990] 等式方程的可满足性
 */
struct Solution;
// @lc code=start
#[allow(unused)]
impl Solution {
    fn find(x: usize, f: &mut [usize]) -> usize {
        if f[x] != x {
            f[x] = Self::find(f[x], f);
        }
        f[x]
    }

    pub fn equations_possible(equations: Vec<String>) -> bool {
        let mut f: Vec<usize> = (0..26).collect();

        let u8_to_usize = |u: u8| u as usize - ('a' as usize);

        for s in equations.iter() {
            if &s[1..3] == "==" {
                let chars = s.as_bytes();
                let (a, b) = (chars[0], chars[3]);
                let ra = Self::find(u8_to_usize(a), &mut f);
                let rb = Self::find(u8_to_usize(b), &mut f);
                // merge
                if ra != rb {
                    f[ra] = rb;
                }
            }
        }
        // eprintln!("eq: {:?}, f:{:?}", equations, f);
        equations
            .into_iter()
            .filter(|eq| &eq[1..3] == "!=")
            .all(|eq| {
                let eq = eq.as_bytes();
                let (a, b) = (eq[0], eq[3]);
                let ra = Self::find(u8_to_usize(a), &mut f);
                let rb = Self::find(u8_to_usize(b), &mut f);
                ra != rb
            })
    }
}
// @lc code=end
#[test]
fn test_solution() {
    macro_rules! test {
        ($v:tt, $ans:expr) => {
            assert_eq!(
                Solution::equations_possible(vec!$v.into_iter().map(|s| s.into()).collect()),
                $ans
            );
        }
    };
    test!(["a==b", "b!=a"], false);
    test!(["b==a", "a==b"], true);
    test!(["a==b", "b==c", "a==c"], true);
    test!(["a==b", "b!=c", "c==a"], false);
    test!(["c==c", "b==d", "x!=z"], true);
}
