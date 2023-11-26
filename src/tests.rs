use super::*;
use std::time::Duration;
use tokio::time;

#[cfg(feature = "settings")]
use crate::settings::*;

#[cfg(feature = "query")]
use crate::query::*;

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
    gopro
        .send_command(GoProCommand::ShutterStop.as_ref())
        .await
        .unwrap();

    time::sleep(Duration::from_secs(4)).await;
    println!("Switching to Photo Mode");
    gopro
        .send_command(GoProCommand::PhotoMode.as_ref())
        .await
        .unwrap();

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

#[cfg(feature = "query")]
#[tokio::test]
async fn test_queryies() {
    let mut central = init(None).await.unwrap();
    let mut devices = scan(&mut central).await.unwrap();
    devices.retain(|d| d.contains("GoPro"));
    assert!(devices.len() > 0, "No GoPro devices found");

    let gopro = connect(devices.first().unwrap().clone(), &mut central)
        .await
        .unwrap();

    println!("Connected to GoPro");

    time::sleep(Duration::from_secs(4)).await;
    println!("Querying Battery Percentage");
    let response = gopro
        .query(&GoProQuery::GetStatusValue(vec![
            StatusID::InternalBatteryPercentage,
        ]))
        .await
        .unwrap();
    println!("Reponse: {:?}", response);

    time::sleep(Duration::from_secs(4)).await;
    println!("Powering off");
    gopro.disconnect_and_poweroff().await.unwrap();
}

#[cfg(feature = "query")]
#[tokio::test]
async fn test_query_interp() {
    let mut central = init(None).await.unwrap();
    let mut devices = scan(&mut central).await.unwrap();
    devices.retain(|d| d.contains("GoPro"));
    assert!(devices.len() > 0, "No GoPro devices found");

    let gopro = connect(devices.first().unwrap().clone(), &mut central)
        .await
        .unwrap();

    println!("Connected to GoPro");

    // Test a query that returns a Percentage interpretation
    time::sleep(Duration::from_secs(4)).await;
    println!("Querying Battery Percentage");
    let response = gopro
        .interpreted_query(&GoProQuery::GetStatusValue(vec![
            StatusID::InternalBatteryPercentage,
        ]))
        .await
        .unwrap()
        .unwrap();
    println!("Percentage left: {:?}", response);

    // Test a query that returns a Bool interpretation
    time::sleep(Duration::from_secs(4)).await;
    println!("Querying System Hot");
    let response = gopro
        .interpreted_query(&GoProQuery::GetStatusValue(vec![StatusID::SystemHot]))
        .await
        .unwrap()
        .unwrap();
    println!("Too Hot?: {:?}", response);

    // Test a query that returns a Byte interpretation
    time::sleep(Duration::from_secs(4)).await;
    println!("Querying Actual Orientation");
    let response = gopro
        .interpreted_query(&GoProQuery::GetStatusValue(vec![
            StatusID::ActualOrientation,
        ]))
        .await
        .unwrap()
        .unwrap();
    println!(
        "Orientation (0:Upright 1:Upside 2:Right 3:Left): {:?}",
        response
    );

    // Test a query that returns a ByteVec interpretation
    time::sleep(Duration::from_secs(4)).await;
    println!("Querying Total SD Card Space Kb");
    let response = gopro
        .interpreted_query(&GoProQuery::GetStatusValue(vec![StatusID::TotalSDSpaceKB]))
        .await
        .unwrap()
        .unwrap();
    println!("Total Space in Kb: {:?}", response);

    // Test a query that returns a String interpretation
    time::sleep(Duration::from_secs(4)).await;
    println!("Querying AP SSID");
    let response = gopro
        .interpreted_query(&GoProQuery::GetStatusValue(vec![StatusID::ApSSID]))
        .await
        .unwrap()
        .unwrap();
    println!("ApSSID: {:?}", response);

    time::sleep(Duration::from_secs(4)).await;
    println!("Powering off");
    gopro.disconnect_and_poweroff().await.unwrap();
}

#[cfg(feature = "settings")]
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
        .send_setting(
            GoProSetting::VideoDigitalLense(Hero11VideoDigitalLense::LinearHorizonLock).as_ref(),
        )
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

use wifi_rs::prelude::*;
//use wifi_rs::prelude::Config;
use wifi_rs::WiFi;
#[cfg(feature = "wifi")]
#[tokio::test]
async fn test_connect_wifi() {

    //Aparently "None" is the default adapter
    let mut wifi = WiFi::new(None);

    match wifi.connect("BigLolz", "00042069" ) {
        Ok(pass_correct) => {
            if pass_correct {
                println!("Connected to WiFi");
            } else {
                println!("Password incorrect");
            }
        }
        Err(e) => {
            println!("Error connecting to WiFi: {:?}", e);
        }
    }
}

#[cfg(feature = "wifi")]
#[tokio::test]
async fn test_connect_ap() {
    let mut central = init(None).await.unwrap();
    let mut devices = scan(&mut central).await.unwrap();
    devices.retain(|d| d.contains("GoPro"));
    assert!(devices.len() > 0, "No GoPro devices found");

    let gopro = connect(devices.first().unwrap().clone(), &mut central)
        .await
        .unwrap();

    println!("Connected to GoPro");

    time::sleep(Duration::from_secs(4)).await;
    println!("Connecting to AP");
    gopro
        .connect_to_ap()
        .await
        .unwrap();

    println!("Done.");
}

#[tokio::test]
//#[ignore]
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
