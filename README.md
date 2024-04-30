# JCM USB Communication protocol

This library is a pure Rust implementation of the JCM USB device communication protocol.

## Compatibility

This library is compatible with the following protocol specifications:

- ID-008

## Example usage

Below is an example startup routine:

```rust
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::{thread, time};

use jcm::{Error, Result};

/// Dummy event responder that sends `ACK` responses to all device-sent events.
fn ack_event_responder(
    stop: Arc<AtomicBool>,
    event_recv: crossbeam::channel::Receiver<jcm::Message>,
    event_res_send: crossbeam::channel::Sender<jcm::Message>,
) -> Result<()> {
    thread::spawn(move || -> Result<()> {
        while !stop.load(Ordering::Relaxed) {
            if let Ok(event) = event_recv.try_recv() {
                event_res_send
                    .try_send(
                        jcm::Message::new().with_data(
                            event
                                .data()
                                .clone()
                                .with_additional(&[jcm::ResponseCode::Ack.into()]),
                        ),
                    )
                    .map_err(|err| Error::Usb(format!("error sending event response: {err}")))?;
            }

            thread::sleep(time::Duration::from_millis(100));
        }
        Ok(())
    });

    Ok(())
}

fn main() -> Result<()> {
    env_logger::Builder::from_default_env()
        .format_timestamp_millis()
        .try_init()
        .ok();

    let usb = Arc::new(Mutex::new(jcm::usb::UsbDeviceHandle::find_usb()?));
    let stop = Arc::new(AtomicBool::new(false));

    let (event_send, event_recv) = crossbeam::channel::unbounded();
    let (response_send, response_recv) = crossbeam::channel::unbounded();
    let (event_res_send, event_res_recv) = crossbeam::channel::unbounded();

    jcm::usb::poll_device_message(
        Arc::clone(&usb),
        Arc::clone(&stop),
        event_send,
        event_res_recv,
        response_send,
    )?;

    jcm::usb::wait_for_power_up(&event_recv, &event_res_send).ok();

    ack_event_responder(Arc::clone(&stop), event_recv, event_res_send)?;

    let req: jcm::Message = jcm::MessageData::from(jcm::UidRequest::new_set(0x1)).into();
    let res = jcm::usb::poll_request(Arc::clone(&usb), &req, &response_recv, 3)?;

    log::info!("UID response: {res}");

    let req: jcm::Message = jcm::MessageData::from(jcm::StatusRequest::new())
        .with_uid(1)
        .into();
    let res = jcm::usb::poll_request(Arc::clone(&usb), &req, &response_recv, 3)?;

    log::debug!("Raw status response: {res}");

    let res = jcm::StatusResponse::try_from(&res)?;

    log::info!("Status response: {res}");

    stop.store(true, Ordering::SeqCst);

    Ok(())
}
```

## Helper functions

The following functions are helpers for common routines:

- `jcm::usb::poll_device_message`: polls for device-sent messages
- `jcm::usb::wait_for_power_up`: waits for `PowerUp` events on cross-thread channels
- `jcm::usb::poll_request`: polls sending a request message to the device for a given number of retries

Each of the functions are short and simple, so re-implementing them is fairly straight-forward.

For example, you may want to use different cross-thread channel primitives, mutex type, etc.
