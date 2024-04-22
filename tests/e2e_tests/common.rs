use jcm::{Error, Result};
use std::sync::{Mutex, MutexGuard};

static INIT: Mutex<()> = Mutex::new(());

pub fn init() -> Result<MutexGuard<'static, ()>> {
    env_logger::Builder::from_default_env()
        .format_timestamp_millis()
        .try_init()
        .ok();
    Ok(INIT
        .lock()
        .map_err(|err| Error::Usb(format!("unable to lock e2e-test mutex: {err}")))?)
}
