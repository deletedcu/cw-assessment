pub mod contract;
mod error;
mod state;
mod msg;


pub use state::State;

#[cfg(target_arch="wasm32")]
cosmwasm_std::create_entry_points!(contract);