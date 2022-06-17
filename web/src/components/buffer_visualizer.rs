use crate::state::app_context::{AppContext, AppContextError};
use std::ops::Mul;
use yew::{function_component, html, prelude::*};

const NUM_AUDIO_DATA_POINTS: usize = 200;

#[function_component(BufferVisualizer)]
pub fn buffer_visualizer() -> Html {
    let app_context = use_context::<AppContext>().expect(AppContextError::NOT_FOUND);
    let audio_bar_width = format!("{:.1}", (100.0 / (NUM_AUDIO_DATA_POINTS as f32)));    
    
    // empty buffer
    if app_context.state_handle.buffer.is_empty() {
        return html! {
            <div class="buffer-visualizer">
                {(0..NUM_AUDIO_DATA_POINTS).into_iter().map(|_| {
                    html!{
                        <div 
                            class="buffer-visualizer__audio-bar" 
                            style={format!("height: 1px; width: {}%;", audio_bar_width)}
                        />
                    }
                }).collect::<Html>()}
            </div>
        };
    }

    // buffer has audio data: display it
    let iteration_group_size = app_context.state_handle.buffer.len() / NUM_AUDIO_DATA_POINTS;
    let sample_averages: Vec<String> = app_context
        .state_handle
        .buffer
        .chunks(iteration_group_size)
        .map(|samples| {
            let sum = samples.iter().map(|sample| sample.abs()).sum::<f32>();
            let average_percent = (sum / samples.len() as f32).mul(100.0).min(100.0).max(0.0);
            let formatted_percent = format!("{:.1}", average_percent);
            formatted_percent
        })
        .collect();

    html! {
        <div class="buffer-visualizer">
            {sample_averages.iter().map(|percent_string| {
                html!{
                    <div 
                        class="buffer-visualizer__audio-bar" 
                        style={format!("height: {}%; width: {}%;", percent_string, audio_bar_width)} 
                    />
                }
            }).collect::<Html>()}
        </div>
    }
}