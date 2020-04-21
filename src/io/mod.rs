use crate::data::point::PointCollection;
use num_traits::Float;
use std::path::PathBuf;

pub mod csv;

trait PointReader<T>
where
    T: Float,
{
    type ErrorType;

    fn read(&self, file: &PathBuf) -> Result<PointCollection<T>, Self::ErrorType>;
}
