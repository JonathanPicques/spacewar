use bevy::prelude::*;
use bevy::window::WindowFocused;

pub trait StopSoundInBackgroundAppExt {
    fn stop_sounds_in_background(&mut self) -> &mut Self;
}

impl StopSoundInBackgroundAppExt for App {
    fn stop_sounds_in_background(&mut self) -> &mut Self {
        self.add_systems(Update, stop_sounds_in_background_events)
    }
}

fn stop_sounds_in_background_events(mut events: EventReader<WindowFocused>, mut audio_sinks: Query<&mut AudioSink>) {
    for event in events.read() {
        for mut audio_sink in audio_sinks.iter_mut() {
            if event.focused {
                audio_sink.unmute();
            } else {
                audio_sink.mute();
            }
        }
    }
}
