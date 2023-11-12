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
