use pin_project::pin_project;
use std::fmt;
use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll};
use tokio::time::{self, Duration};

trait SayHi: fmt::Debug {
    fn say_hi(self: Pin<&Self>) {
        println!("hi from {:?}", self)
    }
}

trait MutMeSomehow {
    fn mut_me_somehow(self: Pin<&mut Self>) {}
}

impl<T> SayHi for Box<T>
where
    T: fmt::Debug,
{
    fn say_hi(self: Pin<&Self>) {
        println!("Hi from Box {:?}", self.get_ref())
    }
}

impl<T> MutMeSomehow for Box<T>
where
    T: fmt::Debug + Default,
{
    fn mut_me_somehow(self: Pin<&mut Self>) {
        let inner_value: &mut T = self.get_mut();

        *inner_value = Default::default();
    }
}

impl<T> SayHi for Rc<T>
where
    T: fmt::Debug,
{
    fn say_hi(self: Pin<&Self>) {
        println!("Hi from Rc {:?}", self.get_ref())
    }
}

impl<T> MutMeSomehow for Rc<T>
where
    T: fmt::Debug + Default,
{
    fn mut_me_somehow(self: Pin<&mut Self>) {
        let inner_value: &mut T = Rc::get_mut(self.get_mut()).unwrap();

        *inner_value = Default::default();
    }
}

impl<T> SayHi for Vec<T>
where
    T: fmt::Debug,
{
    fn say_hi(self: Pin<&Self>) {
        println!("Hi from Vec {:?}", self.get_ref())
    }
}

impl<T> MutMeSomehow for Vec<Pin<Box<T>>>
where
    T: fmt::Debug + Default,
{
    fn mut_me_somehow(self: Pin<&mut Self>) {
        *self.get_mut() = Vec::default();
    }
}

impl SayHi for String {
    fn say_hi(self: Pin<&Self>) {
        println!("Hi from String {:?}", self.get_ref())
    }
}

impl MutMeSomehow for String {
    fn mut_me_somehow(self: Pin<&mut Self>) {
        let inner_value: &mut String = self.get_mut();

        inner_value.push('!');
    }
}

impl<'a> SayHi for &'a [u8] {
    fn say_hi(self: Pin<&Self>) {
        println!("Hi from &[u8]: {:?}", self.get_ref());
    }
}

impl<'a> MutMeSomehow for &'a [u8] {
    fn mut_me_somehow(self: Pin<&mut Self>) {
        let array = self.get_mut();
        const SECRET: [u8; 3] = [1, 2, 3];
        std::mem::swap(array, &mut &SECRET[..]);
    }
}

#[pin_project]
struct MeasurableFuture<Fut> {
    #[pin]
    inner_future: Fut,
    started_at: std::time::Instant,
}

impl<Fut> Future for MeasurableFuture<Fut>
where
    Fut: Future,
{
    type Output = Fut::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        let this = self.project();
        let result = this.inner_future.poll(cx);
        if let Poll::Ready(_) = result {
            println!("Future took {:?} to complete", this.started_at.elapsed());
        }
        result
    }
}

#[tokio::main]
async fn main() {
    let future = time::sleep(Duration::from_secs(3));
    let measurable_future = MeasurableFuture {
        inner_future: future,
        started_at: std::time::Instant::now(),
    };
    measurable_future.await;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::pin::Pin;
    use std::rc::Rc;

    #[test]
    fn test_box_say_hi() {
        let value = Box::new(42);
        let pinned = Pin::new(&value);
        pinned.say_hi();
    }

    #[test]
    fn test_box_mut_me_somehow() {
        let mut value = Box::new(42);
        let mut pinned = Pin::new(&mut value);
        pinned.as_mut().mut_me_somehow();
        assert_eq!(*value, 0);
    }

    #[test]
    fn test_rc_say_hi() {
        let value = Rc::new(42);
        let pinned = Pin::new(&value);
        pinned.say_hi();
    }

    #[test]
    fn test_rc_mut_me_somehow() {
        let mut value = Rc::new(42);
        let mut pinned = Pin::new(&mut value);
        pinned.as_mut().mut_me_somehow();
        assert_eq!(*Rc::get_mut(&mut value).unwrap(), 0);
    }

    #[test]
    fn test_vec_say_hi() {
        let value = vec![1, 2, 3];
        let pinned = Pin::new(&value);
        pinned.say_hi();
    }

    #[test]
    fn test_vec_mut_me_somehow() {
        let mut value = vec![Pin::new(Box::new(1))];
        let mut pinned = Pin::new(&mut value);
        pinned.as_mut().mut_me_somehow();
        assert_eq!(value, Vec::<Pin<Box<i32>>>::default());
    }

    #[test]
    fn test_string_say_hi() {
        let value = String::from("Hello");
        let pinned = Pin::new(&value);
        pinned.say_hi();
    }

    #[test]
    fn test_string_mut_me_somehow() {
        let mut value = String::from("Hello");
        let mut pinned = Pin::new(&mut value);
        pinned.as_mut().mut_me_somehow();
        assert_eq!(value, "Hello!");
    }

    #[test]
    fn test_slice_say_hi() {
        let value: &[u8] = b"Hello";
        let pinned = Pin::new(&value);
        pinned.say_hi();
    }

    #[test]
    fn test_slice_mut_me_somehow() {
        let mut value: &[u8] = b"Hello";
        let mut pinned = Pin::new(&mut value);
        pinned.as_mut().mut_me_somehow();
        let fake_secret = [1, 2, 3];
        assert_eq!(value, &fake_secret);
    }
}
