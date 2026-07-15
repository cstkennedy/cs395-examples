use std::path::PathBuf;

use itertools::{Itertools, MinMaxResult};
use ordered_float::OrderedFloat;

use pyo3::prelude::*;
use rayon::prelude::*;

use crate::factory::ShapeWrapper;
use crate::parser::ShapeParser;

#[pyclass(from_py_object)]
#[derive(Clone, Copy)]
pub enum CompareBy {
    Name,
    Perimeter,
    Area,
}

#[pyclass(skip_from_py_object)]
pub struct ShapeCollection {
    shapes: Vec<ShapeWrapper>,
}

#[pymethods]
impl ShapeCollection {
    #[staticmethod]
    pub fn read_from_file(filename: PathBuf) -> PyResult<Self> {
        Ok(Self {
            shapes: ShapeParser::read_shapes(filename)?,
        })
    }

    /// Get a copy of the "smallest" element using a specified attribute.
    ///
    /// # Raises
    ///
    /// ValueError is ShapeCollection is empty
    ///
    pub fn min(&self, attribute: CompareBy) -> PyResult<ShapeWrapper> {
        if self.shapes.len() == 0 {
            return Err(pyo3::exceptions::PyValueError::new_err(
                "ShapeCollection is empty",
            ));
        }

        let smallest = match attribute {
            CompareBy::Name => self.shapes.par_iter().min_by_key(|shp| shp.name()),
            CompareBy::Perimeter => self
                .shapes
                .par_iter()
                .min_by_key(|shp| OrderedFloat(shp.perimeter())),
            CompareBy::Area => self
                .shapes
                .par_iter()
                .min_by_key(|shp| OrderedFloat(shp.area())),
        };

        Ok(smallest
            .ok_or(pyo3::exceptions::PyValueError::new_err(
                "Min could not be found",
            ))?
            .clone())
    }

    /// Get a copy of the "largest" element using a specified attribute.
    ///
    /// # Raises
    ///
    /// ValueError is ShapeCollection is empty
    ///
    pub fn max(&self, attribute: CompareBy) -> PyResult<ShapeWrapper> {
        if self.shapes.len() == 0 {
            return Err(pyo3::exceptions::PyValueError::new_err(
                "ShapeCollection is empty",
            ));
        }

        let smallest = match attribute {
            CompareBy::Name => self.shapes.par_iter().max_by_key(|shp| shp.name()),
            CompareBy::Perimeter => self
                .shapes
                .par_iter()
                .max_by_key(|shp| OrderedFloat(shp.perimeter())),
            CompareBy::Area => self
                .shapes
                .par_iter()
                .max_by_key(|shp| OrderedFloat(shp.area())),
        };

        Ok(smallest
            .ok_or(pyo3::exceptions::PyValueError::new_err(
                "Max could not be found",
            ))?
            .clone())
    }

    pub fn minmax(
        &self,
        min_attribute: CompareBy,
        max_attribute: CompareBy,
    ) -> PyResult<(ShapeWrapper, ShapeWrapper)> {
        let (smallest_shape, largest_shape) = rayon::join(
            || self.min(min_attribute).unwrap(),
            || self.max(max_attribute).unwrap()
        );

        Ok((smallest_shape, largest_shape))
    }

    /// Sort the ShapeCollection in place
    pub fn sort(&mut self, attribute: CompareBy) {
        match attribute {
            // TODO: rework &str to String allocation/conversion (DONE)
            CompareBy::Name => self.shapes.par_sort_by(|lhs, rhs| lhs.name().cmp(rhs.name())),
            CompareBy::Perimeter => self.shapes.par_sort_by_key(|shp| OrderedFloat(shp.perimeter())),
            CompareBy::Area => self.shapes.par_sort_by_key(|shp| OrderedFloat(shp.area())),
        };
    }

    pub fn __str__(&self) -> String {
        self.shapes.iter().map(ShapeWrapper::__str__).join("\n\n")
    }

    pub fn to_name_string(&self) -> String {
        self.shapes.iter().map(ShapeWrapper::name).join("\n")
    }

    pub fn print(&self) {
        // Imports should be module/file level
        use std::io::stdout;
        use std::io::BufWriter;
        use std::io::Write;

        let locked_stdout = stdout().lock();
        let mut writer = BufWriter::new(locked_stdout);

        for shape in self.shapes.iter().map(ShapeWrapper::__str__) {
            // todo... implement Display for ShapeWrapper
            writeln!(writer, "{shape}");
            writeln!(writer);
        }
    }

    pub fn print_names(&self) {
        // Imports should be module/file level
        use std::io::stdout;
        use std::io::BufWriter;
        use std::io::Write;

        let locked_stdout = stdout().lock();
        let mut writer = BufWriter::new(locked_stdout);

        for name in self.shapes.iter().map(ShapeWrapper::name) {
            writeln!(writer, "{name}");
        }
    }

    // Todo: Add a __iter__
    //   Requires:
    //     - Py - ref counter into Python GC
    //     - Understanding of RC (Arc), RefCell
}
