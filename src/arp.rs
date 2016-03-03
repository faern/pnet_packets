#[cfg(feature = "with-syntex")]
include!(concat!(env!("OUT_DIR"), "/arp.rs"));

#[cfg(not(feature = "with-syntex"))]
include!("arp.rs.in");
