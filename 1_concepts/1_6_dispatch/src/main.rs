use task_1_6::{HashMapStorage, User, UserRepositoryDyn, UserRepositoryStatic};

fn main() {
    // Dynamic Dispatch Example
    let storage = Box::new(HashMapStorage::<u64, User>::new());
    let mut dyn_repo = UserRepositoryDyn::new(storage);

    let user1 = User {
        id: 1,
        email: "user1@example.com".into(),
        activated: true,
    };

    dyn_repo.add_user(user1.clone());
    println!("Dynamic Dispatch - User added: {:?}", dyn_repo.get_user(1));

    // Static Dispatch Example
    let static_storage = HashMapStorage::<u64, User>::new();
    let mut static_repo = UserRepositoryStatic::new(static_storage);

    let user2 = User {
        id: 2,
        email: "user2@example.com".into(),
        activated: true,
    };

    static_repo.add_user(user2.clone());
    println!(
        "Static Dispatch - User added: {:?}",
        static_repo.get_user(2)
    );
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dynamic_dispatch() {
        let storage = Box::new(HashMapStorage::<u64, User>::new());
        let mut repo = UserRepositoryDyn::new(storage);

        let user = User {
            id: 1,
            email: "test@example.com".into(),
            activated: true,
        };

        repo.add_user(user.clone());
        assert_eq!(repo.get_user(1), Some(&user));

        repo.remove_user(1);
        assert_eq!(repo.get_user(1), None);
    }

    #[test]
    fn test_static_dispatch() {
        let storage = HashMapStorage::<u64, User>::new();
        let mut repo = UserRepositoryStatic::new(storage);

        let user = User {
            id: 2,
            email: "static@example.com".into(),
            activated: true,
        };

        repo.add_user(user.clone());
        assert_eq!(repo.get_user(2), Some(&user));

        repo.remove_user(2);
        assert_eq!(repo.get_user(2), None);
    }
}
