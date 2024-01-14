use self::payload::Payload;
pub(crate) mod pack;
pub(crate) mod payload;

pub(crate) static NOTIFY_TX: once_cell::sync::Lazy<
    once_cell::sync::OnceCell<crossbeam_channel::Sender<Notify>>,
> = once_cell::sync::Lazy::new(once_cell::sync::OnceCell::new);

pub fn notify_tx_generator<'a>(window: tauri::Window) -> &'a crossbeam_channel::Sender<Notify> {
    let notify_tx = NOTIFY_TX.get_or_init(|| {
        let (notify_tx, notify_rx) = crossbeam_channel::unbounded::<Notify>();
        std::thread::spawn(|| handle(notify_rx, window));
        // Start a tokio task to handle the receiver
        notify_tx
    });
    notify_tx
}

#[derive(Debug)]
pub enum Notify {
    /// event, data
    SendResponse(String, Option<Payload>),
}

pub(super) enum Notifies {
    Single(Notify),
    _Multi(Vec<Notify>),
}

impl pack::Deliver for Notifies {
    fn send(self) -> Result<(), crate::Error> {
        match self {
            Notifies::Single(n) => n.send(),
            Notifies::_Multi(ns) => {
                for n in ns {
                    n.send()?
                }
                Ok(())
            }
        }
    }
}

impl pack::Pack for Notifies {
    fn pack_one(tag: &str, data: Option<self::payload::Payload>) -> Self {
        Notifies::Single(Notify::SendResponse(tag.to_owned(), data))
    }
}

impl pack::Deliver for Notify {
    fn send(self) -> Result<(), crate::Error> {
        NOTIFY_TX
            .get()
            .unwrap()
            .send(self)
            .map_err(|e| crate::Error::CommandChannelSendFailed(e.to_string()))
    }
}

impl pack::Pack for Notify {
    fn pack_one(tag: &str, data: Option<self::payload::Payload>) -> Self {
        Notify::SendResponse(tag.to_string(), data)
    }
}

pub(super) fn handle(notify_rx: crossbeam_channel::Receiver<Notify>, window: tauri::Window) {
    while let Ok(notify) = notify_rx.recv() {
        match notify {
            Notify::SendResponse(ref event, res) => {
                if let Err(e) = window.emit(event, res.clone()) {
                    tracing::error!("event send error: {e}");
                } else {
                    tracing::info!("send event successfully: {event}");
                }
            }
        }
    }
}
