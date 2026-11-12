//! Tests of constraining types to be identical to each other.

/// ```compile_fail
/// use type_identical::*;
///
/// fn bbb<T: ?Sized + Identical<U>, U: ?Sized>() {}
///
/// bbb::<i32, ()>();
/// ```
///
///
///
/// ```compile_fail
/// use type_identical::*;
///
/// fn bbb<T: ?Sized, U: ?Sized>()
/// where
///     Assertion: Assert<Type<T> = U>,
/// {
/// }
///
/// bbb::<i32, ()>();
/// ```
///
///
///
/// ```compile_fail
/// use type_identical::*;
///
/// fn bbb<T: ?Sized + Assert<Type<U> = T>, U: ?Sized>() {}
///
/// bbb::<i32, ()>();
/// ```
///
///
///
/// ```compile_fail
/// use type_identical::*;
///
/// struct Bar<T: ?Sized>(*const T);
///
/// fn bbb<T: ?Sized + Identical<Bar<U>>, U: ?Sized>() {}
///
/// bbb::<Bar<i32>, ()>();
/// ```
///
///
///
/// ```compile_fail
/// use type_identical::*;
///
/// struct Bar<T: ?Sized>(*const T);
///
/// fn bbb<T: ?Sized + Identical<Bar<U>>, U: ?Sized>() {}
///
/// bbb::<i32, ()>();
/// ```
#[allow(non_camel_case_types)]
struct _test_type_constrain;
