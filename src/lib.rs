#![cfg_attr(feature = "nightly", feature(custom_attribute, plugin))]
#![cfg_attr(feature = "nightly", plugin(pnet_macros_plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]
// We can't implement Iterator since we use streaming iterators
#![cfg_attr(feature="clippy", allow(should_implement_trait))]

extern crate pnet;
extern crate pnet_macros;
extern crate pnet_macros_support;

pub mod arp;
