use std::borrow::Cow;
use task_1_6::{Storage, User, UserRepositoryDyn};

// Command structure
pub struct CreateUser {
    pub id: u64,
    pub email: Cow<'static, str>,
}

// UserRepository trait (mocked for testing purposes)
pub trait UserRepository {
    fn add_user(&mut self, user: User);
    fn get_user(&self, id: u64) -> Option<&User>;
}

// CommandHandler trait
pub trait CommandHandler<C: ?Sized> {
    type Context: ?Sized;
    type Result;

    fn handle_command(&self, cmd: &C, ctx: &mut Self::Context) -> Self::Result;
}

// Implement CommandHandler for User and CreateUser
impl CommandHandler<CreateUser> for User {
    type Context = dyn UserRepository;
    type Result = Result<(), String>;

    fn handle_command(&self, cmd: &CreateUser, ctx: &mut Self::Context) -> Self::Result {
        if ctx.get_user(cmd.id).is_some() {
            Err("User already exists".to_string())
        } else {
            let user = User {
                id: cmd.id,
                email: cmd.email.clone(),
                activated: true,
            };
            ctx.add_user(user);
            Ok(())
        }
    }
}

// Mock implementation of UserRepository for testing
pub struct MockUserRepository {
    users: Vec<User>,
}

impl MockUserRepository {
    pub fn new() -> Self {
        Self { users: vec![] }
    }
}

impl UserRepository for MockUserRepository {
    fn add_user(&mut self, user: User) {
        self.users.push(user);
    }

    fn get_user(&self, id: u64) -> Option<&User> {
        self.users.iter().find(|user| user.id == id)
    }
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_handler_create_user() {
        let mut mock_repo = MockUserRepository::new();
        let user_repo: &mut dyn UserRepository = &mut mock_repo;

        let cmd = CreateUser {
            id: 1,
            email: Cow::Borrowed("test@example.com"),
        };

        let handler = User {
            id: 0,
            email: Cow::Borrowed("placeholder"),
            activated: false,
        };

        // Add a user successfully
        let result = handler.handle_command(&cmd, user_repo);
        assert!(result.is_ok());
        assert!(user_repo.get_user(1).is_some());

        // Try adding the same user again
        let result = handler.handle_command(&cmd, user_repo);
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), "User already exists");
    }
}

fn main() {
    println!("Run `cargo test` to execute the tests.");
}
