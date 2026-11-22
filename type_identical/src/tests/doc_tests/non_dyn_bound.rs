//! Tests of [`Identical`], ensuring that [`Identical`] cannot
//! be trait or supertrait bounds in `dyn` types outside this
//! [`crate`].
//!
//! [`Identical`]: crate::Identical

/// ```compile_fail
/// use type_identical::*;
///
/// fn bbb<T: ?Sized>(_: &dyn Identical<T>) {}
/// ```
///
///
///
/// ```compile_fail
/// use type_identical::*;
///
/// fn bbb<T: ?Sized, U: ?Sized>(_: &dyn Identical<T, NonDynBound<U> = U>) {}
/// ```
///
///
///
/// ```no_run
/// use type_identical::*;
///
/// fn aaa<T: ?Sized + Identical<T>>() {}
///
/// aaa::<dyn Send>();
/// aaa::<[i32]>();
/// ```
#[allow(non_camel_case_types)]
struct _test_non_dyn_bound;
