/*!
Agents execute provided Jobs.

In the past, this had been implemented with generics and an impl
for the basic component types. Now the Dynamic agent uses polymorphism
and trait objects to be able to use any components specified.
*/

mod api;
mod dynamic;

pub use api::*;
pub use dynamic::*;
