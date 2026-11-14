//! Tests of inferring types through assertions.

/// ```no_run
/// use type_identical::*;
///
/// fn bbb<T: ?Sized + Identical<U> + Identical<W>, U: ?Sized, W: ?Sized>() {}
///
/// bbb::<i32, _, _>();
/// bbb::<[i32], _, _>();
/// bbb::<dyn Send, _, _>();
/// fn kkk<A: ?Sized, B>() {
///     bbb::<A, _, _>();
///     bbb::<B, _, _>();
/// }
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
/// fn kkk<A: ?Sized, B>() {
///     bbb::<A, _>();
///     bbb::<_, A>();
///     bbb::<B, _>();
///     bbb::<_, B>();
/// }
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
/// fn kkk<A: ?Sized, B>() {
///     bbb::<A, _>();
///     bbb::<B, _>();
/// }
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
/// fn kkk<A: ?Sized, B>() {
///     bbb::<Bar<A>, _, _>();
///     bbb::<Bar<A>, A, Bar<A>>();
///     bbb::<Bar<B>, _, _>();
///     bbb::<Bar<B>, B, Bar<B>>();
/// }
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
/// fn kkk<A: ?Sized, B>() {
///     bbb::<Bar<A>, _>();
///     bbb::<_, A>();
///     bbb::<Bar<B>, _>();
///     bbb::<_, B>();
/// }
/// ```
///
/// ```no_run
/// use type_identical::*;
///
/// fn bbb<T, U, W>()
/// where
///     T: ?Sized + Identical<U>,
///     U: ?Sized + Identical<W>,
///     W: ?Sized + Identical<T>,
/// {
/// }
///
/// bbb::<i32, _, _>();
/// bbb::<[i32], _, _>();
/// bbb::<dyn Send, _, _>();
/// bbb::<_, i32, _>();
/// bbb::<_, [i32], _>();
/// bbb::<_, dyn Send, _>();
/// bbb::<_, _, i32>();
/// bbb::<_, _, [i32]>();
/// bbb::<_, _, dyn Send>();
/// fn kkk<A: ?Sized, B>() {
///     bbb::<A, _, _>();
///     bbb::<B, _, _>();
///     bbb::<_, A, _>();
///     bbb::<_, B, _>();
///     bbb::<_, _, A>();
///     bbb::<_, _, B>();
/// }
/// ```
///
/// ```no_run
/// use type_identical::*;
///
/// fn bbb<T: ?Sized, U: ?Sized, W: ?Sized>()
/// where
///     Assertion: Assert<Type<T> = U>,
///     Assertion: Assert<Type<T> = W>,
/// {
/// }
///
/// bbb::<i32, _, _>();
/// bbb::<[i32], _, _>();
/// bbb::<dyn Send, _, _>();
/// bbb::<_, i32, _>();
/// bbb::<_, [i32], _>();
/// bbb::<_, dyn Send, _>();
/// bbb::<_, _, i32>();
/// bbb::<_, _, [i32]>();
/// bbb::<_, _, dyn Send>();
/// fn kkk<A: ?Sized, B>() {
///     bbb::<A, _, _>();
///     bbb::<B, _, _>();
///     bbb::<_, A, _>();
///     bbb::<_, B, _>();
///     bbb::<_, _, A>();
///     bbb::<_, _, B>();
/// }
/// ```
#[allow(non_camel_case_types)]
struct _test_type_infer;
