extern crate midir;

mod error;
mod onetesla;

#[cfg(test)]
mod tests {
    use onetesla::OneTesla;
    use std::error::Error;

    #[test]
    fn open_midi_device() {
        match OneTesla::new()
            .and_then(|t| t.volume(10)) {
            Ok(_) => (),
            Err(err) => println!("Error: {}", err.description())
        }
    }
}
