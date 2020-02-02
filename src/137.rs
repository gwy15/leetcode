impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        // A: first 2 (0->0, 1->1)
        // B: thrid (0->0, 1->2)
        // A+B
        let mut A = 0;
        let mut B = 0; // init as 0
        for num in nums {
            // do bitwise add
            let temp_A = (A ^ num) & (!(B & num));
            let temp_B = (A & num) | (B & !num);
            A = temp_A;
            B = temp_B;
        }
        A
    }
}