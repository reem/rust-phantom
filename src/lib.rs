#![deny(missing_docs)]
#![deny(warnings)]

//! Exposes `Phantom`, which is useful for hinting type parameters.

/// Can be used to hint types in places where it is necessary.
///
/// Takes the place of `None::<T>` but is cleaner and more direct.
pub struct Phantom<T>;

