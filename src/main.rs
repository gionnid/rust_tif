use gdal::Dataset;
use gdal::raster::RasterBand;
use gdal::raster::ResampleAlg;

fn get_mean_of_band_1(geotiff_path: &str) -> u64 {
    let dataset = Dataset::open(geotiff_path).unwrap();
    let band = dataset.rasterband(1).unwrap();

    let band_values = band
        .read_as::<u8>((0, 0), band.size(), band.size(), Some(ResampleAlg::Bilinear))
        .unwrap();

    let mean: u64 = band_values.data
        .iter()
        .map(|&x| x as u64)
        .sum();

    mean
}

fn main() {}

#[cfg(test)]
mod tests {
    use crate::get_mean_of_band_1;

    #[test]
    fn test_base() {
        let tested_mean: u64 = get_mean_of_band_1("assets/image.tif");
        let ref_value: u64 = 15810534;

        assert!(tested_mean == ref_value);
    }

    #[test]
    #[should_panic]
    fn test_base_2() {
        let tested_mean: u64 = get_mean_of_band_1("wrong_path.tif");
        let ref_value: u64 = 15810534;

        assert!(tested_mean == ref_value);
    }
}
