use btleplug::api::{CharPropFlags, Characteristic};
use btleplug::api::{bleuuid::uuid_from_u16, Central, Manager as _, Peripheral as _, ScanFilter, WriteType};
use btleplug::platform::{Adapter, Manager, Peripheral};
use std::error::Error;
use std::thread;
use futures::stream::StreamExt;
use std::time::Duration;
use tokio::time;
use uuid::Uuid;

#[macro_export]
macro_rules! gp_uuid {
    ($x:expr) => {{
        // We use the `format!` macro to insert the provided digits into the UUID string.
        let s = format!("b5f9{}-aa8d-11e3-9046-0002a5d5c51b", $x);
        // Now we parse the string into a `Uuid` using the `parse_str` function from the `uuid` crate.
        // If parsing fails, the macro will panic at compile time.
        Uuid::parse_str(&s).expect("Invalid UUID")
    }};
}


//const LIGHT_CHARACTERISTIC_UUID: Uuid = uuid_from_u16(0xFFE9);
const GOPRO_SERVICE_UUID: Uuid = uuid_from_u16(0xFEA6);

pub async fn scan() -> Result<Vec<String>, Box<dyn Error>> {
    let manager = Manager::new().await.unwrap();

    // get the first bluetooth adapter
    //manage multiple adapters ? 
    let adapters = manager.adapters().await?;
    let central = adapters.into_iter().nth(0).unwrap();

    // start scanning for devices

    //TODO: Use the scan filter to narrow down the devices that might be GoPros ?

    let scan_filter = ScanFilter {
       services: vec![GOPRO_SERVICE_UUID] 
    };

    central.start_scan(scan_filter).await?;
    // instead of waiting, you can use central.events() to get a stream which will
    // notify you of new devices, for an example of that see examples/event_driven_discovery.rs
    time::sleep(Duration::from_secs(2)).await;

    let mut devices_names: Vec<String> = Vec::with_capacity(central.peripherals().await?.len());

    for p in central.peripherals().await? {
        let properties = p.properties().await?;
        let name = properties.unwrap().local_name.unwrap_or("Unknown".to_string());
        devices_names.push(name);
    }
    Ok(devices_names)

}

//connect
pub async fn connect(device_local_name: String) -> Result<(), Box<dyn Error>> {
    let manager = Manager::new().await.unwrap();

    // get the first bluetooth adapter
    //manage multiple adapters ? 
    let adapters = manager.adapters().await?;
    let central = adapters.into_iter().nth(0).unwrap();

    // start scanning for devices
    let scan_filter = ScanFilter {
       services: vec![GOPRO_SERVICE_UUID] 
    };

    central.start_scan(scan_filter).await?;

    let device = filter_peripherals(central.peripherals().await?, device_local_name).await?;

    if device.is_none() {
        return Err("Device not found".into());
    }

    let device = device.unwrap();

    // connect to the device
    device.connect().await?;

    //discover all the services on the device
    device.discover_services().await?;

    //subscribe to the proper notify characteristics
    let characteristics = device.characteristics();

    println!("{:#?}", characteristics);

    if characteristics.len() == 0 {
        return Err("No characteristics found on this device".into());
    }

    //Subscribe to all the characteristics that have the notify property
    //TODO: Send off subscriptions concurently
    for c in &characteristics {
        if c.properties.bits() == CharPropFlags::NOTIFY.bits() {
            device.subscribe(&c).await?;
        }
    }

    println!("Sleepin' 5");
    time::sleep(Duration::from_secs(5)).await;

    let command_write_char = characteristics.iter().find(|c| c.uuid == gp_uuid!("0072")).unwrap();

    println!("Writin' records");
    device.write(&command_write_char, &[0x03, 0x01, 0x01, 0x01], WriteType::WithoutResponse).await?;

    println!("Sleepin' 5");
    time::sleep(Duration::from_secs(5)).await;

    println!("Writin' Stop records");
    device.write(&command_write_char, &[0x03, 0x01, 0x01, 0x00], WriteType::WithoutResponse).await?;

    let mut response_stream = device.notifications().await?;

    println!("Listening for notifications...");
    while let Some(notification) = response_stream.next().await {
        println!("Received notification: {:?}", notification);
    }


    //subscribe to command write characteristics


    Ok(())
}

//disconnect
pub async fn disconnect(device_local_name: String) -> Result<(), Box<dyn Error>> {
    let manager = Manager::new().await.unwrap();

    // get the first bluetooth adapter
    //manage multiple adapters ? 
    let adapters = manager.adapters().await?;
    let central = adapters.into_iter().nth(0).unwrap();

    // start scanning for devices
    let scan_filter = ScanFilter {
       services: vec![GOPRO_SERVICE_UUID] 
    };

    central.start_scan(scan_filter).await?;

    let device = filter_peripherals(central.peripherals().await?, device_local_name).await?;

    if device.is_none() {
        return Err("Device not found".into());
    }

    let device = device.unwrap();

    //discover all the services on the device
    device.discover_services().await?;

    let characteristics = device.characteristics();

    let command_write_char = characteristics.iter().find(|c| c.uuid == gp_uuid!("0072")).unwrap();

    println!("Writin' sleeps");
    device.write(&command_write_char, &[0x01, 0x05], WriteType::WithoutResponse).await?;

    // disconnect to the device
    device.disconnect().await?;

    Ok(())
}

async fn filter_peripherals(peripherals: Vec<Peripheral>, device_name: String) -> Result<Option<Peripheral>, Box<dyn Error>> {
    for p in peripherals {
        let properties = p.properties().await?;
        let name = properties.unwrap().local_name.unwrap_or("Unknown".to_string());
        if name.eq(&device_name) {
            return Ok(Some(p))
        }
    }
    Ok(None)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_scan() {
        println!("{:?}",scan().await.unwrap());
    }


    #[tokio::test]
    async fn test_connect() {
        connect("GoPro 8323".to_string()).await.unwrap()
    }

    #[tokio::test]
    async fn test_disconnect() {
        disconnect("GoPro 8323".to_string()).await.unwrap()
    }
}
