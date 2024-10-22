#[cfg(feature = "serde")]
mod serde;

#[cfg(feature = "nonmax")]
mod nonmax;

mod traits;
pub use traits::*;

mod impls;
pub use impls::*;

mod varint;
pub use varint::*;

#[cfg(test)]
mod tests;
