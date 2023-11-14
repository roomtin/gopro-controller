mod services;
mod command;
mod settings;
use crate::services::{
    GoProControlAndQueryCharacteristics as GPCharac, GoProServices, ToUUID, Sendable
};
use crate::settings::GoProSetting;
use crate::command::GoProCommand;
use btleplug::api::{Central, Manager as _, Peripheral as _, ScanFilter, WriteType};
use btleplug::api::{CharPropFlags, ValueNotification};
use btleplug::platform::{Adapter, Manager, Peripheral};
use futures::stream::StreamExt;
use std::error::Error;

///Represents a connected GoPro device
pub struct GoPro {
    device: Peripheral,
}

impl GoPro {
    ///Sends a command to the GoPro without checking for a response
    pub async fn send_command_unchecked(&self, command: &GoProCommand) -> Result<(), Box<dyn Error>> {
        let characteristics = self.device.characteristics();

        let command_write_char = characteristics
            .iter()
            .find(|c| c.uuid == GPCharac::Command.to_uuid())
            .unwrap();

        self.device
            .write(
                &command_write_char,
                command.as_bytes(),
                WriteType::WithoutResponse,
            )
            .await?;

        Ok(())
    }

    ///Sends a command to the GoPro and checks for a response, erroring if the response is incorrect
    pub async fn send_command(&self, command: &GoProCommand) -> Result<(), Box<dyn Error>> {
        self.send_command_unchecked(command).await?;
        let res = self.get_next_notification().await?;
        if res.is_none() {
            return Err("No response from GoPro".into());
        }
        let res = res.unwrap();
        if res.uuid != GPCharac::CommandResponse.to_uuid() {
            return Err("Response from GoPro came from incorrect UUID".into());
        }
        if res.value != command.response_value_bytes() {
            return Err("Response from GoPro was incorrect".into());
        }
        Ok(())
    }

    ///Sends a setting to the GoPro without checking for a response
    pub async fn send_setting_unchecked(&self, setting: &GoProSetting) -> Result<(), Box<dyn Error>> {
        let characteristics = self.device.characteristics();

        let settings_write_char = characteristics
            .iter()
            .find(|c| c.uuid == GPCharac::Settings.to_uuid())
            .unwrap();

        self.device
            .write(
                &settings_write_char,
                setting.as_bytes(),
                WriteType::WithoutResponse,
            )
            .await?;

        Ok(())
    }

    ///Sends a setting to the GoPro and checks for a response, erroring if the response is incorrect
    pub async fn send_setting(&self, setting: &GoProSetting) -> Result<(), Box<dyn Error>> {
        self.send_setting_unchecked(setting).await?;
        let res = self.get_next_notification().await?;
        if res.is_none() {
            return Err("No response from GoPro".into());
        }
        let res = res.unwrap();
        if res.uuid != GPCharac::SettingsResponse.to_uuid() {
            return Err("Response from GoPro came from incorrect UUID".into());
        }
        if res.value != setting.response_value_bytes() {
            return Err("Response from GoPro was incorrect".into());
        }
        Ok(())
    }

    ///Gets the next notification (response from a command) from the GoPro
    pub async fn get_next_notification(&self) -> Result<Option<ValueNotification>, Box<dyn Error>> {
        let mut response_stream = self.device.notifications().await?;
        let notification = response_stream.next().await;
        Ok(notification)
    }

    ///Disconnects the GoPro
    pub async fn disconnect(self) -> Result<(), Box<dyn Error>> {
        self.device.disconnect().await?;
        Ok(())
    }

    ///Disconnects the GoPro and powers it off
    pub async fn disconnect_and_poweroff(self) -> Result<(), Box<dyn Error>> {
        self.send_command(GoProCommand::Sleep.as_ref()).await?;
        self.device.disconnect().await?;
        Ok(())
    }
}

///Inits the bluetooth adapter (central) and returns it to the caller
///
///@param adapter_index is an optional index into the list of bluetooth adapters in case the caller has more than one.
pub async fn init(adapter_index: Option<usize>) -> Result<Adapter, Box<dyn Error>> {
    let manager = Manager::new().await.unwrap();

    //manage multiple adapters ?
    let index = adapter_index.unwrap_or(0);
    // get the first bluetooth adapter
    let adapters = manager.adapters().await?;

    if adapters.len() <= 0 {
        return Err("No Bluetooth Adapters".into());
    }

    let central = adapters.into_iter().nth(index).unwrap();
    Ok(central)
}

///Scans for GoPro devices and returns a list of their names
///(may also return previously connected devices some of which may not be GoPros)
///
///@param central is the bluetooth adapter to use for scanning
pub async fn scan(central: &mut Adapter) -> Result<Vec<String>, Box<dyn Error>> {
    // start scanning for devices
    let scan_filter = ScanFilter {
        services: vec![GoProServices::ControlAndQuery.to_uuid()],
    };

    central.start_scan(scan_filter).await?;

    let mut devices_names: Vec<String> = Vec::with_capacity(central.peripherals().await?.len());

    for p in central.peripherals().await? {
        let properties = p.properties().await?;
        let name = properties
            .unwrap()
            .local_name
            .unwrap_or("Unknown".to_string());
        devices_names.push(name);
    }
    Ok(devices_names)
}

///
///Connects to a GoPro device by name and returns a GoPro object if successful
///
///@param gopro_local_name is the name of the GoPro device to connect to
///
///@param central is the bluetooth adapter to use for connecting
pub async fn connect(
    gopro_local_name: String,
    central: &mut Adapter,
) -> Result<GoPro, Box<dyn Error>> {
    let device = filter_peripherals(central.peripherals().await?, gopro_local_name).await?;
    if device.is_none() {
        return Err("GoPro not found".into());
    }
    let device = device.unwrap();

    // connect to the device
    device.connect().await?;

    //discover all the services on the device
    device.discover_services().await?;

    //subscribe to the proper notify characteristics
    let characteristics = device.characteristics();

    if characteristics.len() == 0 {
        return Err("No characteristics found on this GoPro".into());
    }

    //Subscribe to all the characteristics that have the notify property
    //TODO: Send off subscriptions concurently ?
    for c in &characteristics {
        if c.properties.bits() == CharPropFlags::NOTIFY.bits() {
            device.subscribe(&c).await?;
        }
    }

    Ok(GoPro { device })
}

///Filters a list of peripherals by name and returns the first one that matches
async fn filter_peripherals(
    peripherals: Vec<Peripheral>,
    device_name: String,
) -> Result<Option<Peripheral>, Box<dyn Error>> {
    for p in peripherals {
        let properties = p.properties().await?;
        let name = properties
            .unwrap()
            .local_name
            .unwrap_or("Unknown".to_string());
        if name.eq(&device_name) {
            return Ok(Some(p));
        }
    }
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    use tokio::time;
    
    use crate::settings::*;

    #[tokio::test]
    async fn test_whole_lifecycle() {
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
            .send_command(GoProCommand::ShutterStart.as_ref())
            .await
            .unwrap();

        time::sleep(Duration::from_secs(4)).await;
        println!("Adding HiLight");
        gopro
            .send_command(GoProCommand::AddHilightDuringEncoding.as_ref())
            .await
            .unwrap();

        time::sleep(Duration::from_secs(3)).await;
        println!("Stopping Shutter");
        gopro.send_command(GoProCommand::ShutterStop.as_ref()).await.unwrap();

        time::sleep(Duration::from_secs(4)).await;
        println!("Switching to Photo Mode");
        gopro.send_command(GoProCommand::PhotoMode.as_ref()).await.unwrap();

        time::sleep(Duration::from_secs(4)).await;
        println!("Switching to Timelapse Mode");
        gopro
            .send_command(GoProCommand::TimelapseMode.as_ref())
            .await
            .unwrap();

        time::sleep(Duration::from_secs(4)).await;
        println!("Disconnecting from GoPro");
        gopro.disconnect_and_poweroff().await.unwrap();
    }

    #[tokio::test]
    async fn test_some_settings() {
        let mut central = init(None).await.unwrap();
        let mut devices = scan(&mut central).await.unwrap();
        devices.retain(|d| d.contains("GoPro"));
        assert!(devices.len() > 0, "No GoPro devices found");

        let gopro = connect(devices.first().unwrap().clone(), &mut central)
            .await
            .unwrap();

        println!("Connected to GoPro");

        time::sleep(Duration::from_secs(4)).await;
        println!("Setting Video Lense to Linear + Horizon Lock");
        gopro
            .send_setting(GoProSetting::VideoDigitalLense(Hero11VideoDigitalLense::LinearHorizonLock).as_ref())
            .await
            .unwrap();

        time::sleep(Duration::from_secs(4)).await;
        println!("Setting Video Lense back to Hyperview");
        gopro
            .send_setting(GoProSetting::VideoDigitalLense(Hero11VideoDigitalLense::Hyperview).as_ref())
            .await
            .unwrap();

        time::sleep(Duration::from_secs(4)).await;
        println!("Powering off");
        gopro.disconnect_and_poweroff().await.unwrap();
    }

    #[tokio::test]
    #[ignore]
    async fn reset_testing() {
        let mut central = init(None).await.unwrap();
        let mut devices = scan(&mut central).await.unwrap();
        devices.retain(|d| d.contains("GoPro"));
        assert!(devices.len() > 0, "No GoPro devices found");

        let gopro = connect(devices.first().unwrap().clone(), &mut central)
            .await
            .unwrap();

        gopro.disconnect_and_poweroff().await.unwrap();
    }
}
