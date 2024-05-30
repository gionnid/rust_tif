import python_tif


def test_value():
    assert python_tif.get_mean_of_band_1("assets/image.tif") == 15810534


def test_read_band_1():
    array = python_tif.read_all_band_1("assets/image.tif")
    assert array.sum() == 15810534
    assert array.shape == (358, 179)
