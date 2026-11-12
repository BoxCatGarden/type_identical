//! Tests of inferring types through assertions.

/// ```no_run
/// use type_identical::*;
///
/// fn bbb<T: ?Sized + Identical<U> + Identical<W>, U: ?Sized, W: ?Sized>() {}
///
/// bbb::<i32, _, _>();
/// bbb::<[i32], _, _>();
/// bbb::<dyn Send, _, _>();
/// ```
///
/// ```no_run
/// use type_identical::*;
///
/// fn bbb<T: ?Sized, U: ?Sized>()
/// where
///     Assertion: Assert<Type<T> = U>,
/// {
/// }
///
/// bbb::<i32, _>();
/// bbb::<[i32], _>();
/// bbb::<dyn Send, _>();
/// bbb::<_, i32>();
/// bbb::<_, [i32]>();
/// bbb::<_, dyn Send>();
/// ```
///
/// ```no_run
/// use type_identical::*;
///
/// fn bbb<T: ?Sized + Assert<Type<U> = T>, U: ?Sized>() {}
///
/// bbb::<i32, _>();
/// bbb::<[i32], _>();
/// bbb::<dyn Send, _>();
/// ```
///
/// ```no_run
/// use type_identical::*;
///
/// struct Bar<T: ?Sized>(*const T);
///
/// fn bbb<T: ?Sized + Identical<Bar<U>> + Identical<W>, U: ?Sized, W: ?Sized>() {}
///
/// bbb::<Bar<i32>, _, _>();
/// bbb::<Bar<i32>, i32, Bar<i32>>();
/// bbb::<Bar<[i32]>, _, _>();
/// bbb::<Bar<[i32]>, [i32], Bar<[i32]>>();
/// bbb::<Bar<dyn Send>, _, _>();
/// bbb::<Bar<dyn Send>, dyn Send, Bar<dyn Send>>();
/// ```
///
/// ```no_run
/// use type_identical::*;
///
/// struct Bar<T: ?Sized>(*const T);
///
/// fn bbb<T: ?Sized, U: ?Sized>()
/// where
///     Assertion: Assert<Type<T> = Bar<U>>,
/// {
/// }
///
/// bbb::<Bar<i32>, _>();
/// bbb::<Bar<[i32]>, _>();
/// bbb::<Bar<dyn Send>, _>();
/// bbb::<_, i32>();
/// bbb::<_, [i32]>();
/// bbb::<_, dyn Send>();
/// ```
#[allow(non_camel_case_types)]
struct _test_type_infer;
