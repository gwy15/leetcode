struct Solution;

//
use std::collections::HashMap;

struct CostInfo {
    pub inc: i64,
    pub dec: i64,
    pub jump: Vec<i32>,
    pub cost: Vec<i32>,
}

impl Solution {
    fn cost_to(target: i32, memo: &mut HashMap<i32, i64>, costinfo: &CostInfo) -> i64 {
        if target <= 0 {
            return 0;
        }
        if memo.contains_key(&target) {
            return memo[&target];
        }
        let mut cost: i64 = target as i64 * costinfo.inc;
        for (&times, &jump_cost) in costinfo.jump.iter().zip(costinfo.cost.iter()) {
            // 跳到 target 之前，再 inc
            cost = cost.min(
                Self::cost_to(target / times, memo, costinfo)
                    + jump_cost as i64
                    + (target % times) as i64 * costinfo.inc,
            );
            // 跳到 target 之后，再 dec
            if target > 1 && target % times > 0 {
                cost = cost.min(
                    Self::cost_to(target / times + 1, memo, costinfo)
                        + jump_cost as i64
                        + (times - target % times) as i64 * costinfo.dec,
                )
            }
        }
        memo.insert(target, cost);
        cost
    }

    #[allow(unused)]
    pub fn bus_rapid_transit(
        target: i32,
        inc: i32,
        dec: i32,
        jump: Vec<i32>,
        cost: Vec<i32>,
    ) -> i32 {
        let mut memo = HashMap::new();
        let costinfo = CostInfo {
            inc: inc as i64,
            dec: dec as i64,
            jump,
            cost,
        };
        (Self::cost_to(target, &mut memo, &costinfo) % 1_000_000_007) as i32
    }
}

#[test]
fn test_solution() {
    macro_rules! test {
        (target=$target:expr, inc=$inc:expr, dec=$dec:expr, jump=$jump:tt, cost=$cost:tt, $ans:expr) => {
            assert_eq!(
                Solution::bus_rapid_transit($target, $inc, $dec, vec!$jump, vec!$cost),
                $ans
            )
        };
    }
    test!(target = 31, inc = 5, dec = 3, jump = [6], cost = [10], 33);
    test!(
        target = 612,
        inc = 4,
        dec = 5,
        jump = [3, 6, 8, 11, 5, 10, 4],
        cost = [4, 7, 6, 3, 7, 6, 4],
        26
    );
    test!(
        target = 1_000_000_000,
        inc = 1,
        dec = 1,
        jump = [2],
        cost = [1000000],
        10953125
    );
    test!(
        target = 980632,
        inc = 2933,
        dec = 5626,
        jump = [6061, 5876, 6528, 6680, 5580, 2772, 6619, 7365, 9474, 2136],
        cost = [1792, 6103, 9708, 6519, 2305, 8327, 7393, 9533, 269, 7938],
        1964249
    );
    test!(
        target = 954116209,
        inc = 725988,
        dec = 636911,
        jump = [524425, 158389],
        cost = [41881, 941330],
        556489627
    );
}
