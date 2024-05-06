use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::{thread, time};

use jcm::{Error, Result};

use super::common;

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

#[test]
fn test_device_status() -> Result<()> {
    let _lock = common::init()?;

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

#[test]
fn test_full_startup() -> Result<()> {
    let _lock = common::init()?;

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
    let res = jcm::StatusResponse::try_from(&res)?;

    log::info!("Status response: {res}");

    let req: jcm::Message = jcm::MessageData::from(jcm::ResetRequest::new())
        .with_uid(1)
        .into();
    let res = jcm::usb::poll_request(Arc::clone(&usb), &req, &response_recv, 3)?;

    log::info!("Reset response: {res}");

    let req: jcm::Message = jcm::MessageData::from(jcm::StatusRequest::new())
        .with_uid(1)
        .into();
    let res = jcm::usb::poll_request(Arc::clone(&usb), &req, &response_recv, 3)?;
    let res = jcm::StatusResponse::try_from(&res)?;

    log::info!("Status response: {res}");

    thread::sleep(time::Duration::from_millis(5000));

    let req: jcm::Message = jcm::MessageData::from(jcm::IdleRequest::new())
        .with_uid(1)
        .into();
    let res = jcm::usb::poll_request(Arc::clone(&usb), &req, &response_recv, 3)?;

    log::info!("Idle response: {res}");

    let req: jcm::Message = jcm::MessageData::from(jcm::InhibitRequest::new())
        .with_uid(1)
        .into();
    let res = jcm::usb::poll_request(Arc::clone(&usb), &req, &response_recv, 3)?;

    log::info!("Inhibit response: {res}");

    stop.store(true, Ordering::SeqCst);

    Ok(())
}
