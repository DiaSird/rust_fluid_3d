#[derive(Debug, snafu::Snafu)]
pub enum SimError {
    /// Failed: model config.
    FailedModelConfig,

    /// Failed to read air space data.
    FailedReadAirSpaceData,

    /// Exceeded the maximum number of particles. {n}
    ExceededMaxParticles { n: usize },

    /// Failed: updating velocity.
    FailedUpdateVelocity,

    /// None value is detected: x{i}
    FailedUpdateLocation { i: usize },
}
