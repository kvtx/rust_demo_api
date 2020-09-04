mod db;
pub mod demolition;
pub mod models;
pub mod response_handler;
pub mod routes;
pub mod schema;
#[cfg(test)]
mod tests;
#[macro_use]
extern crate diesel;
extern crate dotenv;
