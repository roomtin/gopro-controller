/// Represents a setting or command query that can be sent to a GoPro device
pub enum GoProQuery {
    GetSettingValue(Vec<StatusID>),
    GetAllSettingValues,
    GetStatusValue(Vec<StatusID>),
    GetAllStatusValues,
    GetAvailableOptionSettings(Vec<StatusID>),
    GetAvailableOptionAllSettings,
    RegisterSettingValueUpdates(Vec<StatusID>),
    RegisterStatusValueUpdates(Vec<StatusID>),
    RegisterAvailableOptionSettings(Vec<StatusID>),
    UnregisterSettingValueUpdates(Vec<StatusID>),
    UnregisterStatusValueUpdates(Vec<StatusID>),
    UnregisterAvailableOptionSettings(Vec<StatusID>),
    /// ## TODO:
    /// This variant seems to not work like the rest of the queries
    /// and definitely shouldn't return the bytes that it does
    /// right now.
    /// # Don't use.
    AsyncNotificationSettingChanged,
    /// ## TODO:
    /// This variant seems to not work like the rest of the queries
    /// and definitely shouldn't return the bytes that it does
    /// right now.
    /// # Don't use.
    AsyncNotificationStatusChanged,
    /// ## TODO:
    /// This variant seems to not work like the rest of the queries
    /// and definitely shouldn't return the bytes that it does
    /// right now.
    /// # Don't use.
    AsyncNotificationOptionSettingChanged,
}

impl AsRef<GoProQuery> for GoProQuery {
    fn as_ref(&self) -> &GoProQuery {
        self
    }
}

use GoProQuery as GPC;
impl GoProQuery {
    /// Returns a byte array that can be sent to a GoPro Query characteristic
    ///
    /// # Note:
    ///
    /// The byte arrays in this implementation were taken directly from the GoPro Open Spec:
    /// <https://gopro.github.io/OpenGoPro/ble_2_0#query-commands>
    pub fn as_bytes(&self) -> Vec<u8> {
        match self.as_ref() {
            GPC::GetSettingValue(ids) => query_builder(0x12, ids.to_vec()),
            GPC::GetAllSettingValues => {
                vec![0x01, 0x12]
            }
            GPC::GetStatusValue(ids) => query_builder(0x13, ids.to_vec()),
            GPC::GetAllStatusValues => {
                vec![0x01, 0x13]
            }
            GPC::GetAvailableOptionSettings(ids) => query_builder(0x32, ids.to_vec()),
            GPC::GetAvailableOptionAllSettings => {
                vec![0x01, 0x32]
            }
            GPC::RegisterSettingValueUpdates(ids) => query_builder(0x52, ids.to_vec()),
            GPC::RegisterStatusValueUpdates(ids) => query_builder(0x53, ids.to_vec()),
            GPC::RegisterAvailableOptionSettings(ids) => query_builder(0x62, ids.to_vec()),
            GPC::UnregisterSettingValueUpdates(ids) => query_builder(0x72, ids.to_vec()),
            GPC::UnregisterStatusValueUpdates(ids) => query_builder(0x73, ids.to_vec()),
            GPC::UnregisterAvailableOptionSettings(ids) => query_builder(0x82, ids.to_vec()),

            //TODO: These seem to not work like the rest of the queries
            // and definitely shouldn't return the bytes that they do
            // right now.
            GPC::AsyncNotificationSettingChanged => {
                vec![0x01, 0x92]
            }
            GPC::AsyncNotificationStatusChanged => {
                vec![0x01, 0x93]
            }
            GPC::AsyncNotificationOptionSettingChanged => {
                vec![0x01, 0xA2]
            }
        }
    }
}

/// Builds a proper query byte array from a query id and a vector of status ids
///
/// # Arguments
/// * `query_id` - The id of the query to be sent
/// * `status_ids` - A vector of status ids to be queried
///
/// # Returns
/// A byte array that can be sent to a GoPro Query characteristic
fn query_builder(query_id: u8, status_ids: Vec<StatusID>) -> Vec<u8> {
    let query_body = status_ids
        .iter()
        .map(|id| id.as_byte())
        .collect::<Vec<u8>>();
    [query_body.len() as u8 + 1, query_id]
        .into_iter()
        .chain(query_body)
        .collect::<Vec<u8>>()
}

/// Camera Status Identifiers
///
/// # Note:
///
/// The integers in this implementation were taken directly from the GoPro Open Spec:
/// <https://gopro.github.io/OpenGoPro/ble_2_0#status-ids>
#[derive(Clone, Copy)]
pub enum StatusID {
    /// Indicates if the system’s internal battery is present.
    InternalBatteryPresent = 1,

    /// Approximate level of the internal battery in bars.
    InternalBatteryLevel = 2,

    /// Indicates if the system is currently overheating.
    SystemHot = 6,

    /// Indicates if the camera is busy.
    SystemBusy = 8,

    /// Indicates if Quick Capture feature is enabled.
    QuickCaptureActive = 9,

    /// Indicates if the system is currently encoding.
    EncodingActive = 10,

    /// Indicates if LCD lock is active.
    LCDLockActive = 11,

    /// Duration (in seconds) of the video encoding; 0 otherwise.
    VideoProgressCounter = 13,

    /// Are Wireless Connections enabled?
    WirelessConnectionsEnabled = 17,

    /// The pairing state of the camera.
    PairingState = 19,

    /// The last type of pairing the camera engaged in.
    LastPairingType = 20,

    /// Time (milliseconds) since boot of last successful pairing complete action.
    PairTime = 21,

    /// State of current scan for WiFi Access Points.
    WiFiScanState = 22,

    /// The time, in milliseconds since boot, that the WiFi Access Point scan completed.
    WiFiScanTimeMsec = 23,

    /// WiFi AP provisioning state.
    WiFiProvisionStatus = 24,

    /// Wireless remote control version.
    RemoteControlVersion = 26,

    /// Indicates if a wireless remote control is connected.
    RemoteControlConnected = 27,

    /// Wireless Pairing State.
    WirelessPairingState = 28,

    /// Provisioned WIFI AP SSID.
    WlanSSID = 29,

    /// Camera’s WIFI SSID.
    ApSSID = 30,

    /// The number of wireless devices connected to the camera.
    AppCount = 31,

    /// Indicates if Preview Stream is enabled.
    PreviewStreamEnabled = 32,

    /// Primary Storage Status.
    SdStatus = 33,

    /// How many photos can be taken before sdcard is full.
    RemainingPhotos = 34,

    /// How many minutes of video can be captured before sdcard is full.
    RemainingVideoTime = 35,

    /// How many group photos can be taken before sdcard is full.
    NumGroupPhotos = 36,

    /// Total number of group videos on sdcard.
    NumGroupVideos = 37,

    /// Total number of photos on sdcard.
    NumTotalPhotos = 38,

    /// Total number of videos on sdcard.
    NumTotalVideos = 39,

    /// The current status of Over The Air (OTA) update.
    OtaStatus = 41,

    /// Indicates if there is a pending request to cancel a firmware update download.
    DownloadCancelRequestPending = 42,

    /// Indicates if the locate camera feature is active.
    CameraLocateActive = 45,

    /// The current timelapse interval countdown value.
    MultiShotCountDown = 49,

    /// Remaining space on the sdcard in Kilobytes.
    RemainingSpace = 54,

    /// Indicates if preview stream is supported in current mode.
    PreviewStreamSupported = 55,

    /// WiFi signal strength in bars.
    WiFiBars = 56,

    /// The number of hilights in encoding video.
    NumHilights = 58,

    /// Time since boot (msec) of most recent hilight in encoding video.
    LastHilightTimeMsec = 59,

    /// The min time between camera status updates (msec).
    NextPollMsec = 60,

    /// How many minutes of Timelapse video can be captured before sdcard is full.
    RemainingTimelapseTime = 64,

    /// Liveview Exposure Select Mode.
    ExposureSelectType = 65,

    /// Liveview Exposure Select: x-coordinate (percent).
    ExposureSelectX = 66,

    /// Liveview Exposure Select: y-coordinate (percent).
    ExposureSelectY = 67,

    /// Indicates if the camera currently has a GPS lock.
    GpsStatus = 68,

    /// Indicates if the camera is in AP Mode.
    ApState = 69,

    /// Internal battery level (percent).
    InternalBatteryPercentage = 70,

    /// Microphone Accessory status.
    AccMicStatus = 74,

    /// Digital Zoom level (percent).
    DigitalZoom = 75,

    /// Wireless Band.
    WirelessBand = 76,

    /// Indicates if Digital Zoom feature is available.
    DigitalZoomActive = 77,

    /// Indicates if current video settings are mobile friendly.
    MobileFriendlyVideo = 78,

    /// Indicates if the camera is in First Time Use (FTU) UI flow.
    FirstTimeUse = 79,

    /// Indicates if 5GHz wireless band is available.
    Band5ghzAvailable = 81,

    /// Indicates if the system is ready to accept commands.
    SystemReady = 82,

    /// Indicates if the internal battery is charged sufficiently for OTA update.
    BattOkayForOta = 83,

    /// Indicates if the camera is getting too cold to continue recording.
    VideoLowTempAlert = 85,

    /// The rotational orientation of the camera.
    ActualOrientation = 86,

    /// Indicates if the camera can zoom while encoding.
    ZoomWhileEncoding = 88,

    /// Current flatmode ID.
    CurrentMode = 89,

    /// Current Video Preset (ID).
    ActiveVideoPresets = 93,

    /// Current Photo Preset (ID).
    ActivePhotoPresets = 94,

    /// Current Timelapse Preset (ID).
    ActiveTimelapsePresets = 95,

    /// Current Preset Group (ID).
    ActivePresetsGroup = 96,

    /// Current Preset (ID).
    ActivePreset = 97,

    /// Preset Modified Status.
    PresetModified = 98,

    /// How many Live Bursts can be captured before sdcard is full.
    RemainingLiveBursts = 99,

    /// Total number of Live Bursts on sdcard.
    NumTotalLiveBursts = 100,

    /// Indicates if Capture Delay is currently active.
    CaptureDelayActive = 101,

    /// Media mod State.
    MediaModMicStatus = 102,

    /// Time Warp Speed.
    TimewarpSpeedRampActive = 103,

    /// Indicates if the system’s Linux core is active.
    LinuxCoreActive = 104,

    /// Camera lens type.
    CameraLensType = 105,

    /// Indicates if Video Hindsight Capture is Active.
    VideoHindsightCaptureActive = 106,

    /// Scheduled Capture Preset ID.
    ScheduledPreset = 107,

    /// Indicates if Scheduled Capture is set.
    ScheduledEnabled = 108,

    /// Media Mode Status.
    MediaModStatus = 110,

    /// Indicates if sdcard meets specified minimum write speed.
    SdRatingCheckError = 111,

    /// Number of sdcard write speed errors since device booted.
    SdWriteSpeedError = 112,

    /// Indicates if Turbo Transfer is active.
    TurboTransfer = 113,

    /// Camera control status ID.
    CameraControlStatus = 114,

    /// Indicates if the camera is connected to a PC via USB.
    UsbConnected = 115,

    /// Camera control over USB state.
    AllowControlOverUsb = 116,

    /// Total SD card capacity in Kilobytes.
    TotalSDSpaceKB = 117,
}

impl StatusID {
    /// Returns the byte representation of the status id
    fn as_byte(self) -> u8 {
        if self as u16 > u8::MAX as u16 {
            panic!("Invalid StatusID, must be less than 255")
        }
        self as u8
    }
}

/// Converts a u8 to a StatusID
impl TryFrom<u8> for StatusID {
    type Error = &'static str;

    fn try_from(id: u8) -> Result<Self, Self::Error> {
        match id {
            1 => Ok(StatusID::InternalBatteryPresent),
            2 => Ok(StatusID::InternalBatteryLevel),
            6 => Ok(StatusID::SystemHot),
            8 => Ok(StatusID::SystemBusy),
            9 => Ok(StatusID::QuickCaptureActive),
            10 => Ok(StatusID::EncodingActive),
            11 => Ok(StatusID::LCDLockActive),
            13 => Ok(StatusID::VideoProgressCounter),
            17 => Ok(StatusID::WirelessConnectionsEnabled),
            19 => Ok(StatusID::PairingState),
            20 => Ok(StatusID::LastPairingType),
            21 => Ok(StatusID::PairTime),
            22 => Ok(StatusID::WiFiScanState),
            23 => Ok(StatusID::WiFiScanTimeMsec),
            24 => Ok(StatusID::WiFiProvisionStatus),
            26 => Ok(StatusID::RemoteControlVersion),
            27 => Ok(StatusID::RemoteControlConnected),
            28 => Ok(StatusID::WirelessPairingState),
            29 => Ok(StatusID::WlanSSID),
            30 => Ok(StatusID::ApSSID),
            31 => Ok(StatusID::AppCount),
            32 => Ok(StatusID::PreviewStreamEnabled),
            33 => Ok(StatusID::SdStatus),
            34 => Ok(StatusID::RemainingPhotos),
            35 => Ok(StatusID::RemainingVideoTime),
            36 => Ok(StatusID::NumGroupPhotos),
            37 => Ok(StatusID::NumGroupVideos),
            38 => Ok(StatusID::NumTotalPhotos),
            39 => Ok(StatusID::NumTotalVideos),
            41 => Ok(StatusID::OtaStatus),
            42 => Ok(StatusID::DownloadCancelRequestPending),
            45 => Ok(StatusID::CameraLocateActive),
            49 => Ok(StatusID::MultiShotCountDown),
            54 => Ok(StatusID::RemainingSpace),
            55 => Ok(StatusID::PreviewStreamSupported),
            56 => Ok(StatusID::WiFiBars),
            58 => Ok(StatusID::NumHilights),
            59 => Ok(StatusID::LastHilightTimeMsec),
            60 => Ok(StatusID::NextPollMsec),
            64 => Ok(StatusID::RemainingTimelapseTime),
            65 => Ok(StatusID::ExposureSelectType),
            66 => Ok(StatusID::ExposureSelectX),
            67 => Ok(StatusID::ExposureSelectY),
            68 => Ok(StatusID::GpsStatus),
            69 => Ok(StatusID::ApState),
            70 => Ok(StatusID::InternalBatteryPercentage),
            74 => Ok(StatusID::AccMicStatus),
            75 => Ok(StatusID::DigitalZoom),
            76 => Ok(StatusID::WirelessBand),
            77 => Ok(StatusID::DigitalZoomActive),
            78 => Ok(StatusID::MobileFriendlyVideo),
            79 => Ok(StatusID::FirstTimeUse),
            81 => Ok(StatusID::Band5ghzAvailable),
            82 => Ok(StatusID::SystemReady),
            83 => Ok(StatusID::BattOkayForOta),
            85 => Ok(StatusID::VideoLowTempAlert),
            86 => Ok(StatusID::ActualOrientation),
            88 => Ok(StatusID::ZoomWhileEncoding),
            89 => Ok(StatusID::CurrentMode),
            93 => Ok(StatusID::ActiveVideoPresets),
            94 => Ok(StatusID::ActivePhotoPresets),
            95 => Ok(StatusID::ActiveTimelapsePresets),
            96 => Ok(StatusID::ActivePresetsGroup),
            97 => Ok(StatusID::ActivePreset),
            98 => Ok(StatusID::PresetModified),
            99 => Ok(StatusID::RemainingLiveBursts),
            100 => Ok(StatusID::NumTotalLiveBursts),
            101 => Ok(StatusID::CaptureDelayActive),
            102 => Ok(StatusID::MediaModMicStatus),
            103 => Ok(StatusID::TimewarpSpeedRampActive),
            104 => Ok(StatusID::LinuxCoreActive),
            105 => Ok(StatusID::CameraLensType),
            106 => Ok(StatusID::VideoHindsightCaptureActive),
            107 => Ok(StatusID::ScheduledPreset),
            108 => Ok(StatusID::ScheduledEnabled),
            110 => Ok(StatusID::MediaModStatus),
            111 => Ok(StatusID::SdRatingCheckError),
            112 => Ok(StatusID::SdWriteSpeedError),
            113 => Ok(StatusID::TurboTransfer),
            114 => Ok(StatusID::CameraControlStatus),
            115 => Ok(StatusID::UsbConnected),
            116 => Ok(StatusID::AllowControlOverUsb),
            117 => Ok(StatusID::TotalSDSpaceKB),
            _ => Err("Invalid StatusID"),
        }
    }
}

#[derive(Debug)]
/// Represents the response to a query sent to a GoPro device
pub struct QueryResponse {
    pub message_length: u16,
    /// The query id that was sent
    pub query_id: u8,
    pub command_status: u8,
    /// The status id that was queried
    pub status_id: u8,
    pub status_value_length: u8,
    /// The actual value that the caller is interested in
    pub status_value: Vec<u8>,
}

/// Represents the different ways that a query response can be interpreted
#[derive(Debug)]
pub enum QueryResponseIntepretation {
    /// The response was a single byte
    Byte(u8),

    Bool(bool),
    /// Like a byte, but the value is 0..=100
    Percentage(u8),

    ByteVec(Vec<u8>),
    /// The response was a single byte
    String(String),
}

use QueryResponseIntepretation as QRI;
use StatusID as S;
impl QueryResponse {
    pub fn deserialize(data: &[u8]) -> Result<Self, &'static str> {
        if data.len() < 5 {
            // Header is at least 5 bytes
            return Err("Data too short for header");
        }

        let message_length = if data[0] & 0x80 == 0 {
            // If the first bit is not set, it's a single byte length
            u16::from(data[0])
        } else {
            // If the first bit is set, we assume the length is specified by two bytes
            if data.len() < 6 {
                // Need at least 6 bytes if message length is encoded in 2 bytes
                return Err("Data too short for extended length header");
            }
            u16::from_be_bytes([data[0] & 0x7F, data[1]]) // Masking to remove the leading 1
        };

        let base_index = if message_length > 0x7F { 2 } else { 1 };

        // Gather the rest of the information after the length
        let query_id = data[base_index];
        let command_status = data[base_index + 1];
        let status_id = data[base_index + 2];
        let status_value_length = data[base_index + 3];

        if data.len() < base_index + 4 + status_value_length as usize {
            return Err("Data too short for status value");
        }

        // Gather the status value(s)
        let status_value_start = base_index + 4;
        let status_value_end = status_value_start + status_value_length as usize;
        let status_value = data[status_value_start..status_value_end].to_vec();

        Ok(QueryResponse {
            message_length,
            query_id,
            command_status,
            status_id,
            status_value_length,
            status_value,
        })
    }

    /// If the caller is only interested in the status value, this function
    /// will return it in a format that is easier to work with
    ///
    /// # Note:
    /// All the ByteVec status values are returned as a Vec<u8> for now
    /// but many of them should actually be interpreted as a struct.
    ///
    /// Furthermore, all of the Byte status values are returned as a u8 but
    /// each of these could be represented as an enum for clearer semantics.
    /// As of now, the caller will have to view the Value field of the GoPro
    /// Open Spec to determine what the value means:
    ///
    /// <https://gopro.github.io/OpenGoPro/ble_2_0#status-ids>
    ///
    /// # Returns:
    /// * `Some(QueryResponseIntepretation)` - If the status id is valid
    /// * `None` - If the status id is invalid
    pub fn interpret(&self) -> Option<QueryResponseIntepretation> {
        let status_id = StatusID::try_from(self.status_id);
        if let Err(_) = status_id {
            return None;
        }
        let status_id = status_id.unwrap();
        let interpretation = match status_id {
            //Group Bool Like Responses
            S::InternalBatteryPresent
            | S::SystemHot
            | S::SystemBusy
            | S::QuickCaptureActive
            | S::EncodingActive
            | S::LCDLockActive
            | S::WirelessConnectionsEnabled
            | S::RemoteControlConnected
            | S::PreviewStreamEnabled
            | S::DownloadCancelRequestPending
            | S::CameraLocateActive
            | S::PreviewStreamSupported
            | S::GpsStatus
            | S::ApState
            | S::DigitalZoomActive
            | S::MobileFriendlyVideo
            | S::FirstTimeUse
            | S::Band5ghzAvailable
            | S::SystemReady
            | S::BattOkayForOta
            | S::VideoLowTempAlert
            | S::ZoomWhileEncoding
            | S::CaptureDelayActive
            | S::LinuxCoreActive
            | S::VideoHindsightCaptureActive
            | S::ScheduledEnabled
            | S::SdRatingCheckError
            | S::TurboTransfer
            | S::UsbConnected
            | S::AllowControlOverUsb => QRI::Bool(self.status_value[0] == 1),

            // Group Byte Like Responses
            S::InternalBatteryLevel
            | S::PairingState
            | S::LastPairingType
            | S::WiFiScanState
            | S::WiFiProvisionStatus
            | S::SdStatus
            | S::OtaStatus
            | S::ExposureSelectType
            | S::AccMicStatus
            | S::WirelessBand
            | S::ActualOrientation
            | S::MediaModMicStatus
            | S::TimewarpSpeedRampActive
            | S::CameraLensType
            | S::MediaModStatus
            | S::CameraControlStatus => QRI::Byte(self.status_value[0]),

            // Group ByteVec Like Responses
            //TODO: Interpret this further, perhaps as a struct
            S::VideoProgressCounter
            | S::PairTime 
            | S::WiFiScanTimeMsec 
            | S::RemoteControlVersion 
            | S::WirelessPairingState
            | S::AppCount
            | S::RemainingPhotos
            | S::RemainingVideoTime
            | S::NumGroupPhotos
            | S::NumGroupVideos
            | S::NumTotalPhotos
            | S::NumTotalVideos
            | S::MultiShotCountDown
            | S::RemainingSpace
            | S::WiFiBars
            | S::NumHilights
            | S::LastHilightTimeMsec
            | S::NextPollMsec
            | S::RemainingTimelapseTime
            | S::CurrentMode
            | S::ActiveVideoPresets
            | S::ActivePhotoPresets
            | S::ActiveTimelapsePresets
            | S::ActivePresetsGroup
            | S::ActivePreset
            | S::PresetModified
            | S::RemainingLiveBursts
            | S::NumTotalLiveBursts
            | S::ScheduledPreset
            | S::SdWriteSpeedError
            | S::TotalSDSpaceKB => QRI::ByteVec(self.status_value.to_vec()),

            //Group String Like Responses

            //TODO: TEST THIS TO MAKE SURE THE STRING IS UTF-8
            S::WlanSSID | S::ApSSID => {
                QRI::String(String::from_utf8(self.status_value.to_vec()).unwrap())
            }

            //Group Percentage Like Responses
            S::ExposureSelectX
            | S::ExposureSelectY
            | S::InternalBatteryPercentage
            | S::DigitalZoom => QRI::Percentage(self.status_value[0]),
        };
        Some(interpretation)
    }
}

#[test]
/// Sanity test for query builder
fn test_query_builder() {
    let query = query_builder(
        0x01,
        vec![
            StatusID::InternalBatteryLevel,
            StatusID::InternalBatteryPresent,
        ],
    );
    assert_eq!(query, vec![0x03, 0x01, 0x02, 0x01]);
}
