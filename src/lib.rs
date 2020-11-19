#[macro_use]
extern crate vst;
extern crate vst_gui;
extern crate lazy_static;

use std::sync::{Arc, Mutex};
use vst::buffer::AudioBuffer;
use vst::editor::Editor;
use vst::plugin::{Category, Plugin, Info};
use lazy_static::lazy_static;

pub mod load_html;

// Load the "index.html"
lazy_static! {
    static ref HTML: String = load_html::load_html();
}

struct OtorParameters {
    pub gain: f32,
    pub output: f32,
}

fn create_javascript_callback(
    otor_parameters: Arc<Mutex<OtorParameters>>) -> vst_gui::JavascriptCallback
{
    Box::new(move |message: String| {
        let mut tokens = message.split_whitespace();

        let command = tokens.next().unwrap_or("");
        let argument = tokens.next().unwrap_or("").parse::<f32>();

        let mut locked_params = otor_parameters.lock().unwrap();

        match command {
            "getGain" => {
                return locked_params.gain.to_string();
            },
            "setGain" => {
                if argument.is_ok() {
                    locked_params.gain = argument.unwrap();
                }
            },
            "getOutput" => {
                return locked_params.output.to_string();
            },
            "setOutput" => {
                if argument.is_ok() {
                    locked_params.output = argument.unwrap();
                }
            },
            _ => {}
        }

        String::new()
    })
}

struct Otor {
    // We access this object both from a UI thread and from an audio processing
    // thread.
    params: Arc<Mutex<OtorParameters>>,
}

impl Default for Otor {
    fn default() -> Otor {
        let otor_parameters = Arc::new(Mutex::new(
            OtorParameters {
                gain: 0.0, //dBFS
                output: 100.0, // range: 0.0 to 100.0
            }
        ));

        Otor {
            params: otor_parameters.clone(),
        }
    }
}

impl Plugin for Otor {
    fn get_info(&self) -> Info {
        Info {
            name: "OTOR".to_string(),
            vendor: "Ippee".to_string(),
            unique_id: 48205622,
            version: 1,
            inputs: 2,
            outputs: 2,
            parameters: 1,
            category: Category::Effect,
            ..Default::default()
        }
    }

    // Here is where the bulk of our audio processing code goes.
    fn process(&mut self, buffer: &mut AudioBuffer<f32>) {
        let locked_params = self.params.lock().unwrap();

        let gain = locked_params.gain;
        let output = locked_params.output;

        // Convert a decibel value to a linear value.
        let gain_linear = 10.0_f32.powf(gain / 20.0);

        // Destructure an audio buffer into input and output buffers.
        for (input_buffer, output_buffer) in buffer.zip() {

            for (input_sample, output_sample) in input_buffer.iter().zip(output_buffer) {
                // Amplify a signal.
                *output_sample = *input_sample * gain_linear;

                // Clip a signal by force.
                if *output_sample > 1.0 {
                    *output_sample = 1.0;
                }
                else if *output_sample < -1.0 {
                    *output_sample = -1.0
                }

                *output_sample *= output / 100.0;
            }
        }
    }

    fn get_editor(&mut self) -> Option<Box<dyn Editor>> {
        let gui = vst_gui::new_plugin_gui(
            HTML.to_string(),
            create_javascript_callback(self.params.clone()),
            Some((900, 300)));
        Some(Box::new(gui))
    }
}

plugin_main!(Otor);