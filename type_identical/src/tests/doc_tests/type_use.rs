//! Tests of using [`Assert::Type<T>`](crate::Assert::Type).

/// ```no_run
/// use type_identical::*;
///
/// fn aaa<T: ?Sized>(m: &T) -> &<T as Assert>::Type<T> {
///     m
/// }
/// ```
///
/// ```no_run
/// use type_identical::*;
///
/// fn aaa<T: ?Sized + Identical<U>, U: ?Sized>(m: &T) -> &T::Type<U> {
///     m
/// }
/// ```
///
/// ```no_run
/// use type_identical::*;
///
/// fn aaa<U: ?Sized>(m: &U) -> &<Assertion as Assert>::Type<U> {
///     m
/// }
/// ```
///
/// ```no_run
/// use type_identical::*;
///
/// fn aaa<T: ?Sized + Assert<Type<U> = T>, U: ?Sized>(m: &T) -> &T::Type<U> {
///     m
/// }
/// ```
#[allow(non_camel_case_types)]
struct _test_type_use;
