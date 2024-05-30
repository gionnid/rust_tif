import python_tif
import rasterio

def test_value():
    assert python_tif.get_mean_of_band_1("assets/image.tif") == 15810534


def test_read_band_1():
    array = python_tif.read_all_band_1("assets/image.tif")
    assert array.sum() == 15810534
    assert array.shape == (358, 179)

def test_read_band_1():
    array = python_tif.read_all_band_1("assets/image.tif")
    with rasterio.open("assets/image.tif") as ds:
        array_rasterio = ds.read(1)

    assert(array == array_rasterio).all()
