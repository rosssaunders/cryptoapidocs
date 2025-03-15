pub mod rest_private;
pub mod rest_public;

// Ensure processors are included in the binary
pub use rest_private::PrivateREST;
pub use rest_public::PublicREST;