#[derive(serde::Deserialize, Debug, Clone)]
pub struct StartReq {
    pub client_prikey: String,
    pub client_pubkey: String,
    pub node_pubkey: String,
    pub node_addr: String,
    pub node_port: u16,
    pub protocol: String,
    pub iface_ipv4: String,
    pub iface_ipv6: String,
    pub dns: String,
}

extern "C" fn on_disconnected_callback(
    _data: *const libc::c_char,
    _error_message: *const libc::c_char,
) {
    // let data = unsafe { std::ffi::CStr::from_ptr(data) }.to_str().unwrap();
    let error_message = unsafe { std::ffi::CStr::from_ptr(_error_message) }
        .to_str()
        .unwrap();
    tracing::info!("[on_disconnected_callback] error_message: {error_message:#?}");
    if !error_message.is_empty() {
        let command = crate::service::node::command::command_tx_generator();
        let res = command.send(crate::service::node::command::Event::Disconnected(
            error_message.to_string(),
        ));
        tracing::info!("[on_disconnected_callback] command send res: {res:?}");
    }

    std::thread::spawn(move || {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        rt.block_on(async move {
            let dispatcher = rf_node_client_desktop::api::dispatcher_generator().await;
            let res = dispatcher.stop().await;
            tracing::info!("[on_disconnected_callback] stop: {res:#?}");
        });
    });
}

pub async fn connect(req: StartReq) -> String {
    #[cfg(target_os = "windows")]
    let path = crate::RESOURCE_PATH.get().cloned();
    let dns = req.dns;
    tracing::warn!("[connect] dns: {dns}");
    let req = rf_node_client_desktop::StartReq {
        client_prikey: req.client_prikey,
        node_pubkey: req.node_pubkey,
        node_addr: req.node_addr,
        node_port: req.node_port,
        tls: None,
        transport_protocol: req.protocol,
        crypto_protocol: "noise".to_string(),
        iface_ipv4: req.iface_ipv4,
        iface_ipv6: req.iface_ipv6,
        timeout: None,
        #[cfg(target_os = "windows")]
        path,
        fd: None,
        on_connected_callback: None,
        on_closed_callback: on_disconnected_callback,
    };
    rf_node_client_desktop::api::connect(req, &dns).await
}

pub async fn disconnect() -> String {
    rf_node_client_desktop::api::disconnect().await
}

pub mod tests {
    pub fn gen_connect_req() -> String {
        let req = serde_json::json!({
            "client_prikey": "f5e88fa4823fb9388cdae0fafb722fc8de6ba583dd0fff8a5cf5bdc18875d08e",
            "iface_ipv4": "10.11.0.1",
            "iface_ipv6": "",
            "node_addr": "52.221.222.252",
            "node_port": 7772,
            "node_pubkey": "51c8ebb98ea56210a5b339459385d9171b83374c54580578bdc331d982922b46",
            "protocol": "tcp"
        });
        req.to_string()
    }

    pub fn gen_jp_connect_req() -> String {
        let req = serde_json::json!({
            "client_prikey": "4073374e484df2411fe62c67a0635d715446e1cedf77248511602c24b81ee1cf",
            "node_pubkey": "1b974411f8c7c0220e8e3a37833265307c4c67125cfeb72e488f917cfeb7652f",
            "node_addr": "52.68.77.185:7777",
            "protocol": "tcp",
            "iface_ipv4": "10.11.11.2",
            "iface_ipv6": "fd86:ea04:1111::"
        });
        req.to_string()
    }

    pub fn gen_win_connect_req() -> String {
        let req = serde_json::json!({
            "client_prikey": "4073374e484df2411fe62c67a0635d715446e1cedf77248511602c24b81ee1cf",
            "node_pubkey": "1b974411f8c7c0220e8e3a37833265307c4c67125cfeb72e488f917cfeb7652f",
            "node_addr": "52.68.77.185:7777",
            "protocol": "tcp",
            "iface_ipv4": "10.11.11.2",
            "iface_ipv6": "fd86:ea04:1111::"
        });
        req.to_string()
    }

    pub fn gen_android_connect_req() -> String {
        let req = serde_json::json!({
            "client_prikey": "eea3583238abb83e15987c1c05e016460b99074d68ac58866a6456342d0b7427",
            "node_pubkey": "e6ee5d9228de85d0379753c60bb4680f0689829afba3dd114084cdc7e395af2d",
            "node_addr": "52.221.222.252",
            "node_port": 5173,
            "protocol": "tcp",
            "iface_ipv4": "10.0.1.3",
            "iface_ipv6": "fd86:ea04:1111::"
        });
        req.to_string()
    }

    pub fn gen_ios_connect_req() -> String {
        let req = serde_json::json!({
            "client_prikey": "1b34c764647274dfcc82d847db252715cf044cc1c9ed43b3886bd0c11db3f9e8",
            "node_pubkey": "e6ee5d9228de85d0379753c60bb4680f0689829afba3dd114084cdc7e395af2d",
            "node_addr": "52.221.222.252",
            "node_port": 5173,
            "protocol": "tcp",
            "iface_ipv4": "10.0.1.4",
            "iface_ipv6": "fd86:ea04:1111::"
        });
        req.to_string()
    }

    #[cfg(test)]
    pub mod test {
        #[tokio::test]
        async fn test_connect() {
            tokio::spawn(async move {
                tracing_subscriber::fmt()
                    .pretty()
                    .with_max_level(tracing::Level::DEBUG)
                    .with_writer(std::io::stdout)
                    .init();
                let req = crate::service::node::action::tests::gen_connect_req();
                let req: crate::service::node::action::StartReq =
                    serde_json::from_str(&req).unwrap();
                let res = crate::service::node::action::connect(req).await;
                tracing::debug!("res: {res}");
            });
            // assert!(false)
            tokio::signal::ctrl_c().await.unwrap()
        }
    }
}
