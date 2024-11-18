use log::{Metadata, Record};

pub mod configuration;
pub mod routes;
pub mod startup;
pub mod telemetry;


pub trait Log: Send + Sync {
  fn enabled(&self, metadata: &Metadata) -> bool;

  fn log(&self, record: &Record);

  fn flush(&self);
}