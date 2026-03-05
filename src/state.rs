use crate::clients::ProxmoxClient;

///
/// Here we will define the global appstate that will be shared
///
#[derive(Clone)]
pub struct State {
    pub proxmox: ProxmoxClient,
}
