use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct ProxmoxClient {
    http: Client,
    base_url: String,
    token_id: String,
    token_secret: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Node {
    pub node: String,
    pub status: String,
    pub upt: u64,
    pub maxcpu: u32,
    pub maxmem: u64,
    pub mem: u64,
    pub cpu: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClusterResource {
    pub r#type: String,
    pub node: Option<String>,
    pub vmid: Option<u64>,
    pub status: String,
    pub id: String,
    pub text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClusterStatus {
    pub name: String,
    pub type_: String,
    pub local: bool,
    pub level: Option<Vec<String>>,
    pub nodes: Option<Vec<ClusterNode>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClusterNode {
    pub node: String,
    pub ip: String,
    pub level: String,
    pub local: bool,
    pub online: bool,
}

#[derive(Debug, Deserialize)]
struct ApiResponse<T> {
    data: T,
}

impl ProxmoxClient {
    pub fn new(url: String, token_id: String, token_secret: String) -> Self {
        Self {
            http: Client::new(),
            base_url: url.trim_end_matches('/').to_string(),
            token_id,
            token_secret,
        }
    }

    fn auth_header(&self) -> String {
        format!("PVEAPIToken={}={}", self.token_id, self.token_secret)
    }

    pub async fn get_nodes(&self) -> Result<Vec<Node>, reqwest::Error> {
        let url = format!("{}/api2/json/nodes", self.base_url);
        let response = self.http
            .get(&url)
            .header("Authorization", self.auth_header())
            .send()
            .await?
            .json::<ApiResponse<Vec<Node>>>()
            .await?;
        Ok(response.data)
    }

    pub async fn get_cluster_status(&self) -> Result<Vec<ClusterStatus>, reqwest::Error> {
        let url = format!("{}/api2/json/cluster/status", self.base_url);
        let response = self.http
            .get(&url)
            .header("Authorization", self.auth_header())
            .send()
            .await?
            .json::<ApiResponse<Vec<ClusterStatus>>>()
            .await?;
        Ok(response.data)
    }

    pub async fn get_cluster_resources(&self) -> Result<Vec<ClusterResource>, reqwest::Error> {
        let url = format!("{}/api2/json/cluster/resources", self.base_url);
        let response = self.http
            .get(&url)
            .header("Authorization", self.auth_header())
            .send()
            .await?
            .json::<ApiResponse<Vec<ClusterResource>>>()
            .await?;
        Ok(response.data)
    }
}
