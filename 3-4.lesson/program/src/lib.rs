pub mod instruction;
pub mod processor;
pub mod state;

// register all the modules you created
// and allow you to publish it
#[cfg(not(feature = "no-entrypoint"))]
pub mod entrypoint;
