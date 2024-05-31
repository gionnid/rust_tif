use gdal::Dataset;
use gdal::raster::ResampleAlg;
use ndarray::{ ArrayBase, OwnedRepr, Dim };

pub fn get_mean_of_band_1(geotiff_path: &str) -> u64 {
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

pub fn read_all_band_1(geotiff_path: &str) -> ArrayBase<OwnedRepr<u8>, Dim<[usize; 2]>> {
    let dataset = Dataset::open(geotiff_path).unwrap();
    let band = dataset.rasterband(1).unwrap();

    let band_values = band.read_as_array::<u8>(
        (0, 0),
        band.size(),
        band.size(),
        Some(ResampleAlg::Bilinear)
    );

    band_values.unwrap()
}

pub fn get_bbox(geotiff_path: &str) -> [f64; 4] {
    let dataset = Dataset::open(geotiff_path).unwrap();
    let geo_transform: [f64; 6] = dataset.geo_transform().expect("Affine not found");

    let (width, height) = dataset.raster_size();

    let (min_x, min_y) = (geo_transform[0], geo_transform[3] + geo_transform[5] * (height as f64));
    let (max_x, max_y) = (geo_transform[0] + geo_transform[1] * (width as f64), geo_transform[3]);

    let bbox: [f64; 4] = [min_x, max_y, max_x, min_y];

    bbox
}

#[cfg(test)]
mod tests {
    use crate::{ get_mean_of_band_1, read_all_band_1, get_bbox };

    const PATH_RASTER_1: &str = "../assets/image.tif";
    const SHAPE_RASTER_1: [usize; 2] = [358, 179];
    const BBOX_RASTER_1: [f64; 4] = [
        1487158.8223163888, 5064167.147572124, 4774562.5348052485, 1667183.3113336363,
    ];
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

    #[test]
    fn test_bbox() {
        let bbox = get_bbox(PATH_RASTER_1);

        assert_eq!(bbox, BBOX_RASTER_1)
    }
}
