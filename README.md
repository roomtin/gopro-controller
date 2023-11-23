# gopro-controller
Open source Rust Library for interacting with GoPro Cameras over BLE and WiFi

## Motivation
GoPros are neat little devices but the only good way of interacting with them is through their mobile app which isn't extensible. This crate hopes to provide a starting point for programatically controling the cameras wirelessly, and eventually downloading media from them, with DIY friendly computers such as Raspberry Pis and other SBCs, microcontrollers etc. There are, I think, a number of use cases for controling GoPros with sensors and other devices which are not supported by the official app. 

## Collaboration
### Code
I am open to collaboration on this crate. PRs/Issues welcome. 

### Testing other Models
Such nifty devices aren't inexpensive and so verifing that features that are expected to work on other models do in fact work could be a challenge. If you'd like to test any of the library features on a non Hero 11 Black model and make an issue with the results, this would be quite helpful. (Currently only the Commands are expected to work on non Hero 11 Black models.)

**Note however** that at the time of writing the open gopro spec supports only the following models: Hero 9 Black, Hero 10 Black, Hero 11 Black, Hero 11 Black Mini, Hero 12 Black 


## GoPro [Specification](https://gopro.github.io/OpenGoPro/)

## Supported Cameras
- GoPro Hero 11 Black

I don't own any other models to test with so for now all I can verify for is the Hero 11 Black. The command structure looks the same for all cameras so it should work with any camera that supports the OpenGoPro spec, however the settings vary between cameras so I can't gurantee that any particular setting will work with a different model. 

## Features -- WIP
- [x] Connect **(See Pairing Notes)*
- [x] Commands:
  - [x] Shutter Start
  - [x] Shutter Stop
  - [x] Power Off
  - [x] Add HiLight
  - [x] Change Modes
  - [ ] Complex Commands
- [x] Settings:
  - [x] Resolution
  - [x] FPS
  - [x] AutoPowerDown
  - [x] VideoDigitalLense 
  - [x] PhotoDigitalLense,
  - [x] TimeLapseDigitalLense,
  - [x] MediaFormat,
  - [x] AntiFlicker,
  - [x] Hypersmooth,
  - [x] HorizonLeveling,
  - [x] MaxLense,
  - [x] Hindsight,
  - [x] Controls,
  - [x] Speed,
  - [x] NightPhoto,
  - [x] WirelessBand,
  - [x] TrailLength,
  - [x] VideoMode,
- [x] Query Camera Status And Settings:
  - [x] There are a bunch. See [here](https://gopro.github.io/OpenGoPro/ble_2_0#status-ids).
- [x] Interpret Camera Statuses 
    - [x] Interpret Boolean statuses
    - [x] Interpret String statuses
    - [x] Interpret Integer statuses **(could be refined with enums)**
    - [x] Interpret Percentage statuses
    - [ ] Interpret complex camera statuses (byte arrays)
- [ ] Conect to camera's WiFi AP
- [ ] Download media from camera
- [ ] Control over wifi
- [ ] Live preview
- [ ] #no\_std support for embedded devices like esp32
- [ ] More Camera Models (Particularly the Hero Black 12 since it's what's new)
- [ ] Protobuf support


## Notes
### BLE
The BLE also seems kinda flakey on a cold start. Getting pairing to work (or even reconnecting after the GoPro has fully gone to sleep after 10 hours of inactivity) can require powering the camera off and on a few times or removing and reinserting the battery.

### Pairing
At the moment the library cannot pair the camera for the first time. You must pair the camera with your system by putting the camera in pairing mode (Preferences -> Wireless Connections -> Connect Device -> GoPro Quik App) and connecting in your system's bluetooth settings. Once the camera is paired, you can connect to it from this library without pairing mode.

### Auto-Wake
The camera will continue to send advertising packets for 10 hours after it is turned off. During this time, reconnecting to it will cause it to wake up and resume idling in the preset it was left in.

### Supported Platforms
This library is currently only tested on Linux, though it is not intended to be plaform specific.  

## Basic Usage

```rust
use gopro_controller::{connect, init, scan, GoProCommand};
use std::time::Duration;
use tokio::time;

let mut central = init(None).await.unwrap();
let mut devices = scan(&mut central).await.unwrap();
devices.retain(|d| d.contains("GoPro"));
assert!(devices.len() > 0, "No GoPro devices found");

let gopro = connect(devices.first().unwrap().clone(), &mut central)
  .await
  .unwrap();

println!("Connected to GoPro");

time::sleep(Duration::from_secs(4)).await;
println!("Starting Shutter");
gopro
  .send_command(GoProCommand::ShutterStart)
  .await
  .unwrap();

//Record for 3 Seconds
time::sleep(Duration::from_secs(3)).await;
println!("Stopping Shutter");
gopro.send_command(GoProCommand::ShutterStop).await.unwrap();

time::sleep(Duration::from_secs(2)).await;
println!("Powering Off");
gopro.disconnect_and_poweroff().await.unwrap();
```
