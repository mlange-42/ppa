//! Point data structures
use num_traits::Float;
use std::fmt;
use std::slice::{Chunks, ChunksMut};

/// Point collection creation result type
type Result<T> = std::result::Result<T, PointConstructionError>;

/// Collection of points with ID
#[derive(Debug)]
pub struct PointCollection<T>
where
    T: Float,
{
    points: Points<T>,
    ids: Option<Vec<String>>,
}

impl<T> PointCollection<T>
where
    T: Float,
{
    pub fn points(&self) -> &Points<T> {
        &self.points
    }
}

impl<T> PointCollection<T>
where
    T: Float,
{
    pub fn new(points: Points<T>, ids: Option<Vec<String>>) -> self::Result<Self> {
        if let Some(id) = &ids {
            if points.len() != id.len() {
                return Err(PointConstructionError(format!(
                    "Data length ({}) does not match number of IDs ({})",
                    points.len(),
                    id.len()
                )));
            }
        }
        Ok(PointCollection { points, ids })
    }
}

/// A collection of n-dimensional float points
#[derive(Debug)]
pub struct Points<T>
where
    T: Float,
{
    data: Vec<T>,
    dim: usize,
}

impl<T> Points<T>
where
    T: Float,
{
    /// Creates an empty point collection
    pub fn empty(dim: usize) -> Self {
        Points {
            data: Vec::new(),
            dim,
        }
    }
    pub fn from_raw(data: Vec<T>, dim: usize) -> self::Result<Self> {
        if data.len() % dim != 0 {
            return Err(PointConstructionError(format!(
                "Data length ({}) does not match number of dimensions ({})",
                data.len(),
                dim
            )));
        }
        Ok(Points { data, dim })
    }
    pub fn from_rows(data: &[Vec<T>]) -> self::Result<Self> {
        let dim = data[0].len();
        let mut data_raw = vec![T::zero(); dim * data.len()];
        for (i, row) in data.iter().enumerate() {
            if row.len() != dim {
                return Err(PointConstructionError(format!(
                    "Row length ({}) does not match number of dimensions ({})",
                    row.len(),
                    dim
                )));
            }
            let index = i * dim;
            for c in 0..dim {
                data_raw[index + c] = row[c];
            }
        }
        Ok(Points {
            data: data_raw,
            dim,
        })
    }
    pub fn from_cols(data: &[Vec<T>]) -> self::Result<Self> {
        let dim = data.len();
        let rows = data[0].len();
        for col in data {
            if col.len() != rows {
                return Err(PointConstructionError(format!(
                    "Column length ({}) does not match number of rows ({})",
                    col.len(),
                    rows
                )));
            }
        }
        let mut data_raw = vec![T::zero(); dim * rows];
        for (i, col) in data.iter().enumerate() {
            for (j, val) in col.iter().enumerate() {
                data_raw[j * dim + i] = *val;
            }
        }
        Ok(Points {
            data: data_raw,
            dim,
        })
    }
    /*fn index(&self, row: usize) -> usize {
        row * self.dim
    }*/
    pub fn len(&self) -> usize {
        self.data.len() / self.dim
    }
    pub fn dim(&self) -> usize {
        self.dim
    }
    pub fn push(&mut self, row: &[T]) {
        if row.len() != self.dim {
            panic!(format!(
                "Row length ({}) does not match number of dimensions ({})",
                row.len(),
                self.dim
            ))
        }
        self.data.extend_from_slice(row);
    }
    pub fn get(&self, index: usize) -> &[T] {
        let start = index * self.dim;
        &self.data[start..(start + self.dim)]
    }
    pub fn get_mut(&mut self, index: usize) -> &[T] {
        let start = index * self.dim;
        &mut self.data[start..(start + self.dim)]
    }
    pub fn iter(&self) -> Chunks<T> {
        self.data.chunks(self.dim)
    }
    pub fn iter_mut(&mut self) -> ChunksMut<T> {
        self.data.chunks_mut(self.dim)
    }
}

/// Error type for failed contruction of point collection.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PointConstructionError(String);

impl fmt::Display for PointConstructionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[cfg(test)]
mod test {
    use crate::data::point::Points;

    #[test]
    fn create_empty() {
        let points = Points::<f32>::empty(3);

        assert_eq!(points.len(), 0);
        assert_eq!(points.dim(), 3);
    }
    #[test]
    fn create_from_raw() {
        let data = (0..12).map(|i| i as f32).collect();
        let points = Points::from_raw(data, 3).unwrap();

        assert_eq!(points.len(), 4);
        assert_eq!(points.dim(), 3);
        assert_eq!(points.get(0), &[0.0, 1.0, 2.0]);
        assert_eq!(points.get(1), &[3.0, 4.0, 5.0]);
    }
    #[test]
    fn create_from_rows() {
        let rows = vec![
            vec![0.0, 1.0, 2.0],
            vec![3.0, 4.0, 5.0],
            vec![6.0, 7.0, 8.0],
            vec![9.0, 10.0, 11.0],
        ];
        let points = Points::from_rows(&rows).unwrap();

        assert_eq!(points.len(), 4);
        assert_eq!(points.dim(), 3);
        assert_eq!(points.get(0), &[0.0, 1.0, 2.0]);
        assert_eq!(points.get(1), &[3.0, 4.0, 5.0]);
    }
    #[test]
    fn create_from_cols() {
        let cols = vec![
            vec![0.0, 3.0, 6.0, 9.0],
            vec![1.0, 4.0, 7.0, 10.0],
            vec![2.0, 5.0, 8.0, 11.0],
        ];
        let points = Points::from_cols(&cols).unwrap();

        assert_eq!(points.len(), 4);
        assert_eq!(points.dim(), 3);
        assert_eq!(points.get(0), &[0.0, 1.0, 2.0]);
        assert_eq!(points.get(1), &[3.0, 4.0, 5.0]);
    }
}
