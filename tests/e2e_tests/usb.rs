use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::{thread, time};

use jcm::usb;
use jcm::{
    Error, Message, MessageCode, MessageData, MessageType, RequestCode, RequestType, ResponseCode,
    Result,
};

use super::common;

/// Dummy event responder that sends `ACK` responses to all device-sent events.
fn ack_event_responder(
    stop: Arc<AtomicBool>,
    event_recv: crossbeam::channel::Receiver<Message>,
    event_res_send: crossbeam::channel::Sender<Message>,
) -> Result<()> {
    thread::spawn(move || -> Result<()> {
        while !stop.load(Ordering::Relaxed) {
            if let Ok(event) = event_recv.try_recv() {
                event_res_send
                    .try_send(
                        Message::new().with_data(
                            event
                                .data()
                                .clone()
                                .with_additional(&[ResponseCode::Ack.into()]),
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

    let usb = Arc::new(Mutex::new(usb::UsbDeviceHandle::find_usb()?));
    let stop = Arc::new(AtomicBool::new(false));

    let (event_send, event_recv) = crossbeam::channel::unbounded();
    let (response_send, response_recv) = crossbeam::channel::unbounded();
    let (event_res_send, event_res_recv) = crossbeam::channel::unbounded();

    usb::poll_device_message(
        Arc::clone(&usb),
        Arc::clone(&stop),
        event_send,
        event_res_recv,
        response_send,
    )?;

    usb::wait_for_power_up(&event_recv, &event_res_send).ok();

    ack_event_responder(Arc::clone(&stop), event_recv, event_res_send)?;

    let uid_data = MessageData::new()
        .with_uid(0)
        .with_message_type(MessageType::Request(RequestType::SetFeature))
        .with_message_code(MessageCode::Request(RequestCode::Uid))
        .with_additional(&[0x1]);

    let req = Message::new().with_data(uid_data);
    let res = usb::poll_request(Arc::clone(&usb), &req, &response_recv, 3)?;

    log::info!("UID response: {res}");

    let stat_data = MessageData::new()
        .with_uid(1)
        .with_message_type(MessageType::Request(RequestType::Status))
        .with_message_code(MessageCode::Request(RequestCode::Status));

    let req = Message::new().with_data(stat_data);
    let res = usb::poll_request(Arc::clone(&usb), &req, &response_recv, 3)?;

    log::debug!("Raw status response: {res}");

    let res = jcm::StatusResponse::try_from(&res)?;

    log::info!("Status response: {res}");

    stop.store(true, Ordering::SeqCst);

    Ok(())
}

#[test]
fn test_full_startup() -> Result<()> {
    let _lock = common::init()?;

    let usb = Arc::new(Mutex::new(usb::UsbDeviceHandle::find_usb()?));
    let stop = Arc::new(AtomicBool::new(false));

    let (event_send, event_recv) = crossbeam::channel::unbounded();
    let (response_send, response_recv) = crossbeam::channel::unbounded();
    let (event_res_send, event_res_recv) = crossbeam::channel::unbounded();

    usb::poll_device_message(
        Arc::clone(&usb),
        Arc::clone(&stop),
        event_send,
        event_res_recv,
        response_send,
    )?;

    usb::wait_for_power_up(&event_recv, &event_res_send).ok();

    ack_event_responder(Arc::clone(&stop), event_recv, event_res_send)?;

    let uid_data = MessageData::new()
        .with_uid(0)
        .with_message_type(MessageType::Request(RequestType::SetFeature))
        .with_message_code(MessageCode::Request(RequestCode::Uid))
        .with_additional(&[0x1]);

    let req = Message::new().with_data(uid_data);
    let res = usb::poll_request(Arc::clone(&usb), &req, &response_recv, 3)?;

    log::info!("UID response: {res}");

    let stat_data = MessageData::new()
        .with_uid(1)
        .with_message_type(MessageType::Request(RequestType::Status))
        .with_message_code(MessageCode::Request(RequestCode::Status));

    let req = Message::new().with_data(stat_data);
    let res = usb::poll_request(Arc::clone(&usb), &req, &response_recv, 3)?;
    let res = jcm::StatusResponse::try_from(&res)?;

    log::info!("Status response: {res}");

    let reset_data = MessageData::new()
        .with_uid(1)
        .with_message_type(MessageType::Request(RequestType::Operation))
        .with_message_code(MessageCode::Request(RequestCode::Reset));

    let req = Message::new().with_data(reset_data);
    let res = usb::poll_request(Arc::clone(&usb), &req, &response_recv, 3)?;

    log::info!("Reset response: {res}");

    let stat_data = MessageData::new()
        .with_uid(1)
        .with_message_type(MessageType::Request(RequestType::Status))
        .with_message_code(MessageCode::Request(RequestCode::Status));

    let req = Message::new().with_data(stat_data);
    let res = usb::poll_request(Arc::clone(&usb), &req, &response_recv, 3)?;
    let res = jcm::StatusResponse::try_from(&res)?;

    log::info!("Status response: {res}");

    thread::sleep(time::Duration::from_millis(5000));

    let idle_data = MessageData::new()
        .with_uid(1)
        .with_message_type(MessageType::Request(RequestType::Operation))
        .with_message_code(MessageCode::Request(RequestCode::Idle));

    let req = Message::new().with_data(idle_data);
    let res = usb::poll_request(Arc::clone(&usb), &req, &response_recv, 3)?;

    log::info!("Idle response: {res}");

    stop.store(true, Ordering::SeqCst);

    Ok(())
}
