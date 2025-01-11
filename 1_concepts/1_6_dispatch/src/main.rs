use std::borrow::Cow;
use std::collections::HashMap;

// Trait defining a generic Storage abstraction
trait Storage<K, V> {
    fn set(&mut self, key: K, val: V);
    fn get(&self, key: &K) -> Option<&V>;
    fn remove(&mut self, key: &K) -> Option<V>;
}

// Implementation of Storage trait using HashMap
struct HashMapStorage<K, V> {
    inner: HashMap<K, V>,
}

impl<K, V> HashMapStorage<K, V>
where
    K: Eq + std::hash::Hash,
{
    fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }
}

impl<K, V> Storage<K, V> for HashMapStorage<K, V>
where
    K: Eq + std::hash::Hash,
{
    fn set(&mut self, key: K, val: V) {
        self.inner.insert(key, val);
    }

    fn get(&self, key: &K) -> Option<&V> {
        self.inner.get(key)
    }

    fn remove(&mut self, key: &K) -> Option<V> {
        self.inner.remove(key)
    }
}

// Another implementation of Storage for dynamic dispatch example
struct VecStorage<K, V> {
    inner: Vec<(K, V)>,
}

impl<K, V> VecStorage<K, V>
where
    K: PartialEq,
{
    fn new() -> Self {
        Self { inner: Vec::new() }
    }
}

impl<K, V> Storage<K, V> for VecStorage<K, V>
where
    K: PartialEq,
{
    fn set(&mut self, key: K, val: V) {
        if let Some(pos) = self.inner.iter().position(|(k, _)| *k == key) {
            self.inner[pos] = (key, val);
        } else {
            self.inner.push((key, val));
        }
    }

    fn get(&self, key: &K) -> Option<&V> {
        self.inner.iter().find(|(k, _)| k == key).map(|(_, v)| v)
    }

    fn remove(&mut self, key: &K) -> Option<V> {
        if let Some(pos) = self.inner.iter().position(|(k, _)| k == key) {
            Some(self.inner.remove(pos).1)
        } else {
            None
        }
    }
}

// User entity
#[derive(Clone, Debug)]
struct User {
    id: u64,
    email: Cow<'static, str>,
    activated: bool,
}

// Dynamic Dispatch: UserRepository with trait objects
struct UserRepositoryDyn {
    storages: Vec<Box<dyn Storage<u64, User>>>,
}

impl UserRepositoryDyn {
    fn new(storages: Vec<Box<dyn Storage<u64, User>>>) -> Self {
        Self { storages }
    }

    fn add_user(&mut self, user: User) {
        for storage in &mut self.storages {
            storage.set(user.id, user.clone());
        }
    }

    fn get_user(&self, id: u64) -> Vec<Option<&User>> {
        self.storages
            .iter()
            .map(|storage| storage.get(&id))
            .collect()
    }

    fn update_user(&mut self, user: User) {
        for storage in &mut self.storages {
            storage.set(user.id, user.clone());
        }
    }

    fn remove_user(&mut self, id: u64) -> Vec<Option<User>> {
        self.storages
            .iter_mut()
            .map(|storage| storage.remove(&id))
            .collect()
    }
}

// Static Dispatch: UserRepository with generic storage
struct UserRepositoryStatic<S>
where
    S: Storage<u64, User>,
{
    storage: S,
}

impl<S> UserRepositoryStatic<S>
where
    S: Storage<u64, User>,
{
    fn new(storage: S) -> Self {
        Self { storage }
    }

    fn add_user(&mut self, user: User) {
        self.storage.set(user.id, user);
    }

    fn get_user(&self, id: u64) -> Option<&User> {
        self.storage.get(&id)
    }

    fn update_user(&mut self, user: User) {
        self.storage.set(user.id, user);
    }

    fn remove_user(&mut self, id: u64) -> Option<User> {
        self.storage.remove(&id)
    }
}

fn main() {
    // Dynamic Dispatch Example
    let hash_map_storage = Box::new(HashMapStorage::<u64, User>::new());
    let vec_storage = Box::new(VecStorage::<u64, User>::new());

    let mut dyn_repo = UserRepositoryDyn::new(vec![hash_map_storage, vec_storage]);

    let user1 = User {
        id: 1,
        email: Cow::Borrowed("user1@example.com"),
        activated: true,
    };

    dyn_repo.add_user(user1.clone());

    let results = dyn_repo.get_user(1);
    for (i, result) in results.iter().enumerate() {
        println!(
            "Dynamic Dispatch (Storage {}): {:?}",
            i + 1,
            result.map(|u| &u.email)
        );
    }

    let removed_results = dyn_repo.remove_user(1);
    for (i, result) in removed_results.iter().enumerate() {
        println!(
            "Dynamic Dispatch (Storage {}) removed: {:?}",
            i + 1,
            result.as_ref().map(|u| &u.email)
        );
    }

    // Static Dispatch Example
    let static_storage = HashMapStorage::<u64, User>::new();
    let mut static_repo = UserRepositoryStatic::new(static_storage);

    let user3 = User {
        id: 3,
        email: Cow::Borrowed("user3@example.com"),
        activated: true,
    };

    static_repo.add_user(user3);
    println!(
        "Static Dispatch: {:?}",
        static_repo.get_user(3).map(|u| &u.email)
    );

    static_repo.remove_user(3);
    println!("Static Dispatch removed: {:?}", static_repo.get_user(3));
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dynamic_dispatch() {
        let storage1 = Box::new(HashMapStorage::<u64, User>::new());
        let storage2 = Box::new(VecStorage::<u64, User>::new());
        let mut repo = UserRepositoryDyn::new(vec![storage1, storage2]);

        let user = User {
            id: 1,
            email: Cow::Borrowed("user1@example.com"),
            activated: true,
        };

        repo.add_user(user.clone());
        let results = repo.get_user(1);
        assert!(results.iter().all(|res| res.is_some()));

        let removed_results = repo.remove_user(1);
        assert!(removed_results.iter().all(|res| res.is_some()));
    }

    #[test]
    fn test_static_dispatch() {
        let storage = HashMapStorage::<u64, User>::new();
        let mut repo = UserRepositoryStatic::new(storage);

        let user = User {
            id: 1,
            email: Cow::Borrowed("user1@example.com"),
            activated: true,
        };

        repo.add_user(user);
        assert!(repo.get_user(1).is_some());

        repo.remove_user(1);
        assert!(repo.get_user(1).is_none());
    }
}
