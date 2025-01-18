use std::borrow::Cow;
use std::collections::HashMap;

// Trait defining a generic Storage abstraction
pub trait Storage<K, V> {
    fn set(&mut self, key: K, val: V);
    fn get(&self, key: &K) -> Option<&V>;
    fn remove(&mut self, key: &K) -> Option<V>;
}

// Implementation of Storage trait using HashMap
pub struct HashMapStorage<K, V> {
    inner: HashMap<K, V>,
}

impl<K, V> HashMapStorage<K, V>
where
    K: Eq + std::hash::Hash,
{
    pub fn new() -> Self {
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

// User entity
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct User {
    pub id: u64,
    pub email: Cow<'static, str>,
    pub activated: bool,
}

// Dynamic Dispatch: UserRepository with trait objects
pub struct UserRepositoryDyn {
    storage: Box<dyn Storage<u64, User>>,
}

impl UserRepositoryDyn {
    pub fn new(storage: Box<dyn Storage<u64, User>>) -> Self {
        Self { storage }
    }

    pub fn add_user(&mut self, user: User) {
        self.storage.set(user.id, user);
    }

    pub fn get_user(&self, id: u64) -> Option<&User> {
        self.storage.get(&id)
    }

    pub fn update_user(&mut self, user: User) {
        self.storage.set(user.id, user);
    }

    pub fn remove_user(&mut self, id: u64) -> Option<User> {
        self.storage.remove(&id)
    }
}

// Static Dispatch: UserRepository with generic storage
pub struct UserRepositoryStatic<S>
where
    S: Storage<u64, User>,
{
    storage: S,
}

impl<S> UserRepositoryStatic<S>
where
    S: Storage<u64, User>,
{
    pub fn new(storage: S) -> Self {
        Self { storage }
    }

    pub fn add_user(&mut self, user: User) {
        self.storage.set(user.id, user);
    }

    pub fn get_user(&self, id: u64) -> Option<&User> {
        self.storage.get(&id)
    }

    pub fn update_user(&mut self, user: User) {
        self.storage.set(user.id, user);
    }

    pub fn remove_user(&mut self, id: u64) -> Option<User> {
        self.storage.remove(&id)
    }
}
