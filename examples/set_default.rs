use wasapi::{DeviceCollection, Direction, get_default_device, initialize_mta};
use wasapi::set_default::PolicyConfig;

fn main() {
    println!("Changing default audio output endpoint");
    initialize_mta().unwrap();
    let audio_jack = "FrontAudioJack (Realtek High Definition Audio)";
    let device = get_default_device(&Direction::Render).unwrap();
    let device_name = device.get_friendlyname().unwrap();
    if device_name.contains(audio_jack) {
        println!("Speakers are default device");
    } else {
        println!("Speakers are not default");
    }

    let speaker_id = DeviceCollection::new(&Direction::Render).unwrap().into_iter().find(|device| {
        let device = device.as_ref().unwrap();
        let device_name = device.get_friendlyname().unwrap();
        device_name.contains(audio_jack)
    }).unwrap().unwrap().get_id().unwrap();
    println!("Speaker id: {}", speaker_id);
    let p = PolicyConfig::new().unwrap();
    match p.set_default_endpoint(&*speaker_id) {
        Ok(_) => println!("Set default endpoint"),
        Err(e) => println!("Error setting default endpoint: {}", e),
    }
}

