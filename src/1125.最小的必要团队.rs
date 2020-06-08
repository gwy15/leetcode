/*
 * @lc app=leetcode.cn id=1125 lang=rust
 *
 * [1125] 最小的必要团队
 */
struct Solution;
// @lc code=start
use std::collections::HashMap;
type Skills = usize;
type Team = u64;
impl Solution {
    #[inline]
    fn skill_names_to_skills(skill_names: Vec<String>, mapper: &HashMap<String, Skills>) -> Skills {
        let mut person_skills = 0;
        for skill_name in skill_names {
            person_skills |= 1 << mapper[&skill_name];
        }
        person_skills
    }

    #[allow(unused)]
    pub fn smallest_sufficient_team(req_skills: Vec<String>, people: Vec<Vec<String>>) -> Vec<i32> {
        let n_skills = req_skills.len();
        let full_skills = (1 << n_skills) - 1;
        let skill_to_index: HashMap<String, Skills> = req_skills
            .into_iter()
            .enumerate()
            .map(|(i, skill)| (skill, i))
            .collect();

        // Skills -> number of people.
        let mut size = vec![-1; full_skills + 1];
        size[0] = 0;

        // Skills -> Team
        let mut team: Vec<Team> = vec![0; full_skills + 1];

        for (i, skill_names) in people.into_iter().enumerate() {
            let my_skills: Skills = Self::skill_names_to_skills(skill_names, &skill_to_index);
            // add this people to existing teams
            for ori_skill in 0..full_skills {
                if size[ori_skill] == -1 {
                    continue;
                }
                let new_skills: Skills = ori_skill | my_skills;
                let new_n = size[ori_skill] + 1;
                if size[new_skills] == -1 || new_n < size[new_skills] {
                    size[new_skills] = new_n;
                    team[new_skills] = team[ori_skill] | (1 << i);
                }
            }
        }

        // Team -> Vec<index>
        (0..64)
            .filter(|i| (team[full_skills] >> i) & 1 == 1)
            .collect()
    }
}
// @lc code=end
mod utils;
#[test]
fn test_solution() {
    macro_rules! test {
        ($skills:tt, $people:tt, $ans:tt) => {
            let f = |v: Vec<&str>| { v.into_iter().map(|s| s.to_owned()).collect() };
            let skills: Vec<String> = f(vec!$skills);
            let people:Vec<Vec<String>> = vec2d!$people.into_iter().map(f).collect();
            assert_eq!(
                Solution::smallest_sufficient_team(skills, people),
                vec!$ans
            );
        }
    };
    test!(
        ["java", "nodejs", "reactjs"],
        [["java"], ["nodejs"], ["nodejs", "reactjs"]],
        [0, 2]
    );
    test!(
        ["algorithms", "math", "java", "reactjs", "csharp", "aws"],
        [
            ["algorithms", "math", "java"],
            ["algorithms", "math", "reactjs"],
            ["java", "csharp", "aws"],
            ["reactjs", "csharp"],
            ["csharp", "math"],
            ["aws", "java"]
        ],
        [1, 2]
    );
}
