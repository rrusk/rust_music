use druid::widget::{Button, Flex, Label};
use druid::{AppLauncher, Widget, WindowDesc};
use rodio::Source;
//use rodio::{Decoder, OutputStream, Source};
//use std::fs::File;
//use std::io::BufReader;
//use std::thread;

fn main() {
    let main_window = WindowDesc::new(build_ui()).title("Music Player");
    AppLauncher::with_window(main_window)
        .log_to_console() // Log to console
        .launch(())
        .unwrap();
}

fn build_ui() -> impl Widget<()> {
    let label = Label::new("Music Player");
    let button = Button::new("Play").on_click(|_, _, _| {
        println!("Play button clicked");
        play_audio("example.mp3");
        println!("Playback triggered");
    });

    Flex::column()
        .with_child(label)
        .with_child(button)
}

fn play_audio(file_path: &str) {
    let file_path = file_path.to_string(); // Clone the file path into an owned String
    std::thread::spawn(move || {
        println!("Opening file: {}", file_path);

        // Create an audio output stream
        let (_stream, stream_handle) = match rodio::OutputStream::try_default() {
            Ok(output) => output,
            Err(e) => {
                eprintln!("Failed to create audio output stream: {}", e);
                return;
            }
        };
        println!("Audio output stream initialized");

        // Helper function to create a new `Decoder` from the file
        let create_decoder = || -> Option<rodio::Decoder<std::io::BufReader<std::fs::File>>> {
            let file = match std::fs::File::open(&file_path) {
                Ok(f) => std::io::BufReader::new(f),
                Err(e) => {
                    eprintln!("Failed to open file {}: {}", file_path, e);
                    return None;
                }
            };
            match rodio::Decoder::new(file) {
                Ok(s) => Some(s),
                Err(e) => {
                    eprintln!("Failed to decode MP3: {}", e);
                    None
                }
            }
        };

        // Create the initial decoder
        let source = match create_decoder() {
            Some(s) => s,
            None => return,
        };
        println!("MP3 file decoded");

        // Get the total duration of the audio, if available
        if let Some(duration) = source.total_duration() {
            println!("Audio duration: {:?}", duration);

            // Play the audio
            if let Err(e) = stream_handle.play_raw(source.convert_samples()) {
                eprintln!("Failed to play audio: {}", e);
                return;
            }
            println!("Playing audio...");

            // Sleep for the duration of the audio
            std::thread::sleep(duration);
        } else {
            // Fallback: Play until the end of the stream
            if let Some(source_chunk) = create_decoder() {
                if let Err(e) = stream_handle.play_raw(source_chunk.convert_samples()) {
                    eprintln!("Failed to play audio: {}", e);
                    return;
                }
                println!("Playing audio...");
            }
        }

        println!("Audio playback completed");
    });
}



