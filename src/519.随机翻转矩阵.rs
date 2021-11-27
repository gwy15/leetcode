/*
 * @lc app=leetcode.cn id=519 lang=rust
 *
 * [519] 随机翻转矩阵
 */

// @lc code=start

use rand::prelude::*;
use std::collections::HashMap;

/// 思路：翻转了 i 个数字之后，直接对剩余的部分做一个 random.range。
/// 对于已经被选走的数字，做一个映射，映射到最后的几个数字上
///
struct Solution {
    m: i32,
    n: i32,
    picked_count: i32,
    map: HashMap<i32, i32>,
    #[cfg(debug_assertions)]
    rng: StdRng,
}

impl Solution {
    fn new(m: i32, n: i32) -> Self {
        Self {
            m,
            n,
            picked_count: 0,
            map: HashMap::default(),
            #[cfg(debug_assertions)]
            rng: StdRng::seed_from_u64(0),
        }
    }

    fn random(&mut self, high: i32) -> i32 {
        #[cfg(debug_assertions)]
        {
            let r = self.rng.gen_range(0, high);
            log::info!("high = {}, picked = {}", high, r);
            r
        }
        #[cfg(not(debug_assertions))]
        {
            thread_rng().gen_range(0, high)
        }
    }

    /// 一维
    fn _flip(&mut self) -> i32 {
        // [0, high)
        let high = self.m * self.n - self.picked_count;
        self.picked_count += 1;

        let mut pick = self.random(high);

        let ret = *self.map.get(&pick).unwrap_or(&pick);

        let mapped = *self.map.get(&(high - 1)).unwrap_or(&(high - 1));
        self.map.insert(pick, mapped);

        ret
    }

    fn flip(&mut self) -> Vec<i32> {
        let x = self._flip();
        return vec![x / self.n, x % self.n];
    }

    fn reset(&mut self) {
        self.picked_count = 0;
        self.map.clear();
    }
}
// @lc code=end

#[test]
fn test_solution_simple() {
    pretty_env_logger::try_init().ok();
    let mut sol = Solution::new(3, 1);
    assert_eq!(sol.flip(), vec![1, 0]);
    assert_eq!(sol.flip(), vec![0, 0]);
    assert_eq!(sol.flip(), vec![2, 0]);
    sol.reset();
    assert_eq!(sol.flip(), vec![0, 0]);
    assert_eq!(sol.flip(), vec![2, 0]);
    assert_eq!(sol.flip(), vec![1, 0]);

    let mut sol = Solution::new(2, 2);
    assert_eq!(sol.flip(), vec![1, 0]);
    assert_eq!(sol.flip(), vec![0, 0]);
    assert_eq!(sol.flip(), vec![1, 1]);
    assert_eq!(sol.flip(), vec![0, 1]);
}
