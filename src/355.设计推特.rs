/*
 * @lc app=leetcode.cn id=355 lang=rust
 *
 * [355] 设计推特
 */

// @lc code=start
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;

type User = i32;
// t, id
type Post = (u32, i32);

struct Twitter {
    t: u32,
    posts: HashMap<User, Vec<Post>>,
    follows: HashMap<User, HashSet<User>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Twitter {
    /** Initialize your data structure here. */
    fn new() -> Self {
        Twitter {
            t: 0,
            posts: HashMap::new(),
            follows: HashMap::new(),
        }
    }
    /** Compose a new tweet. */
    fn post_tweet(&mut self, user_id: i32, tweet_id: i32) {
        match self.posts.get_mut(&user_id) {
            None => {
                self.posts.insert(user_id, vec![(self.t, tweet_id)]);
            }
            Some(p) => p.push((self.t, tweet_id)),
        };
        self.t += 1;
    }
    /** Retrieve the 10 most recent tweet ids in the user's news feed. Each item in the news feed must be posted by users who the user followed or by the user herself. Tweets must be ordered from most recent to least recent. */
    fn get_news_feed(&mut self, user_id: i32) -> Vec<i32> {
        // get follows
        let follows = match self.follows.get_mut(&user_id) {
            None => {
                self.follows.insert(user_id, HashSet::new());
                self.follows.get_mut(&user_id).unwrap()
            }
            Some(f) => f,
        };
        follows.insert(user_id);
        // insert follows into heap
        let mut heap = BinaryHeap::new();
        for followee_id in follows.iter() {
            match self.posts.get(followee_id) {
                None => None,
                Some(posts) => posts.last(),
            }
            .and_then(|&(t, post_id)| Some(heap.push((t, post_id, followee_id))));
        }
        // keep popping from heap
        let mut posts = Vec::new();
        for _ in 0..10 {
            match heap.pop() {
                None => break,
                Some((t, post_id, followee_id)) => {
                    posts.push(post_id);
                    // retrieve another from followee_id
                    self.posts
                        .get(followee_id)
                        .unwrap()
                        .iter()
                        .rev()
                        .filter(|&&(tt, _post_id)| tt < t)
                        .next()
                        .and_then(|&(t, post_id)| Some(heap.push((t, post_id, followee_id))));
                }
            }
        }
        posts
    }
    /** Follower follows a followee. If the operation is invalid, it should be a no-op. */
    fn follow(&mut self, follower_id: i32, followee_id: i32) {
        match self.follows.get_mut(&follower_id) {
            None => {
                let mut f = HashSet::new();
                f.insert(followee_id);
                self.follows.insert(follower_id, f);
            }
            Some(f) => {
                f.insert(followee_id);
            }
        };
    }
    /** Follower unfollows a followee. If the operation is invalid, it should be a no-op. */
    fn unfollow(&mut self, follower_id: i32, followee_id: i32) {
        match self.follows.get_mut(&follower_id) {
            None => {}
            Some(f) => {
                f.remove(&followee_id);
            }
        };
    }
}

// @lc code=end
#[test]
fn test_solution() {
    {
        let mut obj = Twitter::new();
        obj.post_tweet(1, 5);
        assert_eq!(obj.get_news_feed(1), vec![5]);
        obj.follow(1, 2);
        obj.post_tweet(2, 6);
        assert_eq!(obj.get_news_feed(1), vec![6, 5]);
        obj.unfollow(1, 2);
        assert_eq!(obj.get_news_feed(1), vec![5]);
    }
    {
        let mut obj = Twitter::new();
        obj.post_tweet(1, 5);
        obj.post_tweet(1, 3);
        assert_eq!(obj.get_news_feed(1), vec![3, 5]);
    }
}
