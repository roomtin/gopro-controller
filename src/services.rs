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

///Behavior that a datatype must implement in order to be sent to a GoPro device
pub trait Sendable {
    fn as_bytes(&self) -> &'static [u8];
    fn response_value_bytes(&self) -> &'static [u8];
}

///Represents the different services that a GoPro device can have
pub enum GoProServices {
    #[cfg(feature = "wifi")]
    GoProWiFiAp,
    #[cfg(feature = "wifi")]
    GoProCamManagement,
    ControlAndQuery,
}

impl ToUUID for GoProServices {
    fn to_uuid(&self) -> Uuid {
        match self {
            #[cfg(feature = "wifi")]
            GoProServices::GoProWiFiAp => gp_uuid!("0001"),
            #[cfg(feature = "wifi")]
            GoProServices::GoProCamManagement => gp_uuid!("0090"),
            GoProServices::ControlAndQuery => gp_uuid!("FEA6"),
        }
    }
}

///FOR FUTURE USE
///
///Represents the different characteristics that the GoProWiFiAp service has
#[cfg(feature = "wifi")]
pub enum GoProWifiApCharacteristics {
    SSID,
    Password,
    Power,
    State,
}

#[cfg(feature = "wifi")]
use GoProWifiApCharacteristics as GPWAC; //alias for conciseness

#[cfg(feature = "wifi")]
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
#[cfg(feature = "wifi")]
#[allow(dead_code)]
pub enum GoProManagementCharacteristics {
    NetworkManagementCommand,
    NetworkManagementResponse,
}

#[cfg(feature = "wifi")]
use GoProManagementCharacteristics as GPMC; //alias for conciseness
#[cfg(feature = "wifi")]
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
    Settings,
    SettingsResponse,
    Query,
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
