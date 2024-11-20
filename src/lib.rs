use pyo3::prelude::*;
use csv::ReaderBuilder;
use std::error::Error;
use rustfft::{FftPlanner, num_complex::Complex};
use plotters::prelude::*;
use plotters_bitmap::BitMapBackend;
use image::codecs::png::PngEncoder;
use image::ImageEncoder;

/// Reads a CSV file and extracts the first two columns (time, measured_data) as separate vectors.
///     This is a very specific funciton to these examples and is not intended to be a robust data loader
pub fn read_csv(file_path: &str) -> Result<(Vec<f64>, Vec<f64>), Box<dyn Error>> {
    let mut reader = ReaderBuilder::new()
        .has_headers(true)  // Assumes the first row is a header
        .from_path(file_path)?;

    let mut time = Vec::new();
    let mut measured_data = Vec::new();
    for result in reader.records() {
        let record = result?;
        if record.len() >= 2 {
            time.push(record[0].trim().parse()?);
            measured_data.push(record[1].trim().parse()?);
        }
    }

    Ok((time, measured_data))
}

/// Computes the FFT of the provided data (measured_data) and returns separate vectors for real and imaginary parts.
pub fn compute_fft(data: Vec<f64>) -> Result<(Vec<f64>, Vec<f64>), Box<dyn Error>> {
    let mut planner = FftPlanner::<f64>::new();
    let fft = planner.plan_fft_forward(data.len());
    // Convert the input data to Complex numbers
    let mut buffer: Vec<Complex<f64>> = data
        .into_iter()
        .map(|x| Complex::new(x, 0.0))
        .collect();
    // Perform the FFT
    fft.process(&mut buffer);
    // Separate the real and imaginary parts
    let real: Vec<f64> = buffer.iter().map(|c| c.re).collect();
    let imag: Vec<f64> = buffer.iter().map(|c| c.im).collect();

    Ok((real, imag))
}


/// Performs FFT shift on the real and imaginary parts.
pub fn fft_shift(real: Vec<f64>, imag: Vec<f64>) -> Result<(Vec<f64>, Vec<f64>), Box<dyn Error>> {
    let len = real.len();
    let half = len / 2;

    // Rearrange the real and imaginary parts
    let mut shifted_real = real;
    let mut shifted_imag = imag;

    shifted_real.rotate_left(half);
    shifted_imag.rotate_left(half);

    Ok((shifted_real, shifted_imag))
}

/// Computes the magnitude of complex data (real and imaginary parts).
pub fn compute_magnitude(real: Vec<f64>, imag: Vec<f64>) -> Result<Vec<f64>, Box<dyn Error>> {
    if real.len() != imag.len() {
        return Err("Real and imaginary parts must have the same length.".into());
    }
    let magnitude: Vec<f64> = real
        .iter()
        .zip(imag.iter())
        .map(|(re, im)| (re.powi(2) + im.powi(2)).sqrt())
        .collect();
    Ok(magnitude)
}

/// Generates frequency bins for FFT data.
pub fn generate_frequencies(len: usize, sampling_interval: f64) -> Result<Vec<f64>, Box<dyn Error>> {
    if len == 0 || sampling_interval <= 0.0 {
        return Err("Length must be positive and sampling interval must be greater than zero.".into());
    }

    let total_duration = len as f64 * sampling_interval;
    let freq: Vec<f64> = (0..len)
        .map(|k| {
            if k < len / 2 {
                k as f64 / total_duration
            } else {
                -(len as f64 - k as f64) / total_duration
            }
        })
        .collect();

    Ok(freq)
}

/// Shifts the zero-frequency component of the frequency bins to the center.
pub fn fft_shift_frequencies(data: Vec<f64>) -> Result<Vec<f64>, Box<dyn Error>> {
    let len = data.len();
    let half = len / 2;

    let mut shifted_data = data.clone();
    shifted_data.rotate_left(half);

    Ok(shifted_data)
}


/// Generates a plot
pub fn generate_plot(
    data: Vec<(f64, f64)>,
    x_label: &str,
    y_label: &str,
    title: &str,
) -> Result<Vec<u8>, Box<dyn Error>> {
    let width = 1024;
    let height = 768;

    let mut buffer: Vec<u8> = vec![0; (width * height * 3) as usize];
    {
        // This block ensures `root_area` goes out of scope before we return `buffer`
        let root_area =
            BitMapBackend::with_buffer(&mut buffer, (width, height)).into_drawing_area();
        root_area.fill(&WHITE)?;

        let max_x = data.iter().map(|(x, _)| *x).fold(f64::MIN, f64::max);
        let min_x = data.iter().map(|(x, _)| *x).fold(f64::MAX, f64::min);
        let max_y = data.iter().map(|(_, y)| *y).fold(f64::MIN, f64::max);
        let min_y = data.iter().map(|(_, y)| *y).fold(f64::MAX, f64::min);

        let mut chart = ChartBuilder::on(&root_area)
            .caption(title, ("sans-serif", 30))
            .margin(10)
            .x_label_area_size(40)
            .y_label_area_size(40)
            .build_cartesian_2d(min_x..max_x, min_y..max_y)?;

        chart
            .configure_mesh()
            .x_desc(x_label)
            .y_desc(y_label)
            .draw()?;

        chart.draw_series(LineSeries::new(data.into_iter(), &RED))?;

        root_area.present()?;
    }

    // Encode the buffer into PNG format so it will be a known format by the image display
    let mut png_buffer = Vec::new();
    let encoder = PngEncoder::new(&mut png_buffer);
    encoder.write_image(
        &buffer,
        width,
        height,
        image::ExtendedColorType::Rgb8,
    )?;

    Ok(png_buffer)
}


#[pyfunction]
fn read_csv_py(file_path: String) -> PyResult<(Vec<f64>, Vec<f64>)> {
    read_csv(&file_path).map_err(|e| pyo3::exceptions::PyIOError::new_err(e.to_string()))
}

#[pyfunction]
fn compute_fft_py(data: Vec<f64>) -> PyResult<(Vec<f64>, Vec<f64>)> {
    compute_fft(data).map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))
}

#[pyfunction]
fn fft_shift_py(real: Vec<f64>, imag: Vec<f64>) -> PyResult<(Vec<f64>, Vec<f64>)> {
    fft_shift(real, imag).map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))
}

#[pyfunction]
fn compute_magnitude_py(real: Vec<f64>, imag: Vec<f64>) -> PyResult<Vec<f64>> {
    compute_magnitude(real, imag).map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))
}

#[pyfunction]
fn generate_frequencies_py(len: usize, sampling_interval: f64) -> PyResult<Vec<f64>> {
    generate_frequencies(len, sampling_interval)
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))
}

#[pyfunction]
fn fft_shift_frequencies_py(data: Vec<f64>) -> PyResult<Vec<f64>> {
    fft_shift_frequencies(data).map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))
}

#[pyfunction]
fn generate_plot_py(
    x: Vec<f64>,
    y: Vec<f64>,
    x_label: String,
    y_label: String,
    title: String,
) -> PyResult<Vec<u8>> {
    let data: Vec<(f64, f64)> = x.into_iter().zip(y.into_iter()).collect();
    generate_plot(data, &x_label, &y_label, &title)
        .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))
}


/// A Python module implemented in Rust.
#[pymodule]
fn fft_rust_in_python(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(read_csv_py, m)?)?;
    m.add_function(wrap_pyfunction!(compute_fft_py, m)?)?;
    m.add_function(wrap_pyfunction!(fft_shift_py, m)?)?;
    m.add_function(wrap_pyfunction!(compute_magnitude_py, m)?)?;
    m.add_function(wrap_pyfunction!(generate_frequencies_py, m)?)?;
    m.add_function(wrap_pyfunction!(fft_shift_frequencies_py, m)?)?;
    m.add_function(wrap_pyfunction!(generate_plot_py, m)?)?;

    Ok(())
}



