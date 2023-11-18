use crate::services::Sendable;
///Represents a setting that can be changed on a GoPro device
///
/// ### NOTE ###
///
/// The byte arrays in this enum were taken directly from the GoPro Open Spec:
///
///<https://gopro.github.io/OpenGoPro/ble_2_0#settings-quick-reference>
pub enum GoProSetting {
    Resolution(Hero11Resolution),
    Fps(Hero11FPS),
    AutoPowerDown(Hero11AutoPowerDown),
    VideoDigitalLense(Hero11VideoDigitalLense),
    PhotoDigitalLense(Hero11PhotoDigitalLense),
    TimeLapseDigitalLense(Hero11TimeLapseDigitalLense),
    MediaFormat(Hero11MediaFormat),
    AntiFlicker(AntiFlicker),
    Hypersmooth(Hero11Hypersmooth),
    HorizonLeveling(Hero11HorizonLeveling),
    MaxLense(Hero11MaxLense),
    Hindsight(Hero11Hindsight),
    Controls(Hero11Controls),
    Speed(Hero11Speed),
    NightPhoto(Hero11NightPhoto),
    WirelessBand(Hero11WirelessBand),
    TrailLength(Hero11TrailLength),
    VideoMode(Hero11VideoMode),
}

use GoProSetting as GPS;
impl Sendable for GPS {
    fn as_bytes(&self) -> &'static [u8] {
        match self {
            GPS::Resolution(r) => r.as_bytes(),
            GPS::Fps(f) => f.as_bytes(),
            GPS::AutoPowerDown(apd) => apd.as_bytes(),
            GPS::VideoDigitalLense(vdl) => vdl.as_bytes(),
            GPS::PhotoDigitalLense(pdl) => pdl.as_bytes(),
            GPS::TimeLapseDigitalLense(tdl) => tdl.as_bytes(),
            GPS::MediaFormat(mf) => mf.as_bytes(),
            GPS::AntiFlicker(af) => af.as_bytes(),
            GPS::Hypersmooth(hs) => hs.as_bytes(),
            GPS::HorizonLeveling(hl) => hl.as_bytes(),
            GPS::MaxLense(ml) => ml.as_bytes(),
            GPS::Hindsight(h) => h.as_bytes(),
            GPS::Controls(c) => c.as_bytes(),
            GPS::Speed(s) => s.as_bytes(),
            GPS::NightPhoto(np) => np.as_bytes(),
            GPS::WirelessBand(wb) => wb.as_bytes(),
            GPS::TrailLength(tl) => tl.as_bytes(),
            GPS::VideoMode(vm) => vm.as_bytes(),
        }
    }

    fn response_value_bytes(&self) -> &'static [u8] {
        match self {
            GPS::Resolution(r) => r.response_value_bytes(),
            GPS::Fps(f) => f.response_value_bytes(),
            GPS::AutoPowerDown(apd) => apd.response_value_bytes(),
            GPS::VideoDigitalLense(vdl) => vdl.response_value_bytes(),
            GPS::PhotoDigitalLense(pdl) => pdl.response_value_bytes(),
            GPS::TimeLapseDigitalLense(tdl) => tdl.response_value_bytes(),
            GPS::MediaFormat(mf) => mf.response_value_bytes(),
            GPS::AntiFlicker(af) => af.response_value_bytes(),
            GPS::Hypersmooth(hs) => hs.response_value_bytes(),
            GPS::HorizonLeveling(hl) => hl.response_value_bytes(),
            GPS::MaxLense(ml) => ml.response_value_bytes(),
            GPS::Hindsight(h) => h.response_value_bytes(),
            GPS::Controls(c) => c.response_value_bytes(),
            GPS::Speed(s) => s.response_value_bytes(),
            GPS::NightPhoto(np) => np.response_value_bytes(),
            GPS::WirelessBand(wb) => wb.response_value_bytes(),
            GPS::TrailLength(tl) => tl.response_value_bytes(),
            GPS::VideoMode(vm) => vm.response_value_bytes(),
        }
    }
}

impl AsRef<GoProSetting> for GoProSetting {
    fn as_ref(&self) -> &GoProSetting {
        self
    }
}

#[allow(non_camel_case_types)]
pub enum Hero11Resolution {
    Res4K,
    Res2_7K,
    Res2_7K_4x3,
    Res1080,
    Res4K_4x3,
    Res5_3K_8x7,
    Res5_3K_4x3,
    Res4K_8x7,
    Res5_3K,
}

impl AsRef<Hero11Resolution> for Hero11Resolution {
    fn as_ref(&self) -> &Hero11Resolution {
        self
    }
}

use Hero11Resolution as H11R; //alias for conciseness
impl Sendable for H11R {
    fn as_bytes(&self) -> &'static [u8] {
        match self.as_ref() {
            H11R::Res4K => &[0x03, 0x02, 0x01, 0x01],
            H11R::Res2_7K => &[0x03, 0x02, 0x01, 0x04],
            H11R::Res2_7K_4x3 => &[0x03, 0x02, 0x01, 0x06],
            H11R::Res1080 => &[0x03, 0x02, 0x01, 0x09],
            H11R::Res4K_4x3 => &[0x03, 0x02, 0x01, 0x12],
            H11R::Res5_3K_8x7 => &[0x03, 0x02, 0x01, 0x1A],
            H11R::Res5_3K_4x3 => &[0x03, 0x02, 0x01, 0x1B],
            H11R::Res4K_8x7 => &[0x03, 0x02, 0x01, 0x1C],
            H11R::Res5_3K => &[0x03, 0x02, 0x01, 0x64],
        }
    }

    fn response_value_bytes(&self) -> &'static [u8] {
        &[0x02, 0x02, 0x00]
    }
}

pub enum Hero11FPS {
    Fps240,
    Fps120,
    Fps100,
    Fps60,
    Fps50,
    Fps30,
    Fps25,
    Fps24,
    Fps200,
}

impl AsRef<Hero11FPS> for Hero11FPS {
    fn as_ref(&self) -> &Hero11FPS {
        self
    }
}

impl Sendable for Hero11FPS {
    fn as_bytes(&self) -> &'static [u8] {
        match self.as_ref() {
            Hero11FPS::Fps240 => &[0x03, 0x03, 0x01, 0x00],
            Hero11FPS::Fps120 => &[0x03, 0x03, 0x01, 0x01],
            Hero11FPS::Fps100 => &[0x03, 0x03, 0x01, 0x02],
            Hero11FPS::Fps60 => &[0x03, 0x03, 0x01, 0x05],
            Hero11FPS::Fps50 => &[0x03, 0x03, 0x01, 0x06],
            Hero11FPS::Fps30 => &[0x03, 0x03, 0x01, 0x08],
            Hero11FPS::Fps25 => &[0x03, 0x03, 0x01, 0x09],
            Hero11FPS::Fps24 => &[0x03, 0x03, 0x01, 0x0A],
            Hero11FPS::Fps200 => &[0x03, 0x03, 0x01, 0x0D],
        }
    }

    fn response_value_bytes(&self) -> &'static [u8] {
        &[0x02, 0x03, 0x00]
    }
}

pub enum Hero11AutoPowerDown {
    Never,
    OneMinute,
    FiveMinutes,
    FifteenMinutes,
    ThirtyMinutes,
}

impl AsRef<Hero11AutoPowerDown> for Hero11AutoPowerDown {
    fn as_ref(&self) -> &Hero11AutoPowerDown {
        self
    }
}

use Hero11AutoPowerDown as H11APD;
impl Sendable for H11APD {
    fn as_bytes(&self) -> &'static [u8] {
        match self.as_ref() {
            H11APD::Never => &[0x03, 0x3B, 0x01, 0x00],
            H11APD::OneMinute => &[0x03, 0x3B, 0x01, 0x01],
            H11APD::FiveMinutes => &[0x03, 0x3B, 0x01, 0x04],
            H11APD::FifteenMinutes => &[0x03, 0x3B, 0x01, 0x06],
            H11APD::ThirtyMinutes => &[0x03, 0x3B, 0x01, 0x07],
        }
    }

    fn response_value_bytes(&self) -> &'static [u8] {
        &[0x01, 0x3B, 0x00]
    }
}

pub enum Hero11VideoDigitalLense {
    Wide,
    Superview,
    Linear,
    MaxSuperview,
    LinearHorizonLeveling,
    Hyperview,
    LinearHorizonLock,
}

impl AsRef<Hero11VideoDigitalLense> for Hero11VideoDigitalLense {
    fn as_ref(&self) -> &Hero11VideoDigitalLense {
        self
    }
}

use Hero11VideoDigitalLense as H11VDL;
impl Sendable for H11VDL {
    fn as_bytes(&self) -> &'static [u8] {
        match self.as_ref() {
            H11VDL::Wide => &[0x03, 0x79, 0x01, 0x00],
            H11VDL::Superview => &[0x03, 0x79, 0x01, 0x03],
            H11VDL::Linear => &[0x03, 0x79, 0x01, 0x04],
            H11VDL::MaxSuperview => &[0x03, 0x79, 0x01, 0x07],
            H11VDL::LinearHorizonLeveling => &[0x03, 0x79, 0x01, 0x08],
            H11VDL::Hyperview => &[0x03, 0x79, 0x01, 0x09],
            H11VDL::LinearHorizonLock => &[0x03, 0x79, 0x01, 0x0A],
        }
    }

    fn response_value_bytes(&self) -> &'static [u8] {
        &[0x02, 0x79, 0x00]
    }
}

pub enum Hero11PhotoDigitalLense {
    MaxSuperview,
    Wide,
    Linear,
}

impl AsRef<Hero11PhotoDigitalLense> for Hero11PhotoDigitalLense {
    fn as_ref(&self) -> &Hero11PhotoDigitalLense {
        self
    }
}

use Hero11PhotoDigitalLense as H11PDL;
impl Sendable for H11PDL {
    fn as_bytes(&self) -> &'static [u8] {
        match self.as_ref() {
            H11PDL::MaxSuperview => &[0x03, 0x7A, 0x01, 0x64],
            H11PDL::Wide => &[0x03, 0x7A, 0x01, 0x65],
            H11PDL::Linear => &[0x03, 0x7A, 0x01, 0x66],
        }
    }

    fn response_value_bytes(&self) -> &'static [u8] {
        &[0x02, 0x7A, 0x00]
    }
}

pub enum Hero11TimeLapseDigitalLense {
    MaxSuperview,
    Wide,
    Linear,
}

impl AsRef<Hero11TimeLapseDigitalLense> for Hero11TimeLapseDigitalLense {
    fn as_ref(&self) -> &Hero11TimeLapseDigitalLense {
        self
    }
}

use Hero11TimeLapseDigitalLense as H11TDL;
impl Sendable for H11TDL {
    fn as_bytes(&self) -> &'static [u8] {
        match self.as_ref() {
            H11TDL::MaxSuperview => &[0x03, 0x7B, 0x01, 0x64],
            H11TDL::Wide => &[0x03, 0x7B, 0x01, 0x65],
            H11TDL::Linear => &[0x03, 0x7B, 0x01, 0x66],
        }
    }

    fn response_value_bytes(&self) -> &'static [u8] {
        &[0x02, 0x7B, 0x00]
    }
}

pub enum Hero11MediaFormat {
    TimeLapseVideo,
    TimeLapsePhoto,
    NightLapsePhoto,
    NightLapseVideo,
}

impl AsRef<Hero11MediaFormat> for Hero11MediaFormat {
    fn as_ref(&self) -> &Hero11MediaFormat {
        self
    }
}

use Hero11MediaFormat as H11MF;
impl Sendable for H11MF {
    fn as_bytes(&self) -> &'static [u8] {
        match self.as_ref() {
            H11MF::TimeLapseVideo => &[0x03, 0x80, 0x01, 0x0D],
            H11MF::TimeLapsePhoto => &[0x03, 0x80, 0x01, 0x14],
            H11MF::NightLapsePhoto => &[0x03, 0x80, 0x01, 0x15],
            H11MF::NightLapseVideo => &[0x03, 0x80, 0x01, 0x1A],
        }
    }

    fn response_value_bytes(&self) -> &'static [u8] {
        &[0x02, 0x80, 0x00]
    }
}

pub enum AntiFlicker {
    SixtyHertz,
    FiftyHertz,
}

impl AsRef<AntiFlicker> for AntiFlicker {
    fn as_ref(&self) -> &AntiFlicker {
        self
    }
}

use AntiFlicker as AF;
impl Sendable for AF {
    fn as_bytes(&self) -> &'static [u8] {
        match self.as_ref() {
            AF::SixtyHertz => &[0x03, 0x86, 0x01, 0x02],
            AF::FiftyHertz => &[0x03, 0x86, 0x01, 0x03],
        }
    }

    fn response_value_bytes(&self) -> &'static [u8] {
        &[0x02, 0x86, 0x00]
    }
}

pub enum Hero11Hypersmooth {
    Off,
    Low,
    Boost,
    Auto,
}

impl AsRef<Hero11Hypersmooth> for Hero11Hypersmooth {
    fn as_ref(&self) -> &Hero11Hypersmooth {
        self
    }
}

use Hero11Hypersmooth as H11HS;
impl Sendable for H11HS {
    fn as_bytes(&self) -> &'static [u8] {
        match self.as_ref() {
            H11HS::Off => &[0x03, 0x87, 0x01, 0x00],
            H11HS::Low => &[0x03, 0x87, 0x01, 0x01],
            H11HS::Boost => &[0x03, 0x87, 0x01, 0x03],
            H11HS::Auto => &[0x03, 0x87, 0x01, 0x04],
        }
    }

    fn response_value_bytes(&self) -> &'static [u8] {
        &[0x02, 0x87, 0x00]
    }
}

pub enum Hero11HorizonLeveling {
    VideoOff,
    VideoLocked,
    PhotoOff,
    PhotoLocked,
}

impl AsRef<Hero11HorizonLeveling> for Hero11HorizonLeveling {
    fn as_ref(&self) -> &Hero11HorizonLeveling {
        self
    }
}

use Hero11HorizonLeveling as H11HL;
impl Sendable for H11HL {
    fn as_bytes(&self) -> &'static [u8] {
        match self.as_ref() {
            H11HL::VideoOff => &[0x03, 0x96, 0x01, 0x00],
            H11HL::VideoLocked => &[0x03, 0x96, 0x01, 0x02],
            H11HL::PhotoOff => &[0x03, 0x97, 0x01, 0x00],
            H11HL::PhotoLocked => &[0x03, 0x97, 0x01, 0x02],
        }
    }

    fn response_value_bytes(&self) -> &'static [u8] {
        match self.as_ref() {
            H11HL::VideoOff => &[0x02, 0x96, 0x00],
            H11HL::VideoLocked => &[0x02, 0x96, 0x00],
            H11HL::PhotoOff => &[0x02, 0x97, 0x00],
            H11HL::PhotoLocked => &[0x02, 0x97, 0x00],
        }
    }
}

pub enum Hero11MaxLense {
    Off,
    On,
}

impl AsRef<Hero11MaxLense> for Hero11MaxLense {
    fn as_ref(&self) -> &Hero11MaxLense {
        self
    }
}

use Hero11MaxLense as H11ML;
impl Sendable for H11ML {
    fn as_bytes(&self) -> &'static [u8] {
        match self.as_ref() {
            H11ML::Off => &[0x03, 0xA2, 0x01, 0x00],
            H11ML::On => &[0x03, 0xA2, 0x01, 0x01],
        }
    }

    fn response_value_bytes(&self) -> &'static [u8] {
        &[0x02, 0xA2, 0x00]
    }
}

pub enum Hero11Hindsight {
    FifteenSeconds,
    ThirtySeconds,
    Off,
}

impl AsRef<Hero11Hindsight> for Hero11Hindsight {
    fn as_ref(&self) -> &Hero11Hindsight {
        self
    }
}

use Hero11Hindsight as H11H;
impl Sendable for H11H {
    fn as_bytes(&self) -> &'static [u8] {
        match self.as_ref() {
            H11H::FifteenSeconds => &[0x03, 0xA7, 0x01, 0x02],
            H11H::ThirtySeconds => &[0x03, 0xA7, 0x01, 0x03],
            H11H::Off => &[0x03, 0xA7, 0x01, 0x04],
        }
    }

    fn response_value_bytes(&self) -> &'static [u8] {
        &[0x02, 0xA7, 0x00]
    }
}

pub enum Hero11Controls {
    Easy,
    Pro,
}

impl AsRef<Hero11Controls> for Hero11Controls {
    fn as_ref(&self) -> &Hero11Controls {
        self
    }
}

use Hero11Controls as H11C;

impl Sendable for H11C {
    fn as_bytes(&self) -> &'static [u8] {
        match self.as_ref() {
            H11C::Easy => &[0x03, 0xAF, 0x01, 0x00],
            H11C::Pro => &[0x03, 0xAF, 0x01, 0x01],
        }
    }

    fn response_value_bytes(&self) -> &'static [u8] {
        &[0x02, 0xAF, 0x00]
    }
}

pub enum Hero11Speed {
    UltraSlowMo8X,
    SuperSlowMo4X,
    SlowMo2X,
    Normal1X,
    SuperSlowMo4XExtBatt,
    SlowMo2XExtBatt,
    Normal1XExtBatt,
    UltraSlowMo8X50Hz,
    SuperSlowMo4X50Hz,
    SlowMo2X50Hz,
    Normal1X50Hz,
    SuperSlowMo4XExtBatt50Hz,
    SlowMo2XExtBatt50Hz,
    Normal1XExtBatt50Hz,
    UltraSlowMo8XExtBatt,
    UltraSlowMo8XExtBatt50Hz,
    UltraSlowMo8XLongBatt,
    SuperSlowMo4XLongBatt,
    SlowMo2XLongBatt,
    Normal1XLongBatt,
    UltraSlowMo8XLongBatt50Hz,
    SuperSlowMo4XLongBatt50Hz,
    SlowMo2XLongBatt50Hz,
    Normal1XLongBatt50Hz,
    SlowMo2X4K,
    SuperSlowMo4x2_7K,
    SlowMo2X4K50Hz,
    SuperSlowMo4x2_7K50Hz,
}

impl AsRef<Hero11Speed> for Hero11Speed {
    fn as_ref(&self) -> &Hero11Speed {
        self
    }
}

use Hero11Speed as H11S;
impl Sendable for H11S {
    fn as_bytes(&self) -> &'static [u8] {
        match self.as_ref() {
            H11S::UltraSlowMo8X => &[0x03, 0xB0, 0x01, 0x00],
            H11S::SuperSlowMo4X => &[0x03, 0xB0, 0x01, 0x01],
            H11S::SlowMo2X => &[0x03, 0xB0, 0x01, 0x02],
            H11S::Normal1X => &[0x03, 0xB0, 0x01, 0x03],
            H11S::SuperSlowMo4XExtBatt => &[0x03, 0xB0, 0x01, 0x04],
            H11S::SlowMo2XExtBatt => &[0x03, 0xB0, 0x01, 0x05],
            H11S::Normal1XExtBatt => &[0x03, 0xB0, 0x01, 0x06],
            H11S::UltraSlowMo8X50Hz => &[0x03, 0xB0, 0x01, 0x07],
            H11S::SuperSlowMo4X50Hz => &[0x03, 0xB0, 0x01, 0x08],
            H11S::SlowMo2X50Hz => &[0x03, 0xB0, 0x01, 0x09],
            H11S::Normal1X50Hz => &[0x03, 0xB0, 0x01, 0x0A],
            H11S::SuperSlowMo4XExtBatt50Hz => &[0x03, 0xB0, 0x01, 0x0B],
            H11S::SlowMo2XExtBatt50Hz => &[0x03, 0xB0, 0x01, 0x0C],
            H11S::Normal1XExtBatt50Hz => &[0x03, 0xB0, 0x01, 0x0D],
            H11S::UltraSlowMo8XExtBatt => &[0x03, 0xB0, 0x01, 0x0E],
            H11S::UltraSlowMo8XExtBatt50Hz => &[0x03, 0xB0, 0x01, 0x0F],
            H11S::UltraSlowMo8XLongBatt => &[0x03, 0xB0, 0x01, 0x10],
            H11S::SuperSlowMo4XLongBatt => &[0x03, 0xB0, 0x01, 0x11],
            H11S::SlowMo2XLongBatt => &[0x03, 0xB0, 0x01, 0x12],
            H11S::Normal1XLongBatt => &[0x03, 0xB0, 0x01, 0x13],
            H11S::UltraSlowMo8XLongBatt50Hz => &[0x03, 0xB0, 0x01, 0x14],
            H11S::SuperSlowMo4XLongBatt50Hz => &[0x03, 0xB0, 0x01, 0x15],
            H11S::SlowMo2XLongBatt50Hz => &[0x03, 0xB0, 0x01, 0x16],
            H11S::Normal1XLongBatt50Hz => &[0x03, 0xB0, 0x01, 0x17],
            H11S::SlowMo2X4K => &[0x03, 0xB0, 0x01, 0x18],
            H11S::SuperSlowMo4x2_7K => &[0x03, 0xB0, 0x01, 0x19],
            H11S::SlowMo2X4K50Hz => &[0x03, 0xB0, 0x01, 0x1A],
            H11S::SuperSlowMo4x2_7K50Hz => &[0x03, 0xB0, 0x01, 0x1B],
        }
    }

    fn response_value_bytes(&self) -> &'static [u8] {
        &[0x02, 0xB0, 0x00]
    }
}

pub enum Hero11NightPhoto {
    Off,
    On,
}

impl AsRef<Hero11NightPhoto> for Hero11NightPhoto {
    fn as_ref(&self) -> &Hero11NightPhoto {
        self
    }
}

use Hero11NightPhoto as H11NP;
impl Sendable for H11NP {
    fn as_bytes(&self) -> &'static [u8] {
        match self.as_ref() {
            H11NP::Off => &[0x03, 0xB1, 0x01, 0x00],
            H11NP::On => &[0x03, 0xB1, 0x01, 0x01],
        }
    }

    fn response_value_bytes(&self) -> &'static [u8] {
        &[0x02, 0xB1, 0x00]
    }
}

pub enum Hero11WirelessBand {
    TwoPointFourGhz,
    FiveGhz,
}

impl AsRef<Hero11WirelessBand> for Hero11WirelessBand {
    fn as_ref(&self) -> &Hero11WirelessBand {
        self
    }
}

use Hero11WirelessBand as H11WB;
impl Sendable for H11WB {
    fn as_bytes(&self) -> &'static [u8] {
        match self.as_ref() {
            H11WB::TwoPointFourGhz => &[0x03, 0xB2, 0x01, 0x00],
            H11WB::FiveGhz => &[0x03, 0xB2, 0x01, 0x01],
        }
    }

    fn response_value_bytes(&self) -> &'static [u8] {
        &[0x02, 0xB2, 0x00]
    }
}

pub enum Hero11TrailLength {
    Short,
    Long,
    Max,
}

impl AsRef<Hero11TrailLength> for Hero11TrailLength {
    fn as_ref(&self) -> &Hero11TrailLength {
        self
    }
}

use Hero11TrailLength as H11TL;
impl Sendable for H11TL {
    fn as_bytes(&self) -> &'static [u8] {
        match self.as_ref() {
            H11TL::Short => &[0x03, 0xB3, 0x01, 0x01],
            H11TL::Long => &[0x03, 0xB3, 0x01, 0x02],
            H11TL::Max => &[0x03, 0xB3, 0x01, 0x03],
        }
    }

    fn response_value_bytes(&self) -> &'static [u8] {
        &[0x02, 0xB3, 0x00]
    }
}

pub enum Hero11VideoMode {
    HighestQuality,
    ExtendedBattery,
    ExtendedBatteryGreenIcon,
    LongestBatteryGreenIcon,
}

impl AsRef<Hero11VideoMode> for Hero11VideoMode {
    fn as_ref(&self) -> &Hero11VideoMode {
        self
    }
}

use Hero11VideoMode as H11VM;
impl Sendable for H11VM {
    fn as_bytes(&self) -> &'static [u8] {
        match self.as_ref() {
            H11VM::HighestQuality => &[0x03, 0xB4, 0x01, 0x00],
            H11VM::ExtendedBattery => &[0x03, 0xB4, 0x01, 0x01],
            H11VM::ExtendedBatteryGreenIcon => &[0x03, 0xB4, 0x01, 0x65],
            H11VM::LongestBatteryGreenIcon => &[0x03, 0xB4, 0x01, 0x66],
        }
    }

    fn response_value_bytes(&self) -> &'static [u8] {
        &[0x02, 0xB4, 0x00]
    }
}
