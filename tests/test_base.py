import python_tif

def test_value():
    assert python_tif.get_mean_of_band_1("assets/image.tif") == 15810534