use crate::errors::WaveError;

pub trait Decoder {
    fn decode(&mut self) -> Result<Vec<f32>, WaveError>;
    // TODO: Add functions like split_data_into_channels and so on, which later can be used for the playback library
}