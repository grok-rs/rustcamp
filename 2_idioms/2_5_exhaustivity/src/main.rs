use std::time::SystemTime;
use task_2_5::{event, user, EventSourced};

fn main() {
    let mut user = user::User::new("Alice");
    println!("User created: {:?}", user);

    let user_created_event =
        event::UserCreated::new(user.id, user::CreationDateTime(SystemTime::now()));

    let name_updated_event = event::UserNameUpdated::new(
        user.id,
        Some(user::Name(Box::from("Alice Updated"))),
        SystemTime::now(),
    );

    let user_online_event = event::UserBecameOnline::new(user.id, SystemTime::now());

    user.apply(&user_created_event);
    println!("After UserCreated: {:?}", user);

    user.apply(&name_updated_event);
    println!("After NameUpdated: {:?}", user);

    user.apply(&user_online_event);
    println!("After UserBecameOnline: {:?}", user);
}
