/*!
This crate implements a basic extractor using CSS selectors.

It uses the [scraper] library.
!*/
// TODO: why are the docs rendering differently here?

mod scraper;
pub use self::scraper::*;

#[cfg(test)]
mod scraper_test;