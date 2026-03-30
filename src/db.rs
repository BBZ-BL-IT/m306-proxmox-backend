use rusqlite::{Connection, Result, params};

use crate::config::Settings;

pub fn init_db(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS settings (
            id INTEGER PRIMARY KEY CHECK (id = 1),
            role TEXT NOT NULL,
            user_group_templates TEXT NOT NULL,
            prefix_user_group TEXT NOT NULL,
            prefix_resourcepool TEXT NOT NULL,
            prefix_simple_zone TEXT NOT NULL,
            prefix_vnets TEXT NOT NULL,
            postfix_vnet_dmz TEXT NOT NULL,
            postfix_vnet_lan TEXT NOT NULL,
            prefix_firewall TEXT NOT NULL,
            vm_storage TEXT NOT NULL,
            template_storage TEXT NOT NULL,
            wan_interface TEXT NOT NULL
        )",
    )?;
    Ok(())
}

pub fn load_settings(conn: &Connection) -> Result<Option<Settings>> {
    let mut stmt = conn.prepare(
        "SELECT role, user_group_templates, prefix_user_group, prefix_resourcepool,
                prefix_simple_zone, prefix_vnets, postfix_vnet_dmz, postfix_vnet_lan,
                prefix_firewall, vm_storage, template_storage, wan_interface
         FROM settings WHERE id = 1",
    )?;

    let result = stmt.query_row([], |row| {
        Ok(Settings {
            role: row.get(0)?,
            user_group_templates: row.get(1)?,
            prefix_user_group: row.get(2)?,
            prefix_resourcepool: row.get(3)?,
            prefix_simple_zone: row.get(4)?,
            prefix_vnets: row.get(5)?,
            postfix_vnet_dmz: row.get(6)?,
            postfix_vnet_lan: row.get(7)?,
            prefix_firewall: row.get(8)?,
            vm_storage: row.get(9)?,
            template_storage: row.get(10)?,
            wan_interface: row.get(11)?,
        })
    });

    match result {
        Ok(settings) => Ok(Some(settings)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(e),
    }
}

pub fn save_settings(conn: &Connection, settings: &Settings) -> Result<()> {
    conn.execute(
        "INSERT OR REPLACE INTO settings (id, role, user_group_templates, prefix_user_group,
            prefix_resourcepool, prefix_simple_zone, prefix_vnets, postfix_vnet_dmz,
            postfix_vnet_lan, prefix_firewall, vm_storage, template_storage, wan_interface)
         VALUES (1, ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
        params![
            settings.role,
            settings.user_group_templates,
            settings.prefix_user_group,
            settings.prefix_resourcepool,
            settings.prefix_simple_zone,
            settings.prefix_vnets,
            settings.postfix_vnet_dmz,
            settings.postfix_vnet_lan,
            settings.prefix_firewall,
            settings.vm_storage,
            settings.template_storage,
            settings.wan_interface,
        ],
    )?;
    Ok(())
}
