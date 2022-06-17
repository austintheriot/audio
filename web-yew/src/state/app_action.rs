use crate::audio::stream_handle::StreamHandle;

#[derive(Debug, Clone)]
pub enum AppAction {
    SetBuffer(Vec<f32>),
    SetStreamHandle(Option<StreamHandle>)
}
