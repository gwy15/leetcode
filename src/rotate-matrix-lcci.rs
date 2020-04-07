struct Solution;

impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix[0].len();
        macro_rules! v {
            ($p:expr) => {
                matrix[$p.0][$p.1]
            };
        };
        macro_rules! set {
            ($p:expr, $v:expr) => {
                matrix[$p.0][$p.1] = $v
            };
        };
        for layer in 0..(n / 2) {
            // rotate layer
            let box_length = n - 2 * layer;
            let (lower, upper) = (layer, layer + box_length - 1);
            for i in 0..box_length - 1 {
                let p1 = (lower, lower + i);
                let p2 = (lower + i, upper);
                let p3 = (upper, upper - i);
                let p4 = (upper - i, lower);

                //
                let tmp = v!(p4);
                set!(p4, v!(p3));
                set!(p3, v!(p2));
                set!(p2, v!(p1));
                set!(p1, tmp);
            }
        }
    }
}

#[test]
fn test_solution() {
    macro_rules! test {
        ($mat:expr, $ans:expr) => {{
            let mut mat = $mat;
            Solution::rotate(&mut mat);
            assert_eq!(mat, $ans);
        }};
    };
    test!(
        vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]],
        vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3]]
    );
}
