# gopro-controller
Open source Rust Library for interacting with GoPro Cameras over BLE and (maybe eventually) WiFi

## GoPro Specification
https://gopro.github.io/OpenGoPro/

## Supported Cameras
- GoPro Hero 11 Black

I don't own any other models to test with so for now all I can verify for is the Hero 11 Black. The command structure looks the same for all cameras so it should work with any camera that supports the OpenGoPro spec, however the settings vary between cameras so I can't gurantee that any particular setting will work with a different model. 

## Features -- WIP
- [x] Connect* to GoPro Camera
- [x] Commands:
  - [x] Shutter Start
  - [x] Shutter Stop
  - [x] Power Off
  - [x] Add HiLight
  - [x] Change Modes
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
- [] Query Camera Status:
  - [ ] None Completed.
- [ ] Download media from camera
- [ ] Live preview
- [ ] WiFi support
- [ ] More Camera Models
- [ ] Protobuf support

*Note: At the moment the library cannot pair the camera for the first time. You must pair the camera with your system by putting the camera in pairing mode (Preferences -> Wireless Connections -> Connect Device -> GoPro Quik App) and connecting in your system's bluetooth settings. Once the camera is paired, you can connect to it from this library without pairing mode.

## Usage
At the moment usage can best be seen in the lifecycle test of the main lib.rs file: 

```rust
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
