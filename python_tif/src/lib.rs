use ndarray::{ IxDynImpl, ArrayBase, Array2, ViewRepr, Dim, * };
use numpy::PyArrayDyn;
use pyo3::prelude::{ pymodule, PyModule, PyResult, Python };
use rust_tif;

#[pymodule]
fn python_tif(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    #[pyfn(m)]
    fn get_mean_of_band_1(_py: Python<'_>, geotiff_path: &str) -> u64 {
        rust_tif::get_mean_of_band_1(geotiff_path)
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use rust_tif::get_mean_of_band_1;
    
    #[test]
    fn test_base() {
        let tested_mean: u64 = get_mean_of_band_1("../assets/image.tif");
        let ref_value: u64 = 15810534;

        assert!(tested_mean == ref_value);
    }

    #[test]
    #[should_panic]
    fn test_base_2() {
        let tested_mean: u64 = get_mean_of_band_1("wrong_path_1232.tif");
        let ref_value: u64 = 15810534;

        assert!(tested_mean == ref_value);
    }
}
