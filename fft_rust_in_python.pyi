def read_csv_py(file_path: str) -> tuple[list[float], list[float]]:
    """Reads a CSV file and extracts the first two columns (time, measured_data) as separate vectors.
    This is a very specific function to these examples and is not intended to be a robust data loader.

    Parameters:
        file_path (str): Path to the file that should be loaded.

    Returns:
        tuple[list[float], list[float]]: Returns a list of floats for both the time and measured data columns.
    """

def compute_fft_py(data: list[float]) -> tuple[list[float], list[float]]:
    """Computes the FFT of the provided data and returns separate vectors for the real and imaginary parts.

    Parameters:
        data (list[float]): The input data for which the FFT should be computed.

    Returns:
        tuple[list[float], list[float]]: A tuple containing the real and imaginary parts of the FFT result.
    """

def fft_shift_py(real: list[float], imag: list[float]) -> tuple[list[float], list[float]]:
    """Performs FFT shift on the real and imaginary parts, moving the zero-frequency component to the center.

    Parameters:
        real (list[float]): The real part of the FFT data.
        imag (list[float]): The imaginary part of the FFT data.

    Returns:
        tuple[list[float], list[float]]: The FFT-shifted real and imaginary parts.
    """

def compute_magnitude_py(real: list[float], imag: list[float]) -> list[float]:
    """Computes the magnitude of complex data (real and imaginary parts).

    Parameters:
        real (list[float]): The real part of the data.
        imag (list[float]): The imaginary part of the data.

    Returns:
        list[float]: The magnitudes computed from the real and imaginary parts.
    """

def generate_frequencies_py(len: int, sampling_interval: float) -> list[float]:
    """Generates frequency bins for FFT data based on the data length and sampling interval.

    Parameters:
        len (int): The length of the data.
        sampling_interval (float): The sampling interval of the data.

    Returns:
        list[float]: A list of frequency bins.
    """

def fft_shift_frequencies_py(data: list[float]) -> list[float]:
    """Shifts the zero-frequency component of the frequency bins to the center.

    Parameters:
        data (list[float]): The frequency data to be shifted.

    Returns:
        list[float]: The FFT-shifted frequency data.
    """

def generate_plot_py(
    x: list[float],
    y: list[float],
    x_label: str,
    y_label: str,
    title: str,
) -> bytes:
    """Generates a plot from the provided data and returns the plot as a byte array.

    Parameters:
        x (list[float]): The data for the x-axis.
        y (list[float]): The data for the y-axis.
        x_label (str): The label for the x-axis.
        y_label (str): The label for the y-axis.
        title (str): The title of the plot.

    Returns:
        bytes: The plot rendered as a PNG image in byte array format.
    """
