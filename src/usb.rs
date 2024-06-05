use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::{thread, time};

use futures_lite::future::block_on;
use nusb::transfer::{ControlOut, ControlType, Recipient, RequestBuffer};
use smol_timeout::TimeoutExt;

use crate::{Error, Message, ResponseCode, Result};

mod endpoint;

pub use endpoint::*;

pub const JCM_VID: u16 = 0x2475;
pub const JCM_PID: u16 = 0x0105;

/// USB communication timeout (ms)
pub const USB_TIMEOUT: u64 = 100;

/// Represents a host-side USB device handle.
pub struct UsbDeviceHandle {
    device: nusb::Device,
    interface: nusb::Interface,
    req_ep: Endpoint,
    res_ep: Endpoint,
}

impl UsbDeviceHandle {
    /// Finds the JCM XFS USB device by PID:VID pair.
    pub fn find_usb() -> Result<Self> {
        let info = nusb::list_devices()
            .map_err(|err| {
                Error::Usb(format!("no devices found: {err}"))
            })?
        .find(|dev| {
            dev.vendor_id() == JCM_VID && dev.product_id() == JCM_PID
        })
        .ok_or(Error::Usb(format!("failed to find a USB device with the correct VID({JCM_VID:04x}):PID({JCM_PID:04x}) pair")))?;

        let device = info
            .open()
            .map_err(|err| Error::Usb(format!("unable to open device: {err}")))?;

        Self::setup_device(&device).map_err(|err| {
            log::error!("Device setup failed: {err}");
            err
        })?;

        let (interface, req_ep, res_ep) = Self::find_interface(&device)?;

        Ok(Self {
            device,
            interface,
            req_ep,
            res_ep,
        })
    }

    /// Gets a reference to the USB [`Device`](nusb::Device).
    pub const fn device(&self) -> &nusb::Device {
        &self.device
    }

    /// Gets a reference to the USB [`Interface`](nusb::Interface).
    pub const fn interface(&self) -> &nusb::Interface {
        &self.interface
    }

    /// Writes a request [Message] to the JCM device.
    pub fn write_request(&self, message: &Message) -> Result<()> {
        block_on(
            self.interface
                .bulk_out(self.req_ep.address(), message.into())
                .timeout(time::Duration::from_millis(USB_TIMEOUT)),
        )
        .ok_or(Error::Usb("write Request timeout expired".into()))?
        .into_result()
        .map(|_| ())
        .map_err(|err| {
            let err_msg =
                format!(r#"error writing message: {{"message": {message}, "error": {err}}}"#);
            log::warn!("{err_msg}");
            Error::Usb(err_msg)
        })
    }

    /// Reads the response from a JCM device.
    pub fn read_response(&self) -> Result<Message> {
        let mut res_acc = Vec::with_capacity(self.res_ep.max_packet_size());
        let mut res_buf = block_on(
            self.interface
                .bulk_in(
                    self.res_ep.address(),
                    RequestBuffer::new(self.res_ep.max_packet_size()),
                )
                .timeout(time::Duration::from_millis(USB_TIMEOUT)),
        )
        .ok_or(Error::Usb("read Response timeout expired".into()))?
        .into_result()
        .map_err(|err| {
            let err_msg = format!("Error reading response: {err}");
            log::error!("{err_msg}");
            Error::Usb(err_msg)
        })?;

        let mut read = res_buf.len();
        res_acc.append(&mut res_buf);
        while read == self.res_ep.max_packet_size() {
            // clear the buffer to avoid leaving old data in the trailing bytes
            res_buf = match block_on(
                self.interface
                    .bulk_in(
                        self.res_ep.address(),
                        RequestBuffer::reuse(res_buf, self.res_ep.max_packet_size()),
                    )
                    .timeout(time::Duration::from_millis(USB_TIMEOUT)),
            )
            .ok_or(Error::Usb(
                "read Response follow-on packet timeout expired".into(),
            ))?
            .into_result()
            {
                Ok(r) => r,
                Err(_err) => Vec::new(),
            };
            read = res_buf.len();
            if read > 0 {
                res_acc.append(&mut res_buf);
            }
        }

        log::trace!("Raw response: {res_acc:?}");
        match Message::try_from(res_acc.as_slice()) {
            Ok(msg) => Ok(msg),
            Err(err) => {
                log::error!("Error parsing response: {err}");
                Err(err)
            }
        }
    }

    /// Reads an event [Message] from the JCM device.
    pub fn read_event(&self) -> Result<Message> {
        let mut res_acc = Vec::with_capacity(self.res_ep.max_packet_size());
        let mut res_buf = block_on(
            self.interface
                .bulk_in(
                    self.res_ep.address(),
                    RequestBuffer::new(self.res_ep.max_packet_size()),
                )
                .timeout(time::Duration::from_millis(USB_TIMEOUT)),
        )
        .ok_or(Error::Usb("read Event timeout expired".into()))?
        .into_result()
        .map_err(|err| {
            let err_msg = format!("Error reading response: {err}");
            log::error!("{err_msg}");
            Error::Usb(err_msg)
        })?;

        let mut read = res_buf.len();
        res_acc.append(&mut res_buf);
        while read == self.res_ep.max_packet_size() {
            // clear the buffer to avoid leaving old data in the trailing bytes
            res_buf = match block_on(
                self.interface
                    .bulk_in(
                        self.res_ep.address(),
                        RequestBuffer::reuse(res_buf, self.res_ep.max_packet_size()),
                    )
                    .timeout(time::Duration::from_millis(USB_TIMEOUT)),
            )
            .ok_or(Error::Usb(
                "read Event follow-on packet timeout expired".into(),
            ))?
            .into_result()
            {
                Ok(r) => r,
                Err(_err) => Vec::new(),
            };
            read = res_buf.len();
            if read > 0 {
                res_acc.append(&mut res_buf);
            }
        }

        log::trace!("Raw response: {res_acc:?}");
        match Message::try_from(res_acc.as_slice()) {
            Ok(msg) => Ok(msg),
            Err(err) => {
                log::error!("Error parsing response: {err}");
                Err(err)
            }
        }
    }

    /// Writes an event response [Message] to the JCM device.
    pub fn write_event_response(&self, message: &Message) -> Result<()> {
        block_on(
            self.interface
                .bulk_out(self.req_ep.address(), message.into())
                .timeout(time::Duration::from_millis(USB_TIMEOUT)),
        )
        .ok_or(Error::Usb("write Event response timeout expired".into()))?
        .into_result()
        .map(|_| ())
        .map_err(|err| {
            let err_msg =
                format!(r#"error writing message: {{"message": {message}, "error": {err}}}"#);
            log::warn!("{err_msg}");
            Error::Usb(err_msg)
        })
    }

    fn setup_device(device: &nusb::Device) -> Result<()> {
        block_on(
            device
                .control_out(ControlOut {
                    control_type: ControlType::Class,
                    recipient: Recipient::Interface,
                    request: 0x22,
                    value: 0b00,
                    index: 0x0,
                    data: &[],
                })
                .timeout(time::Duration::from_millis(USB_TIMEOUT)),
        )
        .map(|_| ())
        .ok_or(Error::Usb("device setup timeout expired".into()))
    }

    fn find_interface(device: &nusb::Device) -> Result<(nusb::Interface, Endpoint, Endpoint)> {
        let mut iface = None;
        let mut req_ep = None;
        let mut res_ep = None;

        device.configurations().for_each(|cfg| {
            cfg.interfaces().for_each(|cfg_iface| {
                cfg_iface.alt_settings().for_each(|alt| {
                    alt.endpoints()
                        .for_each(|ep| match (ep.transfer_type(), ep.direction()) {
                            (
                                nusb::transfer::EndpointType::Bulk,
                                nusb::transfer::Direction::Out,
                            ) => {
                                if iface.is_none() {
                                    iface = Some(cfg_iface.interface_number());
                                }

                                if req_ep.is_none() {
                                    req_ep =
                                        Some(Endpoint::create(ep.address(), ep.max_packet_size()));
                                }
                            }
                            (nusb::transfer::EndpointType::Bulk, nusb::transfer::Direction::In) => {
                                if iface.is_none() {
                                    iface = Some(cfg_iface.interface_number());
                                }

                                if res_ep.is_none() {
                                    res_ep =
                                        Some(Endpoint::create(ep.address(), ep.max_packet_size()));
                                }
                            }
                            _ => (),
                        });
                });
            });
        });

        let interface = device
            .claim_interface(iface.ok_or(Error::Usb("unable to find matching interface".into()))?)
            .map_err(|err| Error::Usb(format!("unable to open main interface: {err}")))?;

        match (req_ep, res_ep) {
            (Some(req_ep), Some(res_ep)) => Ok((interface, req_ep, res_ep)),
            (None, _) => Err(Error::Usb(
                "unable to find matching request endpoint".into(),
            )),
            (_, None) => Err(Error::Usb(
                "unable to find matching response endpoint".into(),
            )),
        }
    }
}

/// Polls for device-sent [Message]s.
///
/// # Example
///
/// ```no_run
/// use std::sync::{Arc, Mutex};
/// use std::sync::atomic::{AtomicBool, Ordering};
/// use jcm::usb;
///
/// # pub fn main() -> jcm::Result<()> {
/// let usb = Arc::new(Mutex::new(jcm::usb::UsbDeviceHandle::find_usb()?));
/// let stop = Arc::new(AtomicBool::new(false));
///
/// let (event_send, event_recv) = crossbeam::channel::unbounded();
/// let (response_send, response_recv) = crossbeam::channel::unbounded();
/// let (event_res_send, event_res_recv) = crossbeam::channel::unbounded();
///
/// jcm::usb::poll_device_message(
///     Arc::clone(&usb),
///     Arc::clone(&stop),
///     event_send,
///     event_res_recv,
///     response_send,
/// )?;
///
/// # Ok(())
/// # }
/// ```
pub fn poll_device_message(
    usb_handle: Arc<Mutex<UsbDeviceHandle>>,
    stop: Arc<AtomicBool>,
    event_send: crossbeam::channel::Sender<Message>,
    event_res_rcv: crossbeam::channel::Receiver<Message>,
    response_send: crossbeam::channel::Sender<Message>,
) -> Result<()> {
    thread::spawn(move || -> Result<()> {
        while !stop.load(Ordering::Relaxed) {
            match usb_handle.lock() {
                Ok(usb) => match usb.read_response() {
                    Ok(msg) if msg.data().message_type().is_event() => {
                        event_send
                            .send(msg)
                            .map_err(|err| Error::Usb(format!("error sending event: {err}")))?;

                        let res = event_res_rcv.recv().map_err(|err| {
                            Error::Usb(format!("error receiving event response: {err}"))
                        })?;

                        usb.write_event_response(&res)?;
                    }
                    Ok(msg) => response_send
                        .send(msg)
                        .map_err(|err| Error::Usb(format!("error sending response: {err}")))?,
                    Err(err) => log::trace!("No device-sent message available: {err}"),
                },
                Err(err) => log::warn!("unable to lock USB: {err}"),
            }

            thread::sleep(time::Duration::from_millis(100));
        }

        Ok(())
    });

    Ok(())
}

/// Waits for the device to finish sending `Power Up` events at startup.
///
/// # Example
///
/// ```no_run
/// use std::sync::{Arc, Mutex};
/// use std::sync::atomic::{AtomicBool, Ordering};
///
/// # pub fn main() -> jcm::Result<()> {
/// let usb = Arc::new(Mutex::new(jcm::usb::UsbDeviceHandle::find_usb()?));
/// let stop = Arc::new(AtomicBool::new(false));
///
/// let (event_send, event_recv) = crossbeam::channel::unbounded();
/// let (response_send, response_recv) = crossbeam::channel::unbounded();
/// let (event_res_send, event_res_recv) = crossbeam::channel::unbounded();
///
/// jcm::usb::poll_device_message(
///     Arc::clone(&usb),
///     Arc::clone(&stop),
///     event_send,
///     event_res_recv,
///     response_send,
/// )?;
///
/// jcm::usb::wait_for_power_up(&event_recv, &event_res_send)?;
///
/// # Ok(())
/// # }
/// ```
pub fn wait_for_power_up(
    event_recv: &crossbeam::channel::Receiver<Message>,
    event_res_send: &crossbeam::channel::Sender<Message>,
) -> Result<()> {
    let mut powerup = false;
    let mut powerup_count = 0;

    let now = time::Instant::now();

    while now.elapsed() <= time::Duration::from_secs(3) && !powerup {
        match event_recv.recv_timeout(time::Duration::from_secs(1)) {
            Ok(evt) if evt.data().message_code().is_power_up_event() => {
                powerup_count += 1;

                log::info!("receive Power Up event: {evt}");

                event_res_send
                    .send(
                        Message::new().with_data(
                            evt.data()
                                .clone()
                                .with_additional(&[ResponseCode::Ack.into()]),
                        ),
                    )
                    .unwrap();
            }
            Ok(evt) => {
                log::debug!("received unexpected event: {evt}");

                event_res_send
                    .send(
                        Message::new().with_data(
                            evt.data()
                                .clone()
                                .with_additional(&[ResponseCode::Ack.into()]),
                        ),
                    )
                    .unwrap();

                powerup = true;
            }
            Err(err) => {
                log::info!("end of events: {err}");
                powerup = true;
            }
        }
    }

    if powerup_count == 0 {
        Err(Error::Usb("no `Power Up` event before timeout".into()))
    } else {
        Ok(())
    }
}

/// Polls a request [Message] from the host to the device.
///
/// # Example
///
/// ```no_run
/// use std::sync::{Arc, Mutex};
/// use std::sync::atomic::{AtomicBool, Ordering};
///
/// # pub fn main() -> jcm::Result<()> {
/// let usb = Arc::new(Mutex::new(jcm::usb::UsbDeviceHandle::find_usb()?));
/// let stop = Arc::new(AtomicBool::new(false));
///
/// let (event_send, event_recv) = crossbeam::channel::unbounded();
/// let (response_send, response_recv) = crossbeam::channel::unbounded();
/// let (event_res_send, event_res_recv) = crossbeam::channel::unbounded();
///
/// jcm::usb::poll_device_message(
///     Arc::clone(&usb),
///     Arc::clone(&stop),
///     event_send,
///     event_res_recv,
///     response_send,
/// )?;
///
/// let uid_data = jcm::MessageData::new()
///     .with_uid(0)
///     .with_message_type(jcm::MessageType::Request(jcm::RequestType::SetFeature))
///     .with_message_code(jcm::MessageCode::Request(jcm::RequestCode::Uid))
///     .with_additional(&[0x1]);
///
/// let req = jcm::Message::new().with_data(uid_data);
///
/// let retries = 3;
/// let _res = jcm::usb::poll_request(Arc::clone(&usb), &req, &response_recv, retries)?;
///
/// # Ok(())
/// # }
/// ```
pub fn poll_request(
    usb: Arc<Mutex<UsbDeviceHandle>>,
    request: &Message,
    response_recv: &crossbeam::channel::Receiver<Message>,
    retries: usize,
) -> Result<Message> {
    let code = request.data().message_code().request_code()?;

    for retry in 0..retries {
        log::debug!("Sending {code} request, attempt: {retry}...");

        match usb.lock() {
            Ok(usb_lock) => {
                if let Err(err) = usb_lock.write_request(request) {
                    log::warn!("error sending message: {err}");
                } else {
                    match response_recv.recv_timeout(time::Duration::from_millis(500)) {
                        Ok(res) if res.data().message_code().request_code() == Ok(code) => {
                            return Ok(res)
                        }
                        Ok(res) => log::warn!("unexpected response: {res}"),
                        Err(err) => {
                            log::warn!("error receiving {code} response: {err}, retry: {retry}")
                        }
                    }
                }
            }
            Err(err) => {
                log::warn!("error locking USB: {err}");
            }
        }

        thread::sleep(time::Duration::from_millis(100));
    }

    Err(Error::Usb(format!(
        "receiving response failed after {retries} retries"
    )))
}
