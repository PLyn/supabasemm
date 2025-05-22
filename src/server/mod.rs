#[cfg(feature = "ssr")]
pub mod api;
#[cfg(feature = "ssr")]
pub mod handlers;
#[cfg(feature = "ssr")]
pub mod server_init;
#[cfg(feature = "ssr")]
pub mod server_models;

#[cfg(feature = "ssr")]
pub use server_init::server_init;
