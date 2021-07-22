/// 树状数组，单点 incr(1)，
/// 计算区间和 sum(0, x)
struct StreamRank {
    zeros: i32,
    arr: Vec<i32>,
}

fn lowbit(x: i32) -> i32 {
    x & (-x)
}

const n: usize = 65535;

impl StreamRank {
    fn new() -> Self {
        Self {
            zeros: 0,
            arr: vec![0; 65536],
        }
    }

    fn track(&mut self, mut x: i32) {
        if x == 0 {
            self.zeros += 1;
            return;
        }
        while (x as usize <= n) {
            self.arr[x as usize] += 1;
            x += lowbit(x);
        }
    }

    fn get_rank_of_number(&self, mut x: i32) -> i32 {
        let mut ans = self.zeros;
        while x > 0 {
            ans += self.arr[x as usize];
            x -= lowbit(x);
        }
        ans
    }
}
