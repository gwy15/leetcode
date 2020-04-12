struct Solution;

type Point = Vec<i32>;
type Vec2D = Vec<i32>;

impl Solution {
    fn point_to_vec(p1: &Point, p2: &Point) -> Vec2D {
        vec![p2[0] - p1[0], p2[1] - p1[1]]
    }
    fn vec_cross(v1: &Vec2D, v2: &Vec2D) -> i32 {
        v1[0] * v2[1] - v1[1] * v2[0]
    }
    fn cvt(p: Vec<i32>) -> Vec<f64> {
        vec![p[0] as f64, p[1] as f64]
    }
    fn same_line_intersection(A: Point, B: Point, C: Point, D: Point) -> Vec<f64> {
        let use_y = A[0] == B[0];
        let f = |p: &Point| {
            if use_y {
                p[1]
            } else {
                p[0]
            }
        };
        let length = |v: &Vec<i32>| {
            v.iter().fold(i32::min_value(), |a, b| i32::max(a, *b))
                - v.iter().fold(i32::max_value(), |a, b| i32::min(a, *b))
        };
        let whole_length = length(&vec![f(&A), f(&B), f(&C), f(&D)]);
        let ab_length = length(&vec![f(&A), f(&B)]);
        let cd_length = length(&vec![f(&C), f(&D)]);
        if whole_length > ab_length + cd_length {
            // 没有重叠
            Vec::new()
        } else {
            let mut v = vec![A, B, C, D];
            v.sort_by_key(f);
            Solution::cvt(v[1].clone())
        }
    }
    #[allow(non_snake_case)]
    pub fn intersection(A: Point, B: Point, C: Point, D: Point) -> Vec<f64> {
        // 判断是否有交点，需要 AB 和 CD 互在两侧
        let AB = Solution::point_to_vec(&A, &B);
        let AC = Solution::point_to_vec(&A, &C);
        let AD = Solution::point_to_vec(&A, &D);
        //
        let cross_a1 = Solution::vec_cross(&AB, &AC);
        let cross_a2 = Solution::vec_cross(&AB, &AD);
        let cross_a = cross_a1 * cross_a2;
        if cross_a > 0 {
            // 同一侧
            return Vec::new();
        }
        let CD = Solution::point_to_vec(&C, &D);
        let CA = Solution::point_to_vec(&C, &A);
        let CB = Solution::point_to_vec(&C, &B);
        let cross_b1 = Solution::vec_cross(&CD, &CA);
        let cross_b2 = Solution::vec_cross(&CD, &CB);
        let cross_b = cross_b1 * cross_b2;
        if cross_b > 0 {
            return Vec::new();
        }
        // 判断是否有端点刚好重叠在线段上
        match (cross_a, cross_b) {
            (0, 0) => {
                // 共线
                Solution::same_line_intersection(A, B, C, D)
            }
            (0, _neg) => {
                // T 形状
                if cross_a1 == 0 {
                    Solution::cvt(C)
                } else {
                    Solution::cvt(D)
                }
            }
            (_neg, 0) => {
                // T
                if cross_b1 == 0 {
                    Solution::cvt(A)
                } else {
                    Solution::cvt(B)
                }
            }
            (_neg1, _neg2) => {
                // 交点
                let (xcd, ycd, xab, yab) = (D[0] - C[0], D[1] - C[1], B[0] - A[0], B[1] - A[1]);
                let (xca, yca) = (A[0] - C[0], A[1] - C[1]);
                let t1 = (xca * ycd - xcd * yca) as f64 / (xcd * yab - xab * ycd) as f64;
                vec![A[0] as f64 + t1 * xab as f64, A[1] as f64 + t1 * yab as f64]
            }
        }
    }
}

#[test]
fn test_solution() {
    macro_rules! test {
        ($a:tt, $b:tt, $c:tt, $d:tt, $ans:tt) => {
            assert_eq!(
                Solution::intersection(vec!$a, vec!$b, vec!$c, vec!$d),
                vec!$ans
            );
        }
    };
    test!([0, 0], [1, 0], [1, 1], [0, -1], [0.5, 0.]);
    test!([0, 0], [3, 3], [1, 1], [2, 2], [1., 1.]);
    test!([0, 0], [1, 1], [1, 0], [2, 1], []);

    test!([1, 1], [-1, 1], [1, 0], [-3, 2], [-1.0, 1.0]);
}
