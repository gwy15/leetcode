/*
* @lc app=leetcode.cn id=1912 lang=rust
*
* [1912] 设计电影租借系统
*/

// @lc code=start

use std::collections::{BTreeSet, HashMap};

struct MovieRentingSystem {
    // 还没有出借的 movie -> {price, shop}
    inventory: HashMap<i32, BTreeSet<(i32, i32)>>,
    // 价格速查表, movie, shop -> price
    prices: HashMap<(i32, i32), i32>,
    // 已出借的，{price, shop, movie}
    rent: BTreeSet<(i32, i32, i32)>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MovieRentingSystem {
    fn new(_n: i32, entries: Vec<Vec<i32>>) -> Self {
        let mut inventory: HashMap<i32, BTreeSet<(i32, i32)>> = HashMap::new();
        let mut prices = HashMap::new();
        let rent = BTreeSet::new();
        for entry in entries {
            let shop = unsafe { *entry.get_unchecked(0) };
            let movie = unsafe { *entry.get_unchecked(1) };
            let price = unsafe { *entry.get_unchecked(2) };

            prices.insert((movie, shop), price);
            inventory.entry(movie).or_default().insert((price, shop));
        }
        Self {
            inventory,
            prices,
            rent,
        }
    }

    fn search(&self, movie: i32) -> Vec<i32> {
        self.inventory
            .get(&movie)
            .map(|set| set.iter().take(5).map(|(_price, shop)| *shop).collect())
            .unwrap_or_default()
    }

    fn rent(&mut self, shop: i32, movie: i32) {
        let price = *self.prices.get(&(movie, shop)).unwrap();
        self.inventory
            .get_mut(&movie)
            .unwrap()
            .remove(&(price, shop));

        self.rent.insert((price, shop, movie));
    }

    fn drop(&mut self, shop: i32, movie: i32) {
        let price = *self.prices.get(&(movie, shop)).unwrap();
        self.inventory
            .get_mut(&movie)
            .unwrap()
            .insert((price, shop));

        self.rent.remove(&(price, shop, movie));
    }

    fn report(&self) -> Vec<Vec<i32>> {
        self.rent
            .iter()
            .take(5)
            .map(|&(_price, shop, movie)| vec![shop, movie])
            .collect()
    }
}

// @lc code=end

mod utils;

#[test]
fn test_solution() {
    let mut x = MovieRentingSystem::new(
        3,
        vec2d![
            [0, 1, 5],
            [0, 2, 6],
            [0, 3, 7],
            [1, 1, 4],
            [1, 2, 7],
            [2, 1, 5]
        ],
    );
    // [1], [0, 1], [1, 2], [], [1, 2], [2]]
    assert_eq!(x.search(1), vec![1, 0, 2]);
    x.rent(0, 1);
    x.rent(1, 2);
    assert_eq!(x.report(), vec2d![[0, 1], [1, 2]]);
    x.drop(1, 2);
    assert_eq!(x.search(2), vec![0, 1]);
}
