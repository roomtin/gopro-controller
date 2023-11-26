use crate::services::Sendable;
///Represents a command that can be sent to a GoPro device
///
/// ### NOTE ###
///
/// The byte arrays in this enum were taken directly from the GoPro Open Spec:
///
/// <https://gopro.github.io/OpenGoPro/ble_2_0#commands-quick-reference>
pub enum GoProCommand {
    ShutterStart,
    ShutterStop,
    Sleep,
    AddHilightDuringEncoding,
    #[cfg(feature = "wifi")]
    ApOff,
    #[cfg(feature = "wifi")]
    ApOn,
    VideoMode,
    PhotoMode,
    TimelapseMode,
}

///Implement AsRef for GoProCommands so that relevant functions
///can take a GoProCommand by reference or by value
impl AsRef<GoProCommand> for GoProCommand {
    fn as_ref(&self) -> &GoProCommand {
        self
    }
}

use GoProCommand as GPC; //alias for conciseness

///Implement Sendable for all GoProCommands generically
///to avoid the duplicate code of also implementing it
///for references to GoProCommands
///
///
///NOTE: The byte arrays in this implementation were taken directly from the GoPro Open Spec:
///<https://gopro.github.io/OpenGoPro/ble_2_0#commands-quick-reference>
impl Sendable for GPC {
    fn as_bytes(&self) -> &'static [u8] {
        match self.as_ref() {
            GPC::ShutterStart => &[0x03, 0x01, 0x01, 0x01],
            GPC::ShutterStop => &[0x03, 0x01, 0x01, 0x00],
            GPC::Sleep => &[0x01, 0x05],
            #[cfg(feature = "wifi")]
            GPC::ApOff => &[0x03, 0x17, 0x01, 0x00],
            #[cfg(feature = "wifi")]
            GPC::ApOn => &[0x03, 0x17, 0x01, 0x01],
            GPC::AddHilightDuringEncoding => &[0x01, 0x18],
            GPC::VideoMode => &[0x04, 0x3E, 0x02, 0x03, 0xE8],
            GPC::PhotoMode => &[0x04, 0x3E, 0x02, 0x03, 0xE9],
            GPC::TimelapseMode => &[0x04, 0x3E, 0x02, 0x03, 0xEA],
        }
    }
    fn response_value_bytes(&self) -> &'static [u8] {
        match self.as_ref() {
            GPC::ShutterStart => &[0x02, 0x01, 0x00],
            GPC::ShutterStop => &[0x02, 0x01, 0x00],
            GPC::Sleep => &[0x02, 0x05, 0x00],
            #[cfg(feature = "wifi")]
            GPC::ApOff => &[0x02, 0x17, 0x00],
            #[cfg(feature = "wifi")]
            GPC::ApOn => &[0x02, 0x17, 0x00],
            GPC::AddHilightDuringEncoding => &[0x02, 0x18, 0x00],
            GPC::VideoMode => &[0x02, 0x3E, 0x00],
            GPC::PhotoMode => &[0x02, 0x3E, 0x00],
            GPC::TimelapseMode => &[0x02, 0x3E, 0x00],
        }
    }
}
