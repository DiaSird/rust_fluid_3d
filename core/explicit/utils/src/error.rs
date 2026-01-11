use std::path::PathBuf;

#[derive(Debug, snafu::Snafu)]
#[snafu(visibility(pub))]
pub enum SimError {
    /// Failed: model config.
    FailedModelConfig,

    /// Failed to read air space data.
    FailedReadAirSpaceData,

    /// Exceeded the maximum number: {n} < {max_n}
    ExceededMaxNumber { n: usize, max_n: usize },

    /// Total pair particle is zero.
    ZeroParticleNumber,

    /// Failed: Nan value is detected: v{i}.
    CannotContinueSimNan { i: usize },

    /// None value is detected: x{i}
    FailedUpdateLocation { i: usize },

    /// Failed to write file.
    #[snafu(display("Failed to write file: {}", path.display()))]
    FailedWriteFile {
        source: std::io::Error,
        path: PathBuf,
    },

    /// Failed to read file.
    #[snafu(display("Failed to read file: {}", path.display()))]
    FailedReadFile {
        source: std::io::Error,
        path: PathBuf,
    },

    /// Failed to read file.
    #[snafu(display("Failed to read file: {}", path.display()))]
    FailedFeatureReadFile { source: csv::Error, path: PathBuf },

    /// Failed to read postcard file.
    #[snafu(display("Failed to read postcard file: {}", path.display()))]
    PostcardError {
        source: postcard::Error,
        path: PathBuf,
    },

    /// Failed: conservative smoothing.
    FailedConservativeSmoothing,
}

/// # Errors
pub const fn check_nan_to_error(i: usize, value: f64) -> Result<(), SimError> {
    if value.is_nan() {
        return Err(SimError::CannotContinueSimNan { i });
    }
    Ok(())
}

/// # Errors
pub fn check_nan_matrix3_to_error(i: usize, value: nalgebra::Matrix3<f64>) -> Result<(), SimError> {
    for row in 0..3 {
        for col in 0..3 {
            if value[(row, col)].is_nan() {
                return Err(SimError::CannotContinueSimNan { i });
            }
        }
    }
    Ok(())
}
