use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Sample, SampleFormat, SizedSample};
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{self, SyncSender};

const SAMPLE_WIDTH: usize = 128;

fn main() -> Result<(), anyhow::Error> {
    // Initialize audio host
    let host = cpal::default_host();
    host.output_devices()?.for_each(|device| {
        println!("Output device: {}", device.name().unwrap());
    });

    host.input_devices()?.for_each(|device| {
        println!("Input device: {}", device.name().unwrap());
    });
    
    // Get default devices
    let input_device = host.default_input_device()
        .expect("No input device available");
    let output_device = host.default_output_device()
        .expect("No output device available");

    // Get default configs
    let input_config = input_device.default_input_config()?;
    let output_config = output_device.default_output_config()?;

    println!("Input config: {:?}", input_config);
    println!("Output config: {:?}", output_config);

    // Create ring buffer for thread communication
    let (tx, rx) = mpsc::sync_channel::<[f32; SAMPLE_WIDTH]>(1024);

    // Create atomic bool for controlling streams
    let running = Arc::new(AtomicBool::new(true));
    let running_clone = running.clone();

    // Create input stream
    let input_stream = match input_config.sample_format() {
        SampleFormat::F32 => create_input_stream::<f32>(&input_device, &input_config.into(), tx)?,
        _ => return Err(anyhow::Error::msg("Unsupported input format")),
    };

    // Create output stream
    let output_stream = match output_config.sample_format() {
        SampleFormat::F32 => create_output_stream::<f32>(&output_device, &output_config.into(),  rx)?,
        _ => return Err(anyhow::Error::msg("Unsupported output format")),
    };

    // Start the streams
    input_stream.play()?;
    output_stream.play()?;

    println!("Recording and playing audio. Press Enter to stop...");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;

    // Stop streams
    running.store(false, Ordering::SeqCst);
    input_stream.pause()?;
    output_stream.pause()?;

    Ok(())
}

fn create_input_stream<T>(
    device: &cpal::Device,
    config: &cpal::StreamConfig,
    producer: SyncSender<[f32; SAMPLE_WIDTH]>,
) -> Result<cpal::Stream, anyhow::Error>
where
    T: SizedSample, f32: From<<T as Sample>::Float>, f32: From<T>
{
    let input_data_fn = move |data: &[T], _: &cpal::InputCallbackInfo| {
        // Convert samples to f32 and write to ring buffer
        let mut i = 0;
        let mut samples = [0.0; SAMPLE_WIDTH];
        for &sample in data.iter() {
            if i == SAMPLE_WIDTH  {
                producer.send(samples).ok();
                i = 0;
            }

            samples[i] = f32::from(sample);
            i += 1;
        }

        println!("Received {} samples", data.len());
    };

    device.build_input_stream(
        config,
        input_data_fn,
        |err| eprintln!("Error in input stream: {}", err),
        None,
    ).map_err(Into::into)
}

fn create_output_stream<T>(
    device: &cpal::Device,
    config: &cpal::StreamConfig,
    consumer: mpsc::Receiver<[f32; SAMPLE_WIDTH]>,
) -> Result<cpal::Stream, anyhow::Error>
where
    T: SizedSample + cpal::FromSample<f32>,
{
    let mux = Arc::new(Mutex::new(consumer));
    let mux_clone = mux.clone();
    let output_data_fn = move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
        // let consumer = consumer.clone();
        let mut i = 0;
        let mut samples = [0.0; SAMPLE_WIDTH];
        for sample in data.iter_mut() {
            if i == 0 {
                let consumer = mux_clone.lock().unwrap();
                samples = consumer.recv().unwrap();
                i = 0;
            }

            *sample = samples[i].to_sample::<T>();

            i += 1;
            i %= SAMPLE_WIDTH;
        }
    };


    device.build_output_stream(
        config,
        output_data_fn,
        |err| eprintln!("Error in output stream: {}", err),
        None,
    ).map_err(Into::into)
    

}