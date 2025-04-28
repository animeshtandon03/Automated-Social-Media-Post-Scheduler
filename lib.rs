#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, Symbol, String, symbol_short};

// Data structure to hold a scheduled post
#[contracttype]
#[derive(Clone)]
pub struct ScheduledPost {
    pub post_id: u64,
    pub content: String,
    pub platform: String,
    pub scheduled_time: u64,
    pub posted: bool,
}

// Key to store and retrieve individual posts
#[contracttype]
pub enum PostKey {
    Post(u64),
}

const POST_COUNT: Symbol = symbol_short!("P_COUNT");

#[contract]
pub struct SocialSchedulerContract;

#[contractimpl]
impl SocialSchedulerContract {
    // Create a new post and schedule it
    pub fn create_post(env: Env, content: String, platform: String, scheduled_time: u64) -> u64 {
        let mut count = env.storage().instance().get(&POST_COUNT).unwrap_or(0);
        count += 1;

        let post = ScheduledPost {
            post_id: count,
            content,
            platform,
            scheduled_time,
            posted: false,
        };

        env.storage().instance().set(&PostKey::Post(count), &post);
        env.storage().instance().set(&POST_COUNT, &count);
        count
    }

    // View a specific post
    pub fn view_post(env: Env, post_id: u64) -> ScheduledPost {
        env.storage().instance().get(&PostKey::Post(post_id)).unwrap_or(ScheduledPost {
            post_id: 0,
            content: String::from_str(&env, "Not Found"),
            platform: String::from_str(&env, "Unknown"),
            scheduled_time: 0,
            posted: false,
        })
    }

    // Mark a post as posted (triggered by a scheduler/oracle)
    pub fn mark_posted(env: Env, post_id: u64) {
        let mut post = Self::view_post(env.clone(), post_id);
        if !post.posted {
            post.posted = true;
            env.storage().instance().set(&PostKey::Post(post_id), &post);
        }
    }
}
