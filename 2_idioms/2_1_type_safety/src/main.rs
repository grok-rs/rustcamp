use std::marker::PhantomData;

mod state {
    pub struct New;
    pub struct Unmoderated;
    pub struct Published;
    pub struct Deleted;
}

pub struct Post<State> {
    id: u64,
    title: String,
    body: String,
    state: PhantomData<State>,
}

impl Post<state::New> {
    pub fn new(id: u64, title: String, body: String) -> Self {
        Self {
            id,
            title,
            body,
            state: PhantomData,
        }
    }

    /// Transition from `New` to `Unmoderated`
    pub fn publish(self) -> Post<state::Unmoderated> {
        Post {
            id: self.id,
            title: self.title,
            body: self.body,
            state: PhantomData,
        }
    }
}

impl Post<state::Unmoderated> {
    /// Transition from `Unmoderated` to `Published`
    pub fn allow(self) -> Post<state::Published> {
        Post {
            id: self.id,
            title: self.title,
            body: self.body,
            state: PhantomData,
        }
    }

    /// Transition from `Unmoderated` to `Deleted`
    pub fn deny(self) -> Post<state::Deleted> {
        Post {
            id: self.id,
            title: self.title,
            body: self.body,
            state: PhantomData,
        }
    }
}

impl Post<state::Published> {
    /// Transition from `Published` to `Deleted`
    pub fn delete(self) -> Post<state::Deleted> {
        Post {
            id: self.id,
            title: self.title,
            body: self.body,
            state: PhantomData,
        }
    }
}

impl Post<state::Deleted> {
    // No further state transitions allowed from Deleted
}

fn main() {
    let new_post = Post::new(1, "Title".to_string(), "Body".to_string());
    let unmoderated_post = new_post.publish();
    let published_post = unmoderated_post.allow();
    let _deleted_post = published_post.delete();

    // Uncommenting the following line would cause a compile-time error:
    // let invalid_transition = new_post.delete();
}

/// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_post_lifecycle() {
        let new_post = Post::new(1, "Title".to_string(), "Body".to_string());
        let unmoderated_post = new_post.publish();
        let published_post = unmoderated_post.allow();
        let _deleted_post = published_post.delete();

        // Uncommenting the following line would cause a compile-time error:
        // let invalid_transition = new_post.delete();
    }

    #[test]
    fn test_deny_unmoderated_post() {
        let new_post = Post::new(2, "Another Title".to_string(), "Another Body".to_string());
        let unmoderated_post = new_post.publish();
        let _deleted_post = unmoderated_post.deny();

        // Uncommenting the following line would cause a compile-time error:
        // let invalid_transition = deleted_post.deny();
    }
}
