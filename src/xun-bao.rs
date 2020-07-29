struct Solution;

//
use std::collections::VecDeque;

type Maze = Vec<Vec<bool>>;
type Pos = (usize, usize);

impl Solution {
    fn dist(maze: &Maze, from: Pos, to: Pos) -> Option<i32> {
        let (m, n) = (maze.len(), maze[0].len());
        let mut visited = vec![vec![false; n]; m];
        let mut q = VecDeque::new();

        macro_rules! check {
            ($cond:expr, $i:expr, $j:expr) => {
                if $cond && maze[$i][$j] && !visited[$i][$j] {
                    q.push_back(($i, $j));
                    visited[$i][$j] = true;
                }
            };
        }

        check!(true, from.0, from.1);

        let mut dist = 0;
        while q.len() > 0 {
            for _ in 0..q.len() {
                let pt = q.pop_front().unwrap();
                if pt == to {
                    return Some(dist);
                }

                let (i, j) = pt;
                check!(i > 0, i - 1, j);
                check!(j > 0, i, j - 1);
                check!(i + 1 < m, i + 1, j);
                check!(j + 1 < n, i, j + 1);
            }
            dist += 1;
        }

        None
    }

    /// 计算机关到机关之间经过一个石头的最短路径，或是从起点到第一个机关的距离
    fn dist_bypass_stone(maze: &Maze, from: Pos, to: Pos, stone_positions: &[Pos]) -> Option<i32> {
        let mut min_dist = i32::max_value();
        for &stone_pos in stone_positions.iter() {
            // 经过 石头 k 的距离
            let dist_i_k_j = Self::dist(&maze, from, stone_pos).and_then(|dist1| {
                Self::dist(&maze, stone_pos, to).and_then(|dist2| Some(dist1 + dist2))
            });
            min_dist = match dist_i_k_j {
                // 不可连接
                None => continue,
                Some(dist) => min_dist.min(dist),
            };
        }
        if min_dist == i32::max_value() {
            None
        } else {
            Some(min_dist)
        }
    }

    #[allow(unused)]
    pub fn minimal_steps(maze: Vec<String>) -> i32 {
        // 转换为 Maze 类型（true 代表可通过）
        let mut stone_positions = vec![];
        let mut trap_positions = vec![];
        let mut source_position = None;
        let mut target_position = None;
        let maze: Maze = maze
            .into_iter()
            .enumerate()
            .map(|(i, s)| {
                s.chars()
                    .enumerate()
                    .map(|(j, c)| match c {
                        '.' => true,
                        'S' => {
                            source_position = Some((i, j));
                            true
                        }
                        'T' => {
                            target_position = Some((i, j));
                            true
                        }
                        'O' => {
                            stone_positions.push((i, j));
                            true
                        }
                        '#' => false,
                        'M' => {
                            trap_positions.push((i, j));
                            true
                        }
                        _ => unreachable!(),
                    })
                    .collect()
            })
            .collect();
        let source_position = source_position.unwrap();
        let target_position = target_position.unwrap();

        let cnt_trap = trap_positions.len();

        if cnt_trap == 0 {
            return Self::dist(&maze, source_position, target_position).unwrap_or(-1);
        }

        // 计算起点到终点是否连通
        if Self::dist(&maze, source_position, target_position).is_none() {
            return -1;
        }

        // 计算 机关到机关 的距离矩阵
        let mut dist = vec![vec![i32::max_value(); cnt_trap]; cnt_trap];
        for i in 0..cnt_trap {
            for j in i + 1..cnt_trap {
                let dist_i_j = Self::dist_bypass_stone(
                    &maze,
                    trap_positions[i],
                    trap_positions[j],
                    &stone_positions,
                );
                if dist_i_j.is_none() {
                    return -1;
                }
                dist[i][j] = dist_i_j.unwrap();
                dist[j][i] = dist_i_j.unwrap();
            }
        }

        // 计算从起点到机关的距离
        let mut dist_source = vec![i32::max_value(); cnt_trap];
        for i in 0..cnt_trap {
            let dist_i = Self::dist_bypass_stone(
                &maze,
                source_position,
                trap_positions[i],
                &stone_positions,
            );
            if dist_i.is_none() {
                return -1;
            }
            dist_source[i] = dist_i.unwrap();
        }

        // 计算从终点到机关的距离
        let mut dist_target = vec![i32::max_value(); cnt_trap];
        for i in 0..cnt_trap {
            let dist = Self::dist(&maze, trap_positions[i], target_position);
            if dist.is_none() {
                return -1;
            }
            dist_target[i] = dist.unwrap();
        }

        // 计算起点到遍历结束的距离
        let state_max = 1 << cnt_trap;
        let mut dp = vec![vec![i32::max_value(); cnt_trap]; state_max];

        for i in 0..cnt_trap {
            dp[1 << i][i] = dist_source[i];
        }

        for state in 1..state_max {
            for i in 0..cnt_trap {
                if (state & (1 << i)) == 0 {
                    continue;
                }
                // eprintln!("from state=0x{:04b}, i={}, cost={}", state, i, dp[state][i]);
                // 到达下一个（j 机关）
                for j in 0..cnt_trap {
                    if i == j {
                        continue;
                    }
                    let new_state = state | (1 << j);
                    // 已经到达过了
                    if state == new_state {
                        continue;
                    }
                    // 计算 新cost
                    let cost = dp[state][i] + dist[i][j];
                    // 更新 cost
                    dp[new_state][j] = dp[new_state][j].min(cost);
                }
            }
        }

        // 计算遍历结束到终点的距离
        (0..cnt_trap)
            .map(|i| dp[state_max - 1][i] + dist_target[i])
            .min()
            .unwrap()
    }
}

//
#[test]
fn test_solution() {
    macro_rules! test {
        ($maze:tt, $ans:expr) => {
            let maze = (vec!$maze).into_iter().map(|s|s.to_string()).collect();
            assert_eq!(
                Solution::minimal_steps(maze),
                $ans
            );
        }
    };
    test!(["S#O", "M..", "M.T"], 16);
    test!(["S#O", "M.#", "M.T"], -1);
    test!(["S#O", "M.T", "M.."], 17);
    test!(["MMMMM", "MS#MM", "MM#TO"], 95);
    test!(["..#..", ".S#..", "..#T#"], -1);
    test!(["##TOO#O#", "OO##O.S#", "###.O###", "#..O#O##"], 5);
}
