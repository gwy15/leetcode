const MOD: i32 = 1_000_000_000 + 7;
const MOD64: i64 = MOD as i64;

struct Matrix3D {
    data: [[i32; 3]; 3],
}

impl std::ops::MulAssign<&Matrix3D> for Matrix3D {
    fn mul_assign(&mut self, rhs: &Matrix3D) {
        self.data = Self::multi_data(&self.data, &rhs.data);
    }
}

impl std::clone::Clone for Matrix3D {
    fn clone(&self) -> Matrix3D {
        Matrix3D {
            data: self.data.clone(),
        }
    }
}

impl Matrix3D {
    pub fn identity() -> Matrix3D {
        Matrix3D {
            data: [[1, 0, 0], [0, 1, 0], [0, 0, 1]],
        }
    }

    fn multi_data(a: &[[i32; 3]; 3], b: &[[i32; 3]; 3]) -> [[i32; 3]; 3] {
        let mut data: [[i64; 3]; 3] = [[0; 3]; 3];
        for i in 0..3 {
            for j in 0..3 {
                for k in 0..3 {
                    data[i][j] += a[i][k] as i64 * b[k][j] as i64;
                }
            }
        }
        let mut ret = [[0; 3]; 3];
        for i in 0..3 {
            for j in 0..3 {
                ret[i][j] = (data[i][j] % MOD64) as i32;
            }
        }
        ret
    }

    pub fn norm(&mut self) {
        for i in 0..3 {
            for j in 0..3 {
                self.data[i][j] %= MOD;
            }
        }
    }

    pub fn dot_self(&mut self) {
        self.data = Self::multi_data(&self.data, &self.data);
    }

    pub fn power(&self, mut b: i32) -> Matrix3D {
        let mut ret = Matrix3D::identity();
        let mut a: Matrix3D = self.clone();
        while b != 0 {
            if (b & 1) != 0 {
                ret *= &a; // ret = ret * a
            }
            a.dot_self(); // a = a * a
            b >>= 1;
        }
        ret
    }

    pub fn get(&self) -> i32 {
        let row = &self.data[0];
        ((2 * row[0] as i64 + row[1] as i64 + row[2] as i64) % MOD64) as i32
    }
}

struct Solution {}

impl Solution {
    #[allow(unused)]
    pub fn num_tilings(n: i32) -> i32 {
        // dp:
        // f(x) = 2 f(x-1) + f(x-3)
        // matrix:
        //  f(x)    / 2  0  1 \  f(x-1)
        // f(x-1) = | 1  0  0 |  f(x-2) = A ** (x-2) (2 1 1)^T
        // f(x-2)   \ 0  1  0 /  f(x-3)
        match n {
            0 => 1,
            1 => 1,
            2 => 2,
            3 => 5,
            n => {
                let a = Matrix3D {
                    data: [[2, 0, 1], [1, 0, 0], [0, 1, 0]],
                };
                let a_n = a.power(n - 2);
                a_n.get()
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_matrix() {
        let mut i = Matrix3D::identity();
        assert_eq!(i.get(), 2);
        i.dot_self();
        assert_eq!(i.get(), 2);
    }

    #[test]
    fn test_solution() {
        assert_eq!(Solution::num_tilings(0), 1);
        assert_eq!(Solution::num_tilings(1), 1);
        assert_eq!(Solution::num_tilings(2), 2);
        assert_eq!(Solution::num_tilings(3), 5);
        assert_eq!(Solution::num_tilings(4), 11);
        assert_eq!(Solution::num_tilings(6), 53);
        assert_eq!(Solution::num_tilings(10), 1255);
        assert_eq!(Solution::num_tilings(30), 312342182);
    }
}
