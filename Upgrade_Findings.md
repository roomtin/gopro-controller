# Upgrade Findings

## Scope

These notes summarize the current project state and the OpenGoPro documentation findings relevant to cleaning up this crate and adding home-server media upload/sync support for HERO11 Black cameras.

## Verified HERO11 Applicability

- Current OpenGoPro documentation lists HERO11 Black as supported with minimum firmware `v01.10.00`.
- HERO11 Black supports BLE control/query/settings through OpenGoPro.
- HERO11 Black supports HTTP access over WiFi AP and USB.
- USB HTTP support is documented as available from HERO11 onward.
- WiFi AP mode is applicable to HERO11 Black. The camera AP is typically reachable at `http://10.5.5.9:8080` once the host is connected to the camera WiFi.

## Important Limitation

OpenGoPro does not document a supported way to redirect GoPro Cloud auto-upload to a custom server.

The documented and practical approach for a home server is not camera-initiated cloud upload replacement. It should be a pull-based sync flow:

1. Use BLE to wake/connect to the camera.
2. Enable or bounce the camera WiFi AP.
3. Wait for AP status to report ready.
4. Connect the home server or controller host to the GoPro WiFi AP.
5. Use HTTP media APIs to list and download new files.
6. Store a local manifest so repeated syncs only fetch new media.

Anything that impersonates GoPro Cloud or attempts to alter the camera's cloud destination is outside the documented OpenGoPro API and is likely brittle.

## COHN / Home Network Notes

- OpenGoPro has documentation for Camera on the Home Network (COHN), but the current compatibility wording indicates station-mode command/control is HERO12 and later.
- Do not plan around COHN as the primary HERO11 Black solution.
- For HERO11 Black, prefer WiFi AP mode or USB HTTP for media transfer.

## Existing Project State

- The crate is currently a compact Rust library focused on BLE commands, settings, and queries.
- WiFi-related BLE UUID scaffolding exists, but there is no HTTP media layer yet.
- Tests currently include hardware-dependent camera tests that would run against a real GoPro by default.
- The code uses several `unwrap()` calls and stringly typed `Box<dyn Error>` errors that should be cleaned up before expanding functionality.
- Some public names contain typos, for example `QueryResponseIntepretation`.

## Recommended Cleanup Path

1. Split the current code into focused modules for BLE connection, commands, settings, query/status, HTTP media, and sync.
2. Replace string errors and `unwrap()` paths with a small typed error enum.
3. Move hardware-dependent tests behind `#[ignore]` or into explicit integration tests.
4. Add unit tests for byte serialization, UUIDs, query response parsing, and sync manifest behavior.
5. Keep the first cleanup pass minimal before adding new functionality.

## Recommended Feature Path

1. Add an optional HTTP/media feature using a Rust HTTP client and `serde`.
2. Implement typed support for media list, media info, last media, and file download.
3. Add BLE support for Set AP Control and a helper that waits for status `69` (`ApState`) to become ready.
4. Implement a sync manifest that records downloaded camera paths and file metadata.
5. Download files with retry-safe behavior, writing to a temporary file before atomic rename.
6. Add a CLI/example such as `sync --output /path/to/media`.

## Relevant Documented APIs / Operations

- BLE control, settings, query, and statuses are applicable to HERO11 Black.
- BLE networking includes Set AP Control for enabling/disabling/bouncing the camera WiFi AP.
- BLE status `69` (`ApState`) should be monitored after AP enable; the command response only confirms the request was accepted, not that WiFi is ready.
- HTTP media operations include media list and media download. The OpenGoPro HTTP docs expose operation IDs including `OGP_MEDIA_LIST`, `OGP_DOWNLOAD_MEDIA`, `OGP_GET_LAST_MEDIA`, and `OGP_SET_WIRED_USB_CONTROL`.

## Practical Architecture

For HERO11 Black home-server sync, the most reliable architecture is:

```text
home server/controller
  -> BLE: wake/connect and enable GoPro AP
  -> OS/network manager: connect to GoPro WiFi AP
  -> HTTP: list media from camera
  -> HTTP: download missing media
  -> local disk: update sync manifest
```

USB can be supported as a second transport path where the camera is physically connected to the server.
