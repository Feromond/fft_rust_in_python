import fft_rust_in_python as frip
from PIL import Image
import io

# Read data
measured_data, time = frip.read_csv_py("sample_data/data2.csv")

# Compute FFT
fft_real, fft_imag = frip.compute_fft_py(measured_data)

# Compute Magnitude
fft_magnitude = frip.compute_magnitude_py(fft_real, fft_imag)

# Generate Frequencies
sampling_interval = time[1] - time[0]  # uniform sampling
frequencies = frip.generate_frequencies_py(len(measured_data), sampling_interval)

# Apply FFT Shift to Frequencies and Magnitudes
shifted_frequencies = frip.fft_shift_frequencies_py(frequencies)
shifted_real, shifted_imag = frip.fft_shift_py(fft_real, fft_imag)  # Shift FFT real and imag
shifted_magnitude = frip.compute_magnitude_py(shifted_real, shifted_imag)  # Recalculate magnitude after shift


plot_bytes_time_domain = frip.generate_plot_py(time, measured_data, "Time", "Measured Data", "Test Plot")
plot_bytes_freq_domain = frip.generate_plot_py(shifted_frequencies, shifted_magnitude, "Frequency", "Magnitude", "Test Plot2")
image = Image.open(io.BytesIO(bytearray(plot_bytes_time_domain)))
image2 = Image.open(io.BytesIO(bytearray(plot_bytes_freq_domain)))

image.show()
image2.show()
