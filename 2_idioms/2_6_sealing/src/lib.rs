pub mod my_error;
pub mod my_iterator_ext;
pub use self::{my_error::MyError, my_iterator_ext::MyIteratorExt};
pub use std::fmt;

#[cfg(test)]
mod tests {
    /// use super::*;

    /// This demonstrates that the `MyError` trait cannot be implemented or overridden in this module.
    ///
    /// ```compile_fail
    /// use super::MyError;

    /// struct TestError;

    /// impl fmt::Debug for TestError {
    ///     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    ///         write!(f, "TestError(Debug)")
    ///     }
    /// }

    /// impl fmt::Display for TestError {
    ///     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    ///         write!(f, "TestError(Display)")
    ///     }
    /// }
    /// impl MyError for TestError {
    ///     fn source(&self) -> Option<&(dyn MyError + 'static)> {
    ///         None
    ///     }

    ///     // ERROR: Cannot override `type_id` because it requires a private token.
    ///     fn type_id(&self, _: super::my_error::private::Token) -> std::any::TypeId {
    ///         std::any::TypeId::of::<Self>()
    ///     }
    /// }
    /// ```
    #[test]
    fn test_my_error_sealing() {}

    /// This demonstrates that the `MyIteratorExt` trait cannot be implemented in this module.
    ///
    /// ```compile_fail
    /// use super::MyIteratorExt;

    /// struct MyCustomIterator;

    /// impl Iterator for MyCustomIterator {
    ///     type Item = i32;

    ///     fn next(&mut self) -> Option<Self::Item> {
    ///         None
    ///     }
    /// }

    /// // ERROR: Cannot implement `MyIteratorExt` because it is sealed.
    /// impl MyIteratorExt for MyCustomIterator {}
    /// ```
    #[test]
    fn test_my_iterator_ext_sealing() {}
}
