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
}

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
impl<T> Sendable for T
where
  T: AsRef<H11R>,
{
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

    fn response_value_bytes(&self) -> &'static [u8] {
        [0x02, 0x02, 0x00]
    }
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
  Fps200
}

impl AsRef<Hero11FPS> for Hero11FPS {
  fn as_ref(&self) -> &Hero11FPS {
    self
  }
}

impl<T> Sendable for T 
where 
  T: AsRef<Hero11FPS>
{
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
      Hero11FPS::Fps200 => &[0x03, 0x03, 0x01, 0x0D]
    }
  }

  fn response_value_bytes(&self) -> &'static [u8] {
    [0x02, 0x03, 0x00]
  }
}

pub enum Hero11AutoPowerDown {
  Never,
  OneMinute,
  FiveMinutes,
  FifteenMinutes,
  ThirtyMinutes
}

impl AsRef<Hero11AutoPowerDown> for Hero11AutoPowerDown {
  fn as_ref(&self) -> &Hero11AutoPowerDown {
    self
  }
}

use Hero11AutoPowerDown as H11APD;
impl<T> Sendable for T 
where 
  T: AsRef<H11APD>
{
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
    [0x01, 0x3B, 0x00]
  }
}

enum Hero11VideoDigitalLense {
  Wide,
  Superview,
  Linear,
  MaxSuperview,
  LinearHorizonLeveling,
  Hyperview,
  LinearHorizonLock
}

impl AsRef<Hero11VideoDigitalLense> for Hero11VideoDigitalLense {
  fn as_ref(&self) -> &Hero11VideoDigitalLense {
    self
  }
}

use Hero11VideoDigitalLense as H11VDL;
impl<T> Sendable for T 
where 
  T: AsRef<H11DL>
{
  fn as_bytes(&self) -> &'static [u8] {
    match self.as_ref() {
      H11DL::Wide => &[0x03, 0x79, 0x01, 0x00],
      H11DL::Superview => &[0x03, 0x79, 0x01, 0x03],
      H11DL::Linear => &[0x03, 0x79, 0x01, 0x04],
      H11DL::MaxSuperview => &[0x03, 0x79, 0x01, 0x07],
      H11DL::LinearHorizonLeveling => &[0x03, 0x79, 0x01, 0x08],
      H11DL::Hyperview => &[0x03, 0x79, 0x01, 0x09],
      H11DL::LinearHorizonLock => &[0x03, 0x79, 0x01, 0x0A],
    }
  }

  fn response_value_bytes(&self) -> &'static [u8] {
    [0x02, 0x79, 0x00]
  }
}


enum Hero11PhotoDigitalLense {
  MaxSuperview,
  Wide,
  Linear
}

impl AsRef<Hero11PhotoDigitalLense> for Hero11PhotoDigitalLense {
  fn as_ref(&self) -> &Hero11PhotoDigitalLense {
    self
  }
}

use Hero11PhotoDigitalLense as H11PDL;
impl<T> Sendable for T 
where 
  T: AsRef<H11PDL>
{
  fn as_bytes(&self) -> &'static [u8] {
    match self.as_ref() {
      H11PDL::MaxSuperview => &[0x03, 0x7A, 0x01, 0x64],
      H11PDL::Wide => &[0x03, 0x7A, 0x01, 0x65],
      H11PDL::Linear => &[0x03, 0x7A, 0x01, 0x66],
    }
  }

  fn response_value_bytes(&self) -> &'static [u8] {
    [0x02, 0x7A, 0x00]
  }
}

enum Hero11TimeLapseDigitalLense {
  MaxSuperview,
  Wide,
  Linear
}

impl AsRef<Hero11TimeLapseDigitalLense> for Hero11TimeLapseDigitalLense {
  fn as_ref(&self) -> &Hero11TimeLapseDigitalLense {
    self
  }
}

use Hero11TimeLapseDigitalLense as H11TDL;
impl<T> Sendable for T 
where 
  T: AsRef<H11TDL>
{
  fn as_bytes(&self) -> &'static [u8] {
    match self.as_ref() {
      H11TDL::MaxSuperview => &[0x03, 0x7B, 0x01, 0x64],
      H11TDL::Wide => &[0x03, 0x7B, 0x01, 0x65],
      H11TDL::Linear => &[0x03, 0x7B, 0x01, 0x66],
    }
  }

  fn response_value_bytes(&self) -> &'static [u8] {
    [0x02, 0x7B, 0x00]
  }
}

enum Hero11MediaFormat {
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
impl<T> Sendable for T 
where 
  T: AsRef<H11MF>
{
  fn as_bytes(&self) -> &'static [u8] {
    match self.as_ref() {
      H11MF::TimeLapseVideo => &[0x03, 0x80, 0x01, 0x0D],
      H11MF::TimeLapsePhoto => &[0x03, 0x80, 0x01, 0x14],
      H11MF::NightLapsePhoto => &[0x03, 0x80, 0x01, 0x15],
      H11MF::NightLapseVideo => &[0x03, 0x80, 0x01, 0x1A],
    }
  }

  fn response_value_bytes(&self) -> &'static [u8] {
    [0x02, 0x80, 0x00]
  }
}

enum AntiFlicker {
  SixtyHertz,
  FiftyHertz,
}

impl AsRef<AntiFlicker> for AntiFlicker {
  fn as_ref(&self) -> &AntiFlicker {
    self
  }
}

use AntiFlicker as AF;
impl<T> Sendable for T 
where 
  T: AsRef<AF>
{
  fn as_bytes(&self) -> &'static [u8] {
    match self.as_ref() {
      AF::SixtyHertz => &[0x03, 0x86, 0x01, 0x02],
      AF::FiftyHertz => &[0x03, 0x86, 0x01, 0x03],
    }
  }

  fn response_value_bytes(&self) -> &'static [u8] {
    [0x02, 0x86, 0x00]
  }
}

enum Hero11Hypersmooth {
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


