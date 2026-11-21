//! Tests of [`Identical`](crate::Identical), ensuring that
//! the implementations of [`Identical`](crate::Identical)
//! are forbidden to be extended outside this [`crate`].

/// ```compile_fail
/// struct Bar;
/// impl type_identical::private_identical::PrivateIdentical<()> for Bar {}
/// ```
///
///
///
/// ```compile_fail
/// use type_identical::*;
///
/// struct Bar<T: ?Sized>(T);
///
/// impl<T: ?Sized> Identical<T> for Bar<T> {}
/// ```
///
///
///
/// ```compile_fail
/// use type_identical::*;
///
/// struct Bar<T: ?Sized>(T);
///
/// impl<T: ?Sized> Identical<T> for Bar<T>
/// where
///     Bar<T>: type_identical::private_identical::PrivateIdentical<T>,
/// {
/// }
/// ```
#[allow(non_camel_case_types)]
struct _test_no_impl_extension;
