import python_tif
import rasterio

class raster1:
    sum = 15810534
    shape = (358, 179)
    path = "assets/image.tif"

def test_value():
    assert python_tif.get_mean_of_band_1(raster1.path) == raster1.sum


def test_read_band_1():
    array = python_tif.read_all_band_1(raster1.path)
    assert array.sum() == raster1.sum
    assert array.shape == raster1.shape

def test_read_band_1():
    array = python_tif.read_all_band_1(raster1.path)
    with rasterio.open(raster1.path) as ds:
        array_rasterio = ds.read(1)

    assert(array == array_rasterio).all()
