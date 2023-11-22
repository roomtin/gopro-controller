use crate::services::Sendable;

/// Implements the Sendable trait for a given enum where there is
/// only one kind of response value
///
/// # Arguments
/// * `$enum` - The enum to implement the trait for
/// * `$response` - The response bytes for the enum
/// * `$($variant:path => $bytes:expr),*` - The bytes for each variant
///
/// # Usage
/// ```
///sendable_impl!(
///    Hero11Resolution,
///    &[0x02, 0x02, 0x00],
///    Hero11Resolution::Res4K => &[0x03, 0x02, 0x01, 0x01],
///    Hero11Resolution::Res2_7K => &[0x03, 0x02, 0x01, 0x04],
///    Hero11Resolution::Res2_7K_4x3 => &[0x03, 0x02, 0x01, 0x06],
///    Hero11Resolution::Res1080 => &[0x03, 0x02, 0x01, 0x09],
///    Hero11Resolution::Res4K_4x3 => &[0x03, 0x02, 0x01, 0x12],
///    Hero11Resolution::Res5_3K_8x7 => &[0x03, 0x02, 0x01, 0x1A],
///    Hero11Resolution::Res5_3K_4x3 => &[0x03, 0x02, 0x01, 0x1B],
///    Hero11Resolution::Res4K_8x7 => &[0x03, 0x02, 0x01, 0x1C],
///    Hero11Resolution::Res5_3K => &[0x03, 0x02, 0x01, 0x64]
///);
///```
/// ### Which expands into
///```
///impl std::convert::AsRef<Hero11Resolution> for Hero11Resolution {
///    fn as_ref(&self) -> &Hero11Resolution {
///        self
///    }
///}
///
///impl Sendable for Hero11Resolution {
///    fn as_bytes(&self) -> &'static [u8] {
///        match self.as_ref() {
///            Hero11Resolution::Res4K => &[0x03, 0x02, 0x01, 0x01],
///            Hero11Resolution::Res2_7K => &[0x03, 0x02, 0x01, 0x04],
///            Hero11Resolution::Res2_7K_4x3 => &[0x03, 0x02, 0x01, 0x06],
///            Hero11Resolution::Res1080 => &[0x03, 0x02, 0x01, 0x09],
///            Hero11Resolution::Res4K_4x3 => &[0x03, 0x02, 0x01, 0x12],
///            Hero11Resolution::Res5_3K_8x7 => &[0x03, 0x02, 0x01, 0x1A],
///            Hero11Resolution::Res5_3K_4x3 => &[0x03, 0x02, 0x01, 0x1B],
///            Hero11Resolution::Res4K_8x7 => &[0x03, 0x02, 0x01, 0x1C],
///            Hero11Resolution::Res5_3K => &[0x03, 0x02, 0x01, 0x64],
///        }
///    }
///
///    fn response_value_bytes(&self) -> &'static [u8] {
///        &[0x02, 0x02, 0x00]
///    }
///}
macro_rules! sendable_impl {
    ($enum:ty, $response:expr, $($variant:path => $bytes:expr),*) => {
        impl std::convert::AsRef<$enum> for $enum {
            fn as_ref(&self) -> &$enum {
                self
            }
        }

        impl Sendable for $enum {
            fn as_bytes(&self) -> &'static [u8] {
                match self.as_ref() {
                    $( $variant => $bytes, )*
                }
            }

            fn response_value_bytes(&self) -> &'static [u8] {
                $response
            }
        }
    };
}

/// Implements the Sendable trait for a given enum where there is
/// more than one kind of response value
macro_rules! sendable_impl_complex {
    ($enum:ty, $($variant:path => $bytes:expr, $response:expr),*) => {
        impl std::convert::AsRef<$enum> for $enum {
            fn as_ref(&self) -> &$enum {
                self
            }
        }

        impl Sendable for $enum {
            fn as_bytes(&self) -> &'static [u8] {
                match self.as_ref() {
                    $( $variant => $bytes, )*
                }
            }

            fn response_value_bytes(&self) -> &'static [u8] {
                match self.as_ref() {
                    $( $variant => $response, )*
                }
            }
        }
    };
}

///Represents a setting that can be changed on a GoPro device
///
/// ### NOTE:
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

sendable_impl!(
    Hero11Resolution,
    &[0x02, 0x02, 0x00], // Response bytes
    Hero11Resolution::Res4K => &[0x03, 0x02, 0x01, 0x01],
    Hero11Resolution::Res2_7K => &[0x03, 0x02, 0x01, 0x04],
    Hero11Resolution::Res2_7K_4x3 => &[0x03, 0x02, 0x01, 0x06],
    Hero11Resolution::Res1080 => &[0x03, 0x02, 0x01, 0x09],
    Hero11Resolution::Res4K_4x3 => &[0x03, 0x02, 0x01, 0x12],
    Hero11Resolution::Res5_3K_8x7 => &[0x03, 0x02, 0x01, 0x1A],
    Hero11Resolution::Res5_3K_4x3 => &[0x03, 0x02, 0x01, 0x1B],
    Hero11Resolution::Res4K_8x7 => &[0x03, 0x02, 0x01, 0x1C],
    Hero11Resolution::Res5_3K => &[0x03, 0x02, 0x01, 0x64]
);

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

sendable_impl!(
    Hero11FPS,
    &[0x02, 0x02, 0x00], // Response bytes
    Hero11FPS::Fps240 => &[0x03, 0x03, 0x01, 0x00],
    Hero11FPS::Fps120 => &[0x03, 0x03, 0x01, 0x01],
    Hero11FPS::Fps100 => &[0x03, 0x03, 0x01, 0x02],
    Hero11FPS::Fps60 => &[0x03, 0x03, 0x01, 0x05],
    Hero11FPS::Fps50 => &[0x03, 0x03, 0x01, 0x06],
    Hero11FPS::Fps30 => &[0x03, 0x03, 0x01, 0x08],
    Hero11FPS::Fps25 => &[0x03, 0x03, 0x01, 0x09],
    Hero11FPS::Fps24 => &[0x03, 0x03, 0x01, 0x0A],
    Hero11FPS::Fps200 => &[0x03, 0x03, 0x01, 0x0D]
);

pub enum Hero11AutoPowerDown {
    Never,
    OneMinute,
    FiveMinutes,
    FifteenMinutes,
    ThirtyMinutes,
}

sendable_impl!(
    Hero11AutoPowerDown,
    &[0x02, 0x02, 0x00], // Response bytes
    Hero11AutoPowerDown::Never => &[0x03, 0x3B, 0x01, 0x00],
    Hero11AutoPowerDown::OneMinute => &[0x03, 0x3B, 0x01, 0x01],
    Hero11AutoPowerDown::FiveMinutes => &[0x03, 0x3B, 0x01, 0x04],
    Hero11AutoPowerDown::FifteenMinutes => &[0x03, 0x3B, 0x01, 0x06],
    Hero11AutoPowerDown::ThirtyMinutes => &[0x03, 0x3B, 0x01, 0x07]
);

pub enum Hero11VideoDigitalLense {
    Wide,
    Superview,
    Linear,
    MaxSuperview,
    LinearHorizonLeveling,
    Hyperview,
    LinearHorizonLock,
}

sendable_impl!(
    Hero11VideoDigitalLense,
    &[0x02, 0x02, 0x00], // Response bytes
    Hero11VideoDigitalLense::Wide => &[0x03, 0x79, 0x01, 0x00],
    Hero11VideoDigitalLense::Superview => &[0x03, 0x79, 0x01, 0x03],
    Hero11VideoDigitalLense::Linear => &[0x03, 0x79, 0x01, 0x04],
    Hero11VideoDigitalLense::MaxSuperview => &[0x03, 0x79, 0x01, 0x07],
    Hero11VideoDigitalLense::LinearHorizonLeveling => &[0x03, 0x79, 0x01, 0x08],
    Hero11VideoDigitalLense::Hyperview => &[0x03, 0x79, 0x01, 0x09],
    Hero11VideoDigitalLense::LinearHorizonLock => &[0x03, 0x79, 0x01, 0x0A]
);

pub enum Hero11PhotoDigitalLense {
    MaxSuperview,
    Wide,
    Linear,
}

sendable_impl!(
    Hero11PhotoDigitalLense,
    &[0x02, 0x02, 0x00], // Response bytes
    Hero11PhotoDigitalLense::MaxSuperview => &[0x03, 0x7A, 0x01, 0x64],
    Hero11PhotoDigitalLense::Wide => &[0x03, 0x7A, 0x01, 0x65],
    Hero11PhotoDigitalLense::Linear => &[0x03, 0x7A, 0x01, 0x66]
);

pub enum Hero11TimeLapseDigitalLense {
    MaxSuperview,
    Wide,
    Linear,
}

sendable_impl!(
    Hero11TimeLapseDigitalLense,
    &[0x02, 0x02, 0x00], // Response bytes
    Hero11TimeLapseDigitalLense::MaxSuperview => &[0x03, 0x7B, 0x01, 0x64],
    Hero11TimeLapseDigitalLense::Wide => &[0x03, 0x7B, 0x01, 0x65],
    Hero11TimeLapseDigitalLense::Linear => &[0x03, 0x7B, 0x01, 0x66]
);

pub enum Hero11MediaFormat {
    TimeLapseVideo,
    TimeLapsePhoto,
    NightLapsePhoto,
    NightLapseVideo,
}

sendable_impl!(
    Hero11MediaFormat,
    &[0x02, 0x02, 0x00], // Response bytes
    Hero11MediaFormat::TimeLapseVideo => &[0x03, 0x80, 0x01, 0x0D],
    Hero11MediaFormat::TimeLapsePhoto => &[0x03, 0x80, 0x01, 0x14],
    Hero11MediaFormat::NightLapsePhoto => &[0x03, 0x80, 0x01, 0x15],
    Hero11MediaFormat::NightLapseVideo => &[0x03, 0x80, 0x01, 0x1A]
);

pub enum AntiFlicker {
    SixtyHertz,
    FiftyHertz,
}

sendable_impl!(
    AntiFlicker,
    &[0x02, 0x02, 0x00], // Response bytes
    AntiFlicker::SixtyHertz => &[0x03, 0x86, 0x01, 0x02],
    AntiFlicker::FiftyHertz => &[0x03, 0x86, 0x01, 0x03]
);

pub enum Hero11Hypersmooth {
    Off,
    Low,
    Boost,
    Auto,
}

sendable_impl!(
    Hero11Hypersmooth,
    &[0x02, 0x02, 0x00], // Response bytes
    Hero11Hypersmooth::Off => &[0x03, 0x87, 0x01, 0x00],
    Hero11Hypersmooth::Low => &[0x03, 0x87, 0x01, 0x01],
    Hero11Hypersmooth::Boost => &[0x03, 0x87, 0x01, 0x03],
    Hero11Hypersmooth::Auto => &[0x03, 0x87, 0x01, 0x04]
);

pub enum Hero11HorizonLeveling {
    VideoOff,
    VideoLocked,
    PhotoOff,
    PhotoLocked,
}

sendable_impl_complex!(
    Hero11HorizonLeveling,
    Hero11HorizonLeveling::VideoOff => &[0x03, 0x96, 0x01, 0x00],
    &[0x02, 0x02, 0x00], // Response bytes
    Hero11HorizonLeveling::VideoLocked => &[0x03, 0x96, 0x01, 0x02],
    &[0x02, 0x02, 0x00], // Response bytes
    Hero11HorizonLeveling::PhotoOff => &[0x03, 0x97, 0x01, 0x00],
    &[0x02, 0x02, 0x00], // Response bytes
    Hero11HorizonLeveling::PhotoLocked => &[0x03, 0x97, 0x01, 0x02],
    &[0x02, 0x97, 0x00] // Response bytes
);

pub enum Hero11MaxLense {
    Off,
    On,
}

sendable_impl!(
    Hero11MaxLense,
    &[0x02, 0x02, 0x00], // Response bytes
    Hero11MaxLense::Off => &[0x03, 0xA2, 0x01, 0x00],
    Hero11MaxLense::On => &[0x03, 0xA2, 0x01, 0x01]
);

pub enum Hero11Hindsight {
    FifteenSeconds,
    ThirtySeconds,
    Off,
}

sendable_impl!(
    Hero11Hindsight,
    &[0x02, 0x02, 0x00], // Response bytes
    Hero11Hindsight::FifteenSeconds => &[0x03, 0xA7, 0x01, 0x02],
    Hero11Hindsight::ThirtySeconds => &[0x03, 0xA7, 0x01, 0x03],
    Hero11Hindsight::Off => &[0x03, 0xA7, 0x01, 0x04]
);

pub enum Hero11Controls {
    Easy,
    Pro,
}

sendable_impl!(
    Hero11Controls,
    &[0x02, 0x02, 0x00], // Response bytes
    Hero11Controls::Easy => &[0x03, 0xAF, 0x01, 0x00],
    Hero11Controls::Pro => &[0x03, 0xAF, 0x01, 0x01]
);

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

sendable_impl!(
    Hero11Speed,
    &[0x02, 0x02, 0x00], // Response bytes
    Hero11Speed::UltraSlowMo8X => &[0x03, 0xB0, 0x01, 0x00],
    Hero11Speed::SuperSlowMo4X => &[0x03, 0xB0, 0x01, 0x01],
    Hero11Speed::SlowMo2X => &[0x03, 0xB0, 0x01, 0x02],
    Hero11Speed::Normal1X => &[0x03, 0xB0, 0x01, 0x03],
    Hero11Speed::SuperSlowMo4XExtBatt => &[0x03, 0xB0, 0x01, 0x04],
    Hero11Speed::SlowMo2XExtBatt => &[0x03, 0xB0, 0x01, 0x05],
    Hero11Speed::Normal1XExtBatt => &[0x03, 0xB0, 0x01, 0x06],
    Hero11Speed::UltraSlowMo8X50Hz => &[0x03, 0xB0, 0x01, 0x07],
    Hero11Speed::SuperSlowMo4X50Hz => &[0x03, 0xB0, 0x01, 0x08],
    Hero11Speed::SlowMo2X50Hz => &[0x03, 0xB0, 0x01, 0x09],
    Hero11Speed::Normal1X50Hz => &[0x03, 0xB0, 0x01, 0x0A],
    Hero11Speed::SuperSlowMo4XExtBatt50Hz => &[0x03, 0xB0, 0x01, 0x0B],
    Hero11Speed::SlowMo2XExtBatt50Hz => &[0x03, 0xB0, 0x01, 0x0C],
    Hero11Speed::Normal1XExtBatt50Hz => &[0x03, 0xB0, 0x01, 0x0D],
    Hero11Speed::UltraSlowMo8XExtBatt => &[0x03, 0xB0, 0x01, 0x0E],
    Hero11Speed::UltraSlowMo8XExtBatt50Hz => &[0x03, 0xB0, 0x01, 0x0F],
    Hero11Speed::UltraSlowMo8XLongBatt => &[0x03, 0xB0, 0x01, 0x10],
    Hero11Speed::SuperSlowMo4XLongBatt => &[0x03, 0xB0, 0x01, 0x11],
    Hero11Speed::SlowMo2XLongBatt => &[0x03, 0xB0, 0x01, 0x12],
    Hero11Speed::Normal1XLongBatt => &[0x03, 0xB0, 0x01, 0x13],
    Hero11Speed::UltraSlowMo8XLongBatt50Hz => &[0x03, 0xB0, 0x01, 0x14],
    Hero11Speed::SuperSlowMo4XLongBatt50Hz => &[0x03, 0xB0, 0x01, 0x15],
    Hero11Speed::SlowMo2XLongBatt50Hz => &[0x03, 0xB0, 0x01, 0x16],
    Hero11Speed::Normal1XLongBatt50Hz => &[0x03, 0xB0, 0x01, 0x17],
    Hero11Speed::SlowMo2X4K => &[0x03, 0xB0, 0x01, 0x18],
    Hero11Speed::SuperSlowMo4x2_7K => &[0x03, 0xB0, 0x01, 0x19],
    Hero11Speed::SlowMo2X4K50Hz => &[0x03, 0xB0, 0x01, 0x1A],
    Hero11Speed::SuperSlowMo4x2_7K50Hz => &[0x03, 0xB0, 0x01, 0x1B]
);

pub enum Hero11NightPhoto {
    Off,
    On,
}

sendable_impl!(
    Hero11NightPhoto,
    &[0x02, 0xB1, 0x00], //Response bytes
    Hero11NightPhoto::Off => &[0x03, 0xB1, 0x01, 0x00],
    Hero11NightPhoto::On => &[0x03, 0xB1, 0x01, 0x01]
);

pub enum Hero11WirelessBand {
    TwoPointFourGhz,
    FiveGhz,
}

sendable_impl!(
    Hero11WirelessBand,
    &[0x02, 0xB2, 0x00], //Response bytes
    Hero11WirelessBand::TwoPointFourGhz => &[0x03, 0xB2, 0x01, 0x00],
    Hero11WirelessBand::FiveGhz => &[0x03, 0xB2, 0x01, 0x01]
);

pub enum Hero11TrailLength {
    Short,
    Long,
    Max,
}

sendable_impl!(
    Hero11TrailLength,
    &[0x02, 0xB3, 0x00], //Response bytes
    Hero11TrailLength::Short => &[0x03, 0xB3, 0x01, 0x01],
    Hero11TrailLength::Long => &[0x03, 0xB3, 0x01, 0x02],
    Hero11TrailLength::Max => &[0x03, 0xB3, 0x01, 0x03]
);

pub enum Hero11VideoMode {
    HighestQuality,
    ExtendedBattery,
    ExtendedBatteryGreenIcon,
    LongestBatteryGreenIcon,
}

sendable_impl!(
    Hero11VideoMode,
    &[0x02, 0xB4, 0x00], //Response bytes
    Hero11VideoMode::HighestQuality => &[0x03, 0xB4, 0x01, 0x00],
    Hero11VideoMode::ExtendedBattery => &[0x03, 0xB4, 0x01, 0x01],
    Hero11VideoMode::ExtendedBatteryGreenIcon => &[0x03, 0xB4, 0x01, 0x65],
    Hero11VideoMode::LongestBatteryGreenIcon => &[0x03, 0xB4, 0x01, 0x66]
);
