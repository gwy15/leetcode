struct ProductOfNumbers {
    product: Vec<i32>,
}

impl ProductOfNumbers {
    fn new() -> Self {
        ProductOfNumbers { product: vec![1] }
    }
    fn add(&mut self, num: i32) {
        if num == 0 {
            self.product.clear();
            self.product.push(1);
        } else {
            self.product.push(self.product.last().unwrap() * num);
        }
    }
    fn get_product(&self, k: i32) -> i32 {
        let k = k as usize;
        let n = self.product.len() - 1;
        if k > n {
            0
        } else {
            self.product[n] / self.product[n - k]
        }
    }
}

#[test]
fn test_solution() {
    let mut p = ProductOfNumbers::new();
    p.add(3);
    p.add(0);
    p.add(2);
    p.add(5);
    p.add(4);
    assert_eq!(p.get_product(2), 20);
    assert_eq!(p.get_product(3), 40);
    assert_eq!(p.get_product(4), 0);
}
