pub static COMMAND_TX: once_cell::sync::Lazy<
    once_cell::sync::OnceCell<crossbeam_channel::Sender<Event>>,
    // once_cell::sync::OnceCell<tokio::sync::mpsc::UnboundedSender<Event>>,
> = once_cell::sync::Lazy::new(once_cell::sync::OnceCell::new);

// pub(crate) static LINK_FLAG: once_cell::sync::Lazy<std::sync::Arc<std::sync::RwLock<bool>>> =
//     once_cell::sync::Lazy::new(|| std::sync::Arc::new(std::sync::RwLock::new(false)));

pub fn command_tx_generator<'a>() -> &'a crossbeam_channel::Sender<Event> {
    // pub fn command_tx_generator<'a>() -> &'a tokio::sync::mpsc::UnboundedSender<Event> {
    let command_tx = COMMAND_TX.get_or_init(|| {
        let (command_tx, command_rx) = crossbeam_channel::unbounded::<Event>();
        // let (command_tx, command_rx) = tokio::sync::mpsc::unbounded_channel::<Event>();
        // let command_rx = tokio_stream::wrappers::UnboundedReceiverStream::new(command_rx);

        // Start a tokio task to handle the receiver
        std::thread::spawn(move || {
            // tokio::runtime::Builder::new_current_thread()
            //     .enable_all()
            //     .build()
            //     .unwrap()
            //     .block_on(async {
            command_recv(command_rx);
            //         })
        });
        command_tx
    });
    command_tx
}

pub(super) fn command_recv(command_rx: crossbeam_channel::Receiver<Event>) {
    // pub(super) async fn command_recv(
    //     mut command_rx: tokio_stream::wrappers::UnboundedReceiverStream<Event>,
    // ) {
    // use tokio_stream::StreamExt as _;
    while let Ok(event) = command_rx.recv() {
        use crate::service::node::notify::pack::Deliver as _;
        use crate::service::node::notify::pack::Pack as _;
        let (name, msg) = match event {
            Event::Disconnected(msg) => {
                if let Err(e) = crate::service::tauri::action::update_system_tray_icon(false) {
                    tracing::error!("[command_recv] update_system_tray_icon error: {e}");
                };
                ("Disconnected", msg)
                // let flag = crate::service::node::command::LINK_FLAG.read().unwrap();
                // if *flag {
                //     ("Disconnected", msg)
                // } else {
                //     continue;
                // }
            }
            Event::UpdateToken(msg) => ("UpdateToken", msg),
            Event::PublicDBInitialized => ("PublicDBInitialized", "".to_string()),
            Event::PublicDBUninitialized(msg) => ("PublicDBUninitialized", msg),
        };
        if let Err(e) = crate::service::node::notify::Notifies::pack_one(
            name,
            Some(crate::service::node::notify::payload::Payload::data(msg)),
        )
        .send()
        {
            tracing::error!("[command_recv] notify send error: {e}");
        };
        tracing::info!("[command_recv] notify send ok");
    }
}

pub enum Event {
    Disconnected(String),
    UpdateToken(String),
    PublicDBInitialized,
    PublicDBUninitialized(String),
}
