pub trait EventSourced<Ev: ?Sized> {
    fn apply(&mut self, event: &Ev);
}

pub mod user {
    use super::{event, EventSourced};
    use std::time::{SystemTime, UNIX_EPOCH};

    #[non_exhaustive]
    #[derive(Debug)]
    pub struct User {
        pub id: Id,
        pub name: Option<Name>,
        pub online_since: Option<SystemTime>,
        pub created_at: CreationDateTime,
        pub last_activity_at: LastActivityDateTime,
        pub deleted_at: Option<DeletionDateTime>,
    }

    impl User {
        pub fn new<S: Into<String>>(name: S) -> Self {
            let now = SystemTime::now();
            Self {
                id: Id::new(),
                name: Some(Name(name.into().into_boxed_str())),
                online_since: None,
                created_at: CreationDateTime(now),
                last_activity_at: LastActivityDateTime(now),
                deleted_at: None,
            }
        }
    }

    impl EventSourced<event::UserCreated> for User {
        fn apply(&mut self, ev: &event::UserCreated) {
            let event::UserCreated {
                user_id,
                at,
                new_field,
            } = *ev;

            println!("UserCreated: {:?}", new_field);
            self.id = user_id;
            self.created_at = at;
            self.last_activity_at = LastActivityDateTime(ev.at.0);
        }
    }

    impl EventSourced<event::UserNameUpdated> for User {
        fn apply(&mut self, ev: &event::UserNameUpdated) {
            self.name = ev.name.clone();
        }
    }

    impl EventSourced<event::UserBecameOnline> for User {
        fn apply(&mut self, ev: &event::UserBecameOnline) {
            self.online_since = Some(ev.at);
        }
    }

    impl EventSourced<event::UserBecameOffline> for User {
        fn apply(&mut self, ev: &event::UserBecameOffline) {
            self.online_since = None;
            self.last_activity_at = LastActivityDateTime(ev.at);
        }
    }

    impl EventSourced<event::UserDeleted> for User {
        fn apply(&mut self, ev: &event::UserDeleted) {
            self.deleted_at = Some(ev.at);
            self.last_activity_at = LastActivityDateTime(ev.at.0);
        }
    }

    #[non_exhaustive]
    #[derive(Debug)]
    pub enum Event {
        Created(event::UserCreated),
        NameUpdated(event::UserNameUpdated),
        Online(event::UserBecameOnline),
        Offline(event::UserBecameOffline),
        Deleted(event::UserDeleted),
    }

    impl EventSourced<Event> for User {
        fn apply(&mut self, ev: &Event) {
            match ev {
                Event::Created(ev) => self.apply(ev),
                Event::NameUpdated(ev) => self.apply(ev),
                Event::Online(ev) => self.apply(ev),
                Event::Offline(ev) => self.apply(ev),
                Event::Deleted(ev) => self.apply(ev),
            }
        }
    }

    #[derive(Clone, Copy, Debug)]
    pub struct Id(pub u64);

    impl Id {
        /// Generate a unique ID (for simplicity, just use a timestamp).
        pub fn new() -> Self {
            let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
            Id(now.as_secs())
        }
    }

    #[derive(Clone, Debug)]
    pub struct Name(pub Box<str>);

    #[derive(Clone, Copy, Debug)]
    pub struct CreationDateTime(pub SystemTime);

    #[derive(Clone, Copy, Debug)]
    pub struct LastActivityDateTime(pub SystemTime);

    #[derive(Clone, Copy, Debug)]
    pub struct DeletionDateTime(pub SystemTime);
}

pub mod event {
    use std::time::SystemTime;

    use super::user;

    #[non_exhaustive]
    #[derive(Debug)]
    pub struct UserCreated {
        pub user_id: user::Id,
        pub at: user::CreationDateTime,
        pub new_field: Option<i32>,
    }

    impl UserCreated {
        pub fn new(user_id: user::Id, at: user::CreationDateTime) -> Self {
            Self {
                user_id,
                at,
                new_field: Some(0),
            }
        }
    }

    #[non_exhaustive]
    #[derive(Debug)]
    pub struct UserNameUpdated {
        pub user_id: user::Id,
        pub name: Option<user::Name>,
        pub at: SystemTime,
    }

    impl UserNameUpdated {
        pub fn new(user_id: user::Id, name: Option<user::Name>, at: SystemTime) -> Self {
            Self { user_id, name, at }
        }
    }

    #[non_exhaustive]
    #[derive(Debug)]
    pub struct UserBecameOnline {
        pub user_id: user::Id,
        pub at: SystemTime,
        pub new_field: Option<String>,
    }

    impl UserBecameOnline {
        pub fn new(user_id: user::Id, at: SystemTime) -> Self {
            Self {
                user_id,
                at,
                new_field: None,
            }
        }
    }

    #[non_exhaustive]
    #[derive(Debug)]
    pub struct UserBecameOffline {
        pub user_id: user::Id,
        pub at: SystemTime,
    }

    #[non_exhaustive]
    #[derive(Debug)]
    pub struct UserDeleted {
        pub user_id: user::Id,
        pub at: user::DeletionDateTime,
    }
}
