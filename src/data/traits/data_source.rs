//! Parent trait for all data sources.
//! Use to implement common functionality across different data providers.

use std::any::Any;

pub trait DataSource: Any + Send + Sync {
    fn as_any(&self) -> &dyn Any;
}
