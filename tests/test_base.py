import python_tif
import rasterio


class raster1:
    sum = 15810534
    shape = (358, 179)
    path = "assets/image.tif"
    bbox = [
        1487158.8223163888,
        5064167.147572124,
        4774562.5348052485,
        1667183.3113336363,
    ]


def test_value():
    assert python_tif.get_mean_of_band_1(raster1.path) == raster1.sum


def test_read_band_1():
    array = python_tif.read_all_band_1(raster1.path)
    assert array.sum() == raster1.sum
    assert array.shape == raster1.shape


def test_get_bbox():
    bbox = python_tif.get_bbox(raster1.path)
    assert bbox == raster1.bbox
