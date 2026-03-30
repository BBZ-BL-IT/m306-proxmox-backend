use config::{Config, ConfigError, Environment};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Settings {
    pub role: String,
    pub user_group_templates: String,
    pub prefix_user_group: String,
    pub prefix_resourcepool: String,
    pub prefix_simple_zone: String,
    pub prefix_vnets: String,
    pub postfix_vnet_dmz: String,
    pub postfix_vnet_lan: String,
    pub prefix_firewall: String,
    pub vm_storage: String,
    pub template_storage: String,
    pub wan_interface: String,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub struct AppConfig {
    pub server_port: u16,
    pub proxmox_url: String,
    pub proxmox_token_id: String,
    pub proxmox_token_secret: String,
    pub ssl_verify: bool,
    pub ssl_cert_path: Option<String>,
    pub username_admin: Option<String>,
    pub password_admin: Option<String>,
    pub role: String,
    pub user_group_templates: String,
    pub prefix_user_group: String,
    pub prefix_resourcepool: String,
    pub prefix_simple_zone: String,
    pub prefix_vnets: String,
    pub postfix_vnet_dmz: String,
    pub postfix_vnet_lan: String,
    pub prefix_firewall: String,
    pub vm_storage: String,
    pub template_storage: String,
    pub wan_interface: String,
    pub cors_origin: Option<String>,
}

impl AppConfig {
    pub fn load() -> Result<Self, ConfigError> {
        let settings = Config::builder()
            .set_default("server_port", 3000)?
            .set_default("ssl_verify", true)?
            .set_default("role", "uro_bbzblit_PVELabUsers")?
            .set_default("user_group_templates", "ugr_bbzblit_Lernende")?
            .set_default("prefix_user_group", "ug")?
            .set_default("prefix_resourcepool", "rp")?
            .set_default("prefix_simple_zone", "sz")?
            .set_default("prefix_vnets", "vn")?
            .set_default("postfix_vnet_dmz", "DMZ")?
            .set_default("postfix_vnet_lan", "LAN")?
            .set_default("prefix_firewall", "fw")?
            .set_default("vm_storage", "pvecephpool01")?
            .set_default("template_storage", "templates")?
            .set_default("wan_interface", "vmbr1")?
            .add_source(Environment::with_prefix("APP").prefix_separator("_"))
            .build()?;

        settings.try_deserialize()
    }

    pub fn settings(&self) -> Settings {
        Settings {
            role: self.role.clone(),
            user_group_templates: self.user_group_templates.clone(),
            prefix_user_group: self.prefix_user_group.clone(),
            prefix_resourcepool: self.prefix_resourcepool.clone(),
            prefix_simple_zone: self.prefix_simple_zone.clone(),
            prefix_vnets: self.prefix_vnets.clone(),
            postfix_vnet_dmz: self.postfix_vnet_dmz.clone(),
            postfix_vnet_lan: self.postfix_vnet_lan.clone(),
            prefix_firewall: self.prefix_firewall.clone(),
            vm_storage: self.vm_storage.clone(),
            template_storage: self.template_storage.clone(),
            wan_interface: self.wan_interface.clone(),
        }
    }
}
