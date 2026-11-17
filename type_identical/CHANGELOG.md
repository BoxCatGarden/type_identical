# CHANGELOG

Format: [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).
Versioning: [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [2.0.0] - 2026-11-17

### Breaking Changes

- `Identical` is separated from `Assert`, and the blanket implementation
  of `Identical` is also changed. This change mainly affects nested
  function calls.

    - `Identical` does not introduce `Assert`, and `Assert` does not introduce
      `Identical`. e.g., the following code cannot compile now.

      ```rust, compile_fail
      use type_identical::*;
      
      fn aaa<T: Identical<U>, U>() {
          bbb::<T, U>();
      }
      
      fn bbb<T: Assert<Type<U> = T>, U>() {
          aaa::<T, U>();
      }
      ```

    - `T: Identical<T>` always holds for `T`. e.g., the following
      code cannot compile previously, and can compile now.

      ```rust, no_run
      use type_identical::*;
      
      fn aaa<U, W, T: Identical<U> + Identical<W>>() {
          aaa::<T, T, T>();
      }
      ```
