/*
 * @lc app=leetcode id=355 lang=kotlin
 *
 * [355] Design Twitter
 */

import java.util.Queue

class Tweet() {
    val id: Int
    val t: Int
}

typealias Tweets = Queue<Tweet>

// @lc code=start
class Twitter() {

    val t = 0
    val tweets: Map<Int, Tweets>
    val followees: Map<Int, Set<Int>>

    /** Compose a new tweet. */
    fun postTweet(userId: Int, tweetId: Int) {
        userTweets: Tweets? = null
        if (tweets.containsKey((userId))) {
            userTweets = tweets[userId]
        } else {
            userTweets = tweets[userId] = Tweets()
        }
        userTweets.push(tweetId)
    }

    /** Retrieve the 10 most recent tweet ids in the user's news feed. Each item in the news feed must be posted by users who the user followed or by the user herself. Tweets must be ordered from most recent to least recent. */
    fun getNewsFeed(userId: Int): List<Int> {

    }

    /** Follower follows a followee. If the operation is invalid, it should be a no-op. */
    fun follow(followerId: Int, followeeId: Int) {
        followee : Set<Int>? = followees.get(followerId)
        if (followee == null) {
            followee = followees[followeeId] = Set<Int>()
        }
        followee.add(followeeId)
    }

    /** Follower unfollows a followee. If the operation is invalid, it should be a no-op. */
    fun unfollow(followerId: Int, followeeId: Int) {
        followees.get(followerId)?.remove(followeeId)
    }

}

/**
 * Your Twitter object will be instantiated and called as such:
 * var obj = Twitter()
 * obj.postTweet(userId,tweetId)
 * var param_2 = obj.getNewsFeed(userId)
 * obj.follow(followerId,followeeId)
 * obj.unfollow(followerId,followeeId)
 */
// @lc code=end
fun main() {
    val twitter = Twitter()
    twitter.postTweet(1, 1)
    twitter.follow(2, 1)
    twitter.getNewsFeed(2)
    twitter.unfollow(2, 1)
    twitter.getNewsFeed(2)
}
