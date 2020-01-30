struct Solution {}

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        // Boyer-Moore
        let mut votes = 0;
        let mut candidate = nums[0];
        for a in nums {
            if votes == 0 { // reset
                candidate = a;
                votes += 1
            } else if a == candidate {
                votes += 1;
            } else {
                votes -= 1;
            }
        }

        candidate
    }
}

fn main() {
    let nums = vec![2,2,1,1,1,2,2];
    println!("{}", Solution::majority_element(nums));
}
