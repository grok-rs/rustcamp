use std::{
    borrow::{Borrow, BorrowMut},
    num::NonZeroU64,
};

fn main() {
    // Simple example usage:
    println!("--- Example usage of the refactored code ---");

    // Create a new empty hydrated aggregate (version=Initial, snapshot=None).
    let mut hydrated = HydratedAggregate::<MyAggregate>::default();
    println!("HydratedAggregate before events: {:?}", hydrated);

    // Apply a couple events.
    hydrated.apply(MyEvent::Increment(10));
    hydrated.apply(MyEvent::Increment(5));

    println!("After events, version: {:?}", hydrated.version());
    println!("After events, state: {:?}", hydrated.state());

    // Create an identifier and wrap the hydrated aggregate into an Entity.
    let my_id = MyAggregateId(String::from("user-123"));
    let entity = Entity::new(my_id, hydrated);

    // Because we have an impl block for `Entity<I, A>` with
    //   where I: AggregateId<A>, A: Aggregate
    // we can call `identifier_str`.
    println!("Entity version = {:?}", entity.aggregate().version());

    // End of example
    println!("-------------------------------------------");
}

/// A projected state built from a series of events.
pub trait Aggregate: Default {
    /// A static string representing the type of the aggregate.
    ///
    /// Note: This should effectively be a constant value, and should never change.
    fn aggregate_type() -> &'static str;

    /// Consumes the event, applying its effects to the aggregate.
    fn apply<E>(&mut self, event: E)
    where
        E: AggregateEvent<Self>,
    {
        event.apply_to(self);
    }
}

/// An identifier for an aggregate.
pub trait AggregateId<A> {
    /// Gets the stringified aggregate identifier.
    fn as_str(&self) -> &str;
}

/// A thing that happened.
pub trait Event {
    /// A static description of the event.
    fn event_type(&self) -> &'static str;
}

/// An event that can be applied to an aggregate.
pub trait AggregateEvent<A>: Event {
    /// Consumes the event, applying its effects to the aggregate.
    fn apply_to(self, aggregate: &mut A);
}

/// Represents an event sequence number, starting at 1
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct EventNumber(NonZeroU64);

impl EventNumber {
    pub const MIN_VALUE: EventNumber = EventNumber(unsafe { NonZeroU64::new_unchecked(1) });

    /// Increments the event number to the next value.
    #[inline]
    pub fn incr(&mut self) {
        self.0 = NonZeroU64::new(self.0.get() + 1).unwrap();
    }
}

/// An aggregate version.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Version {
    /// The version of an aggregate that has not had any events applied to it.
    Initial,
    /// The version of the last event applied to the aggregate.
    Number(EventNumber),
}

impl Default for Version {
    #[inline]
    fn default() -> Self {
        Version::Initial
    }
}

impl Version {
    #[inline]
    pub fn new(number: u64) -> Self {
        NonZeroU64::new(number)
            .map(EventNumber)
            .map(Version::Number)
            .unwrap_or(Version::Initial)
    }

    #[inline]
    pub fn incr(&mut self) {
        match *self {
            Version::Initial => *self = Version::Number(EventNumber::MIN_VALUE),
            Version::Number(ref mut en) => en.incr(),
        }
    }
}

#[derive(Debug)]
pub struct HydratedAggregate<A> {
    version: Version,
    snapshot_version: Option<Version>,
    state: A,
}

impl<A: Default> Default for HydratedAggregate<A> {
    fn default() -> Self {
        HydratedAggregate {
            version: Version::Initial,
            snapshot_version: None,
            state: A::default(),
        }
    }
}

impl<A> HydratedAggregate<A> {
    /// The current version of the aggregate.
    pub fn version(&self) -> Version {
        self.version
    }

    /// The version of the snapshot from which the aggregate was loaded.
    pub fn snapshot_version(&self) -> Option<Version> {
        self.snapshot_version
    }

    /// Updates the snapshot version. Generally used to indicate that a snapshot was taken.
    pub fn set_snapshot_version(&mut self, new_snapshot_version: Version) {
        self.snapshot_version = Some(new_snapshot_version);
    }

    /// The actual aggregate.
    pub fn state(&self) -> &A {
        &self.state
    }

    /// Applies a sequence of events to the internal aggregate.
    pub fn apply_events<E, I>(&mut self, events: I)
    where
        A: Aggregate,
        E: AggregateEvent<A>,
        I: IntoIterator<Item = E>,
    {
        for event in events {
            self.apply(event);
        }
    }

    /// Applies a single event to the aggregate, keeping track of the new aggregate version.
    pub fn apply<E>(&mut self, event: E)
    where
        A: Aggregate,
        E: AggregateEvent<A>,
    {
        self.state.apply(event);
        self.version.incr();
    }
}

impl<A> AsRef<A> for HydratedAggregate<A> {
    fn as_ref(&self) -> &A {
        &self.state
    }
}

impl<A> Borrow<A> for HydratedAggregate<A> {
    fn borrow(&self) -> &A {
        &self.state
    }
}

/// An identified, specific instance of a hydrated aggregate.
pub struct Entity<I, A> {
    id: I,
    aggregate: HydratedAggregate<A>,
}

impl<I, A> Entity<I, A> {
    /// Creates a new entity from an identifier and an associated hydrated aggregate.
    pub fn new(id: I, aggregate: HydratedAggregate<A>) -> Self {
        Entity { id, aggregate }
    }

    /// The entity's identifier.
    pub fn id(&self) -> &I {
        &self.id
    }

    /// An immutable reference to the underlying aggregate.
    pub fn aggregate(&self) -> &HydratedAggregate<A> {
        &self.aggregate
    }

    /// A mutable reference to the underlying aggregate.
    pub fn aggregate_mut(&mut self) -> &mut HydratedAggregate<A> {
        &mut self.aggregate
    }
}

impl<I, A> Entity<I, A>
where
    I: AggregateId<A>,
    A: Aggregate,
{
    pub fn identifier_str(&self) -> &str {
        self.id.as_str()
    }
}

impl<I, A> From<Entity<I, A>> for HydratedAggregate<A> {
    fn from(entity: Entity<I, A>) -> Self {
        entity.aggregate
    }
}

impl<I, A> AsRef<HydratedAggregate<A>> for Entity<I, A> {
    fn as_ref(&self) -> &HydratedAggregate<A> {
        &self.aggregate
    }
}

impl<I, A> AsMut<HydratedAggregate<A>> for Entity<I, A> {
    fn as_mut(&mut self) -> &mut HydratedAggregate<A> {
        &mut self.aggregate
    }
}

impl<I, A> Borrow<HydratedAggregate<A>> for Entity<I, A> {
    fn borrow(&self) -> &HydratedAggregate<A> {
        &self.aggregate
    }
}

impl<I, A> BorrowMut<HydratedAggregate<A>> for Entity<I, A> {
    fn borrow_mut(&mut self) -> &mut HydratedAggregate<A> {
        &mut self.aggregate
    }
}

// We can also let `Entity<I, A>` borrow as `A` if we like:
impl<I, A> Borrow<A> for Entity<I, A> {
    fn borrow(&self) -> &A {
        self.aggregate.borrow()
    }
}

/// A concrete type that implements `Aggregate`.
#[derive(Debug, Default)]
pub struct MyAggregate {
    pub count: i64,
}

impl Aggregate for MyAggregate {
    fn aggregate_type() -> &'static str {
        "MyAggregate"
    }
}

/// A simple `Event` type for demonstration.
#[derive(Debug)]
pub enum MyEvent {
    Increment(i64),
    Decrement(i64),
}

impl Event for MyEvent {
    fn event_type(&self) -> &'static str {
        match *self {
            MyEvent::Increment(_) => "Increment",
            MyEvent::Decrement(_) => "Decrement",
        }
    }
}

/// Applying our `MyEvent` to `MyAggregate`.
impl AggregateEvent<MyAggregate> for MyEvent {
    fn apply_to(self, aggregate: &mut MyAggregate) {
        match self {
            MyEvent::Increment(x) => {
                aggregate.count += x;
            }
            MyEvent::Decrement(x) => {
                aggregate.count -= x;
            }
        }
    }
}

/// Example of an ID type for `MyAggregate`.
#[derive(Debug)]
pub struct MyAggregateId(pub String);

impl AggregateId<MyAggregate> for MyAggregateId {
    fn as_str(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_increments() {
        let mut agg = HydratedAggregate::<MyAggregate>::default();
        assert_eq!(agg.version(), Version::Initial);

        // Apply single event
        agg.apply(MyEvent::Increment(10));
        match agg.version() {
            Version::Number(n) => assert_eq!(n.0.get(), 1),
            _ => panic!("Expected version to be Number(1) after first event"),
        }

        // Apply multiple events
        agg.apply_events(vec![MyEvent::Increment(5), MyEvent::Decrement(3)]);
        match agg.version() {
            Version::Number(n) => assert_eq!(n.0.get(), 3),
            _ => panic!("Expected version to be Number(3) after three events"),
        }
    }

    #[test]
    fn test_aggregate_state_changes() {
        let mut agg = HydratedAggregate::<MyAggregate>::default();
        assert_eq!(agg.state().count, 0);

        agg.apply(MyEvent::Increment(10));
        assert_eq!(agg.state().count, 10);

        agg.apply_events(vec![MyEvent::Increment(5), MyEvent::Decrement(2)]);
        assert_eq!(agg.state().count, 13);
    }

    #[test]
    fn test_entity_id() {
        let id = MyAggregateId("abc123".to_owned());
        let agg = HydratedAggregate::<MyAggregate>::default();
        let entity = Entity::new(id, agg);
        assert_eq!(entity.identifier_str(), "abc123");
    }

    #[test]
    fn test_entity_versions() {
        let id = MyAggregateId("xyz".to_owned());
        let mut entity = Entity::new(id, HydratedAggregate::<MyAggregate>::default());

        assert_eq!(entity.aggregate().version(), Version::Initial);
        entity.aggregate_mut().apply(MyEvent::Increment(2));

        match entity.aggregate().version() {
            Version::Number(n) => assert_eq!(n.0.get(), 1),
            _ => panic!("Expected version to be Number(1)"),
        }
    }
}
