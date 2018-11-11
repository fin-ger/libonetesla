use midir::{MidiOutput, MidiOutputConnection};
use error::OneTeslaError;

type OneTeslaResult = Result<OneTesla, OneTeslaError>;

const NOTE_ON_MSG: u8 = 0x90;
const NOTE_OFF_MSG: u8 = 0x80;
const MSG_SYS: u8 = 0xF0;
const MSG_RLT: u8 = 0x7F;
const MSG_CALL: u8 = 0x7F;
const MSG_ID_DEV: u8 = 0x04;
const MSG_ID_VOL: u8 = 0x01;
const MSG_EOX: u8 = 0xF7;

pub struct OneTesla {
    connection: MidiOutputConnection,
    note: u8,
    is_on: bool,
}

impl OneTesla {
    pub fn new() -> OneTeslaResult {
        let output = MidiOutput::new("oneTesla output")
            .map_err(|_| OneTeslaError::SystemHasNoMidiSupport)?;
        let mut port = 0;
        let mut found = false;

        for i in 0..output.port_count() {
            output.port_name(i).and_then(|name| {
                if name.as_str().contains("oneTesla") {
                    port = i;
                    found = true;
                }
                Ok(())
            });
        }

        if !found {
            return Err(OneTeslaError::NoCompatibleDeviceFound);
        }

        let connection = output.connect(port, "oneTesla")
            .map_err(|_| OneTeslaError::CannotConnectToDevice)?;

        Ok(OneTesla {
            connection: connection,
            note: 65,
            is_on: false,
        })
    }

    pub fn on(mut self) -> OneTeslaResult {
        let msg: [u8; 3] = [NOTE_ON_MSG, self.note, 0x64];

        self.is_on = true;

        self.connection
            .send(&msg)
            .map_err(|_| OneTeslaError::FailedToSendMessageToDevice)?;

        Ok(self)
    }

    pub fn off(mut self) -> OneTeslaResult {
        let msg: [u8; 3] = [NOTE_OFF_MSG, self.note, 0x64];

        self.is_on = false;

        self.connection
            .send(&msg)
            .map_err(|_| OneTeslaError::FailedToSendMessageToDevice)?;

        Ok(self)
    }

    pub fn volume(mut self, volume: u8) -> OneTeslaResult {
        let msg: [u8; 8] = [MSG_SYS, MSG_RLT, MSG_CALL, MSG_ID_DEV, MSG_ID_VOL, volume, volume, MSG_EOX];

        self.connection
            .send(&msg)
            .map_err(|_| OneTeslaError::FailedToSendMessageToDevice)?;

        Ok(self)
    }

    pub fn note(mut self, note: u8) -> OneTeslaResult {
        self.note = note;

        if self.is_on {
            return self.on();
        }

        Ok(self)
    }
}
