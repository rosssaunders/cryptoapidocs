pub mod rest_private;
pub mod rest_public;
pub mod websocket_private;
pub mod websocket_public;

// Ensure processors are included in the binary
pub use rest_private::*;
pub use rest_public::*;
pub use websocket_private::*;
pub use websocket_public::*;
