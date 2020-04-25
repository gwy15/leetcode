struct Solution;

const K: i32 = 31;
const MOD: i32 = 131071;

impl Solution {
    fn b2i32(b: u8) -> i32 {
        (b - ('a' as u8)) as i32
    }
    pub fn longest_prefix(s: String) -> String {
        let n = s.len();

        let (mut left, mut right) = (0, 0);
        let mut base: i32 = 1;
        let mut best_length: usize = 0;

        let bytes: Vec<u8> = s.bytes().collect();

        for i in 0..n - 1 {
            left = (left * K + Self::b2i32(bytes[i])) % MOD;
            right = (Self::b2i32(bytes[n - 1 - i]) * base + right) % MOD;
            base = (base * K) % MOD;

            if left == right {
                best_length = i + 1;
            }
        }
        s[..best_length].into()
    }
}

#[test]
fn test_solution() {
    macro_rules! test {
        ($s:expr, $a:expr) => {
            assert_eq!(Solution::longest_prefix($s.into()), $a);
        };
    };
    test!("level", "l");
    test!("ababab", "abab");
    test!("aba", "a");
    test!("leetcodeleet", "leet");
    test!("a", "");
    test!(
        "okzuekdqhvytxqsjmikrdmcbuknwyoignupibvcgyzcdadqrawdunkljcrmdmgishurroijjybfjawogcnezzsofipkvjkktvjdgncjheimajtgrdnqoosxlrqwaoqwqdwipeetiparmvshuusmmzfqivvmiktspdtllrvknwsbeojvtrhdnufzjwrstobpzivsobwsmqijuzkqcpeutshutfeohcrvlyygrwrpxxboohqhxextzaazlmmbvzpsiainzyiwdqmrnliifbqhxsnyfufbrweuojsbaqwjhyiodfcxqfxjgrwyejtcjoyxxmfmjpawsjaxclwdmvlzclqfrpsrhubnoqsdujktputfvnohhlwcachepuyokzuekdqhvytxqsjmikrdmc",
        "okzuekdqhvytxqsjmikrdmc"
    );
}
