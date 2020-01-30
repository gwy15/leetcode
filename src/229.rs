struct Solution {}

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
        const K: usize = 2;

        #[derive(Debug)]
        struct Candidate {
            pub num: i32,
            pub votes: i32,
        }
        impl Clone for Candidate {
            fn clone(&self) -> Candidate {
                Candidate {
                    num: self.num,
                    votes: self.votes,
                }
            }
        }

        let mut candidates = vec![Candidate { num: 0, votes: 0 }; K];

        for num in &nums {
            let already_candidate = candidates.iter().any(|c| *num == c.num);
            if already_candidate {
                // up vote!
                for candidate in &mut candidates {
                    if *num == candidate.num {
                        candidate.votes += 1;
                        break;
                    }
                }
            } else {
                // replace or down vote
                let need_replace = candidates.iter().any(|c| 0 == c.votes);
                if need_replace {
                    for candidate in &mut candidates {
                        if candidate.votes == 0 {
                            candidate.num = *num;
                            candidate.votes = 1;
                            break;
                        }
                    }
                } else {
                    // not new one
                    for candidate in &mut candidates {
                        candidate.votes -= 1;
                    }
                }
            }
            println!("candidates: {:?}", candidates);
        }

        let threshold: usize = &nums.len() / 3;
        candidates
            .into_iter()
            .filter(|c| c.votes > 0)
            .filter(|c| nums.iter().filter(|&n| *n == c.num).count() > threshold)
            .map(|c| c.num)
            .collect()
    }
}

fn main() {
    let nums = vec![1, 1, 1, 3, 3, 2, 2, 2];
    println!("{:?}", Solution::majority_element(nums));

    let nums = vec![3, 2, 3];
    println!("{:?}", Solution::majority_element(nums));

    let nums = vec![0, 0, 0];
    println!("{:?}", Solution::majority_element(nums));
}
