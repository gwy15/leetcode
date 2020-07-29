struct Solution;

//
use std::collections::VecDeque;

type Maze = Vec<Vec<bool>>;
type Pos = (usize, usize);
type DistMap = Vec<Vec<i32>>; // -1 代表无法触及

impl Solution {
    /// BFS 寻找从单源开始的最短路径
    fn dist_map(maze: &Maze, from: Pos) -> DistMap {
        let (m, n) = (maze.len(), maze[0].len());
        let mut dist = vec![vec![-1; n]; m];

        let mut q = VecDeque::new();

        macro_rules! check {
            ($cond:expr, $i:expr, $j:expr, $cur:expr) => {
                if $cond && maze[$i][$j] && dist[$i][$j] == -1 {
                    q.push_back(($i, $j));
                    dist[$i][$j] = $cur;
                }
            };
        }

        check!(true, from.0, from.1, 0);

        let mut cur_dist = 1;
        while q.len() > 0 {
            for _ in 0..q.len() {
                let pt = q.pop_front().unwrap();
                let (i, j) = pt;

                check!(i > 0, i - 1, j, cur_dist);
                check!(j > 0, i, j - 1, cur_dist);
                check!(i + 1 < m, i + 1, j, cur_dist);
                check!(j + 1 < n, i, j + 1, cur_dist);
            }
            cur_dist += 1;
        }

        dist
    }

    /// 解析字符串形式迷宫。
    /// 返回 (maze, source_position, target_position, trap_positions, stone_positions)
    fn parse_maze(maze: Vec<String>) -> (Maze, Pos, Pos, Vec<Pos>, Vec<Pos>) {
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
                        '#' => false,
                        _ => {
                            match c {
                                'S' => source_position = Some((i, j)),
                                'T' => target_position = Some((i, j)),
                                'O' => stone_positions.push((i, j)),
                                'M' => trap_positions.push((i, j)),
                                _ => {}
                            }
                            true
                        }
                    })
                    .collect()
            })
            .collect();
        (
            maze,
            source_position.unwrap(),
            target_position.unwrap(),
            trap_positions,
            stone_positions,
        )
    }

    /// 根据两个 dist_map 找到最短的经过一个石头的路径长度
    fn find_bypass_stone(stones: &Vec<Pos>, dist_map1: &DistMap, dist_map2: &DistMap) -> i32 {
        let mut dist = i32::max_value();
        for &(stone_x, stone_y) in stones.iter() {
            let dist_1 = dist_map1[stone_x][stone_y];
            if dist_1 == -1 {
                continue;
            }
            let dist_2 = dist_map2[stone_x][stone_y];
            if dist_2 == -1 {
                continue;
            }
            dist = dist.min(dist_1 + dist_2);
        }
        dist
    }

    #[allow(unused)]
    pub fn minimal_steps(maze: Vec<String>) -> i32 {
        macro_rules! fail {
            ($cond:expr) => {
                if $cond {
                    return -1;
                }
            };
        }
        // 转换为 Maze 类型（true 代表可通过）
        let (maze, source_position, target_position, trap_positions, stone_positions) =
            Self::parse_maze(maze);
        let cnt_trap = trap_positions.len();

        // 生成起点、终点、各个机关的 dist map
        let dist_source = Self::dist_map(&maze, source_position);
        let dist_target = Self::dist_map(&maze, target_position);

        // 边界情况：没有机关，直接走到终点
        if cnt_trap == 0 {
            return dist_source[target_position.0][target_position.1];
        }
        // 边界情况：没有石头，但是有机关，直接 GG
        fail!(stone_positions.len() == 0);
        // 边界情况：处理起点到终点不连通的情况
        fail!(dist_source[target_position.0][target_position.1] == -1);

        // 生成各个机关的 dist map
        let mut dist_trap: Vec<DistMap> = (0..cnt_trap)
            .map(|i| Self::dist_map(&maze, trap_positions[i]))
            .collect();
        // 边界情况：处理无法到达机关的情况
        for i in 0..cnt_trap {
            fail!(dist_trap[i][source_position.0][source_position.1] == -1);
        }

        // 生成起点到机关的（经过石头）距离
        let dist_between_source_trap: Vec<i32> = (0..cnt_trap)
            .map(|i| Self::find_bypass_stone(&stone_positions, &dist_source, &dist_trap[i]))
            .collect();
        // 生成机关到机关（经过石头）的距离矩阵
        let mut dist_between_traps = vec![vec![i32::max_value(); cnt_trap]; cnt_trap];
        for i in 0..cnt_trap {
            for j in i + 1..cnt_trap {
                let dist = Self::find_bypass_stone(&stone_positions, &dist_trap[i], &dist_trap[j]);
                fail!(dist == i32::max_value());
                dist_between_traps[i][j] = dist;
                dist_between_traps[j][i] = dist;
            }
        }

        // DP 计算起点到遍历结束的距离
        let state_max = 1 << cnt_trap;
        let mut dp = vec![vec![i32::max_value(); cnt_trap]; state_max];

        for i in 0..cnt_trap {
            dp[1 << i][i] = dist_between_source_trap[i];
        }

        for state in 1..state_max - 1 {
            for i in 0..cnt_trap {
                if (state & (1 << i)) == 0 {
                    continue;
                }
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
                    let cost = dp[state][i] + dist_between_traps[i][j];
                    // 更新 cost
                    dp[new_state][j] = dp[new_state][j].min(cost);
                }
            }
        }
        // 计算遍历结束到终点的距离
        (0..cnt_trap)
            .map(|i| {
                let (trap_x, trap_y) = trap_positions[i];
                dp[state_max - 1][i] + dist_target[trap_x][trap_y]
            })
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
    test!(["T#O", ".##", "O..", ".#.", "OSM", "#.."], 9);
}
