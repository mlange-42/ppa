use crate::data::point::{PointCollection, PointConstructionError, Points};
use crate::io::PointReader;
use csv::{ReaderBuilder, StringRecord};
use num_traits::Float;
use std::path::PathBuf;
use std::{fmt, io};

/// PCsv result type
type CsvResult<T> = Result<T, CsvError>;

/// Csv file options
#[derive(Clone, Debug)]
pub struct CsvOptions {
    delimiter: u8,
    no_data: String,
}
impl Default for CsvOptions {
    fn default() -> Self {
        CsvOptions {
            delimiter: b';',
            no_data: "NA".to_string(),
        }
    }
}

pub struct CsvPointReader {
    columns: Vec<String>,
    id_column: Option<String>,
    options: CsvOptions,
}
impl CsvPointReader {
    pub fn new(columns: &[&str], id_column: Option<&str>, options: CsvOptions) -> Self {
        CsvPointReader {
            columns: columns.iter().map(|c| c.to_string()).collect(),
            id_column: id_column.and_then(|c| Some(c.to_string())),
            options,
        }
    }
    fn column_index(
        &self,
        header: &[&str],
        column: &str,
    ) -> std::result::Result<usize, ColumnIndexError> {
        header
            .iter()
            .position(|n| &column == n)
            .ok_or(ColumnIndexError(format!("Column {} not found.", column)))
    }
}

impl<T> PointReader<T> for CsvPointReader
where
    T: Float,
{
    type ErrorType = CsvError;

    fn read(&self, file: &PathBuf) -> CsvResult<PointCollection<T>> {
        let no_data = &self.options.no_data;

        // Read csv
        let mut reader = ReaderBuilder::new()
            .delimiter(self.options.delimiter)
            .from_path(file)?;
        let header: StringRecord = reader.headers()?.clone();
        let header: Vec<_> = header.iter().collect();

        let id_index = match &self.id_column {
            Some(col) => Some(self.column_index(&header[..], &col)?),
            None => None,
        };

        let mut col_indices = vec![];
        for col in &self.columns {
            col_indices.push(self.column_index(&header[..], col)?)
        }

        let mut ids = vec![];
        let mut data: Vec<T> = vec![];
        for record in reader.records() {
            let rec = record?;
            if let Some(id_idx) = id_index {
                ids.push(rec.get(id_idx).unwrap().to_string());
            }
            for col in &col_indices {
                let str = rec.get(*col).unwrap();
                let val = match T::from_str_radix(str, 10) {
                    Ok(v) => v,
                    Err(_e) => {
                        return Err(CsvError::ParseError(format!(
                            "Unable to parse value '{}' to float.",
                            str
                        )))
                    }
                };

                data.push(val);
            }
        }

        Ok(PointCollection::new(
            Points::from_raw(data, col_indices.len())?,
            if id_index.is_some() { Some(ids) } else { None },
        )?)
    }
}

#[derive(Debug)]
pub enum CsvError {
    ColumnError(ColumnIndexError),
    IoError(std::io::Error),
    CsvError(csv::Error),
    ParseError(String),
    PointError(PointConstructionError),
}
impl From<ColumnIndexError> for CsvError {
    fn from(err: ColumnIndexError) -> CsvError {
        CsvError::ColumnError(err)
    }
}
impl From<PointConstructionError> for CsvError {
    fn from(err: PointConstructionError) -> CsvError {
        CsvError::PointError(err)
    }
}
impl From<io::Error> for CsvError {
    fn from(err: io::Error) -> CsvError {
        CsvError::IoError(err)
    }
}
impl From<csv::Error> for CsvError {
    fn from(err: csv::Error) -> CsvError {
        CsvError::CsvError(err)
    }
}
impl fmt::Display for CsvError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.fmt(f)
    }
}

/// Error type for missing columns.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ColumnIndexError(String);

impl fmt::Display for ColumnIndexError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[cfg(test)]
mod test {
    use crate::data::point::PointCollection;
    use crate::io::csv::{CsvOptions, CsvPointReader};
    use crate::io::PointReader;
    use std::path::PathBuf;

    #[test]
    fn read_csv() {
        let path = PathBuf::from("test_data/test-25p.csv");
        let reader = CsvPointReader::new(&["X", "Y", "Z"], Some("ID"), CsvOptions::default());
        let points: PointCollection<f32> = reader.read(&path).unwrap();

        assert_eq!(points.points().len(), 25);
        assert_eq!(points.points().dim(), 3);
    }
}
