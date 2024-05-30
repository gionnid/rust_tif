use ndarray::{ IxDynImpl, ArrayBase, Array2, ViewRepr, Dim, * };
use numpy::{ PyArrayDyn, IntoPyArray, PyArray };
use pyo3::prelude::{ pymodule, PyModule, PyResult, Python, Bound };
use rust_tif;

#[pymodule]
fn python_tif(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    #[pyfn(m)]
    fn get_mean_of_band_1(_py: Python<'_>, geotiff_path: &str) -> u64 {
        rust_tif::get_mean_of_band_1(geotiff_path)
    }

    #[pyfn(m)]
    fn read_all_band_1<'a>(
        _py: Python<'a>,
        geotiff_path: &str
    ) -> Bound<'a, PyArray<u8, Dim<IxDynImpl>>> {
        let array: ArrayBase<OwnedRepr<u8>, Dim<[usize; 2]>> = rust_tif::read_all_band_1(
            geotiff_path
        );

        array.into_dyn().into_pyarray_bound(_py)
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use rust_tif::{ get_mean_of_band_1, read_all_band_1 };

    const PATH_RASTER_1: &str = "../assets/image.tif";
    const SHAPE_RASTER_1: [usize; 2] = [358, 179];
    const REF_RASTER_1_VALUE: u64 = 15810534;

    #[test]
    fn test_base() {
        let tested_mean: u64 = get_mean_of_band_1(PATH_RASTER_1);

        assert!(tested_mean == REF_RASTER_1_VALUE);
    }

    #[test]
    #[should_panic]
    fn test_base_2() {
        get_mean_of_band_1("wrong_path_1232.tif");
        assert!(false);
    }

    #[test]
    fn test_read_all_band_1() {
        let array = read_all_band_1(PATH_RASTER_1);
        assert_eq!(SHAPE_RASTER_1, array.shape());

        let mean = array.map(|&x| x as u64).sum();

        assert_eq!(mean, REF_RASTER_1_VALUE);
    }
}
