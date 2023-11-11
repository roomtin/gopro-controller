use uuid::Uuid;

///Macro for creating a Full GoPro UUID from a 16 bit UUID
///NOTE: This macro will cause a runtime error if the UUID is not passed in as a u32
macro_rules! gp_uuid {
    ($x:expr) => {{
        // Concatenate to form the full UUID
        let full_uuid = format!("b5f9{}-aa8d-11e3-9046-0002a5d5c51b", $x);
        Uuid::parse_str(&full_uuid).expect("Invalid UUID")
    }};
}

#[test]
fn test_macro() {
    let uuid = gp_uuid!("0072");
    assert_eq!(uuid, gp_uuid!("0072"));
}

///Behavior for converting a datatype to a GoPro global UUID
pub trait ToUUID {
    fn to_uuid(&self) -> Uuid;
}

///Represents the different services that a GoPro device can have
pub enum GoProServices {
    #[allow(dead_code)]
    GoProWiFiAp,
    #[allow(dead_code)]
    GoProCamManagement,
    ControlAndQuery,
}

impl ToUUID for GoProServices {
    fn to_uuid(&self) -> Uuid {
        match self {
            GoProServices::GoProWiFiAp => gp_uuid!("0001"),
            GoProServices::GoProCamManagement => gp_uuid!("0090"),
            GoProServices::ControlAndQuery => gp_uuid!("FEA6"),
        }
    }
}

///FOR FUTURE USE
///
///Represents the different characteristics that the GoProWiFiAp service has
#[allow(dead_code)]
pub enum GoProWifiApCharacteristics {
    SSID,
    Password,
    Power,
    State,
}

use GoProWifiApCharacteristics as GPWAC; //alias for conciseness
impl ToUUID for GoProWifiApCharacteristics {
    fn to_uuid(&self) -> Uuid {
        match self {
            GPWAC::SSID => gp_uuid!("0002"),
            GPWAC::Password => gp_uuid!("0003"),
            GPWAC::Power => gp_uuid!("0004"),
            GPWAC::State => gp_uuid!("0005"),
        }
    }
}

///FOR FUTURE USE
///
///Represents the different characteristics that the GoProCamManagement service has
#[allow(dead_code)]
pub enum GoProManagementCharacteristics {
    NetworkManagementCommand,
    NetworkManagementResponse,
}

use GoProManagementCharacteristics as GPMC; //alias for conciseness
impl ToUUID for GoProManagementCharacteristics {
    fn to_uuid(&self) -> Uuid {
        match self {
            GPMC::NetworkManagementCommand => gp_uuid!("0091"),
            GPMC::NetworkManagementResponse => gp_uuid!("0092"),
        }
    }
}

///Represents the different characteristics that the GoProControlAndQuery service has
pub enum GoProControlAndQueryCharacteristics {
    Command,
    CommandResponse,
    #[allow(dead_code)]
    Settings,
    #[allow(dead_code)]
    SettingsResponse,
    #[allow(dead_code)]
    Query,
    #[allow(dead_code)]
    QueryResponse,
}

use GoProControlAndQueryCharacteristics as GPCAQ; //alias for conciseness
impl ToUUID for GoProControlAndQueryCharacteristics {
    fn to_uuid(&self) -> Uuid {
        match self {
            GPCAQ::Command => gp_uuid!("0072"),
            GPCAQ::CommandResponse => gp_uuid!("0073"),
            GPCAQ::Settings => gp_uuid!("0074"),
            GPCAQ::SettingsResponse => gp_uuid!("0075"),
            GPCAQ::Query => gp_uuid!("0076"),
            GPCAQ::QueryResponse => gp_uuid!("0077"),
        }
    }
}

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

///Behavior that a datatype must implement in order to be sent to a GoPro device
pub trait Sendable {
    fn as_bytes(&self) -> &'static [u8];
    fn response_value_bytes(&self) -> &'static [u8];
}

use GoProCommand as GPC; //alias for conciseness

///Implement Sendable for all GoProCommands generically
///to avoid the duplicate code of also implementing it
///for references to GoProCommands
///
///
///NOTE: The byte arrays in this implementation were taken directly from the GoPro Open Spec:
///<https://gopro.github.io/OpenGoPro/ble_2_0#commands-quick-reference>
impl<T> Sendable for T
where
    T: AsRef<GoProCommand>,
{
    fn as_bytes(&self) -> &'static [u8] {
        match self.as_ref() {
            GPC::ShutterStart => &[0x03, 0x01, 0x01, 0x01],
            GPC::ShutterStop => &[0x03, 0x01, 0x01, 0x00],
            GPC::Sleep => &[0x01, 0x05],
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
            GPC::AddHilightDuringEncoding => &[0x02, 0x18, 0x00],
            GPC::VideoMode => &[0x02, 0x3E, 0x00],
            GPC::PhotoMode => &[0x02, 0x3E, 0x00],
            GPC::TimelapseMode => &[0x02, 0x3E, 0x00],
        }
    }
}
