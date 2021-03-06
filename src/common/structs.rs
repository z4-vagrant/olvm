use std::error::Error as StdError;
use std::collections::HashMap;
use std::vec::Vec;

use serde_json;
use bson::{self, Document, Bson};

use common::{Result, Error};

fn default_i32() -> i32 {
    0
}

/*
 * Data structure to represent an image
 */
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Image {
    pub name: String,

    pub backend: String,

    #[serde(default = "default_i32")]
    pub node: i32,

    pub file: String,

    #[serde(default = "HashMap::new")]
    pub parameters: HashMap<String, String>
}

impl Image {
    pub fn from_json(s: &str) -> Result<Image> {
        match serde_json::from_str(s) {
            Ok(img) => Ok(img),
            Err(e) => Err(Error::new(format!("Failed to parse JSON into an Image structure: {}", e)))
        }
    }

    pub fn from_bson(doc: Document) -> Result<Image> {
        match bson::from_bson::<Image>(Bson::Document(doc)) {
            Ok(img) => Ok(img),
            Err(e) => Err(Error::new(e.description()))
        }
    }

    pub fn to_json(&self) -> Result<String> {
        let json = match serde_json::to_string(self) {
            Ok(json) => json,
            Err(e) => return Err(Error::new(e.description()))
        };

        Ok(json)
    }

    pub fn to_bson(&self) -> Result<Document> {
        let doc = match bson::to_bson(self) {
            Ok(bson) => try!(bson.as_document().ok_or(Error::new("Invalid document"))).clone(),
            Err(e) => return Err(Error::new(e.description()))
        };

        Ok(doc)
    }
}

/*
 * Data structure to represent a network interface attached to a VM
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct Interface {
    pub network: String, // Name of the network to connect the interface to
    pub ip: String, // Interface's IPv4 address

    #[serde(default = "String::new")]
    pub mac: String, // MAC address, set this to override the random default address
}

/*
 * Data structure to represent a vm in database
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct VM {
    pub name: String,

    #[serde(default = "default_i32")]
    pub node: i32,

    pub backend: String,

    #[serde(default = "String::new")]
    pub image: String, // Name of the image the VM is based on (if any)

    #[serde(default = "Vec::new")]
    pub interfaces: Vec<Interface>,

    #[serde(default = "HashMap::new")]
    pub parameters: HashMap<String, String>
}

impl VM {
    pub fn from_json(s: &str) -> Result<VM> {
        match serde_json::from_str(s) {
            Ok(vm) => Ok(vm),
            Err(e) => Err(Error::new(format!("Failed to parse JSON into a VM structure: {}", e)))
        }
    }

    pub fn from_bson(doc: Document) -> Result<VM> {
        match bson::from_bson::<VM>(Bson::Document(doc)) {
            Ok(vm) => Ok(vm),
            Err(e) => Err(Error::new(e.description()))
        }
    }

    pub fn to_json(&self) -> Result<String> {
        let json = match serde_json::to_string(self) {
            Ok(json) => json,
            Err(e) => return Err(Error::new(e.description()))
        };

        Ok(json)
    }

    pub fn to_bson(&self) -> Result<Document> {
        let doc = match bson::to_bson(self) {
            Ok(bson) => try!(bson.as_document().ok_or(Error::new("Invalid document"))).clone(),
            Err(e) => return Err(Error::new(e.description()))
        };

        Ok(doc)
    }
}

/*
 * Data structure to represent the recoverable saved state of a VM
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct Snapshot {
    #[serde(default = "String::new")]
    pub vm: String, // Name of the VM

    #[serde(default = "default_i32")]
    pub node: i32,

    #[serde(default = "String::new")]
    pub name: String // Name of the snapshot
}

impl Snapshot {
    pub fn from_json(s: &str) -> Result<Snapshot> {
        match serde_json::from_str(s) {
            Ok(snap) => Ok(snap),
            Err(e) => Err(Error::new(format!("Failed to parse JSON into a Snapshot structure: {}", e)))
        }
    }

    pub fn from_bson(doc: Document) -> Result<Snapshot> {
        match bson::from_bson::<Snapshot>(Bson::Document(doc)) {
            Ok(snap) => Ok(snap),
            Err(e) => Err(Error::new(e.description()))
        }
    }

    pub fn to_json(&self) -> Result<String> {
        let json = match serde_json::to_string(self) {
            Ok(json) => json,
            Err(e) => return Err(Error::new(e.description()))
        };

        Ok(json)
    }

    pub fn to_bson(&self) -> Result<Document> {
        let doc = match bson::to_bson(self) {
            Ok(bson) => try!(bson.as_document().ok_or(Error::new("Invalid document"))).clone(),
            Err(e) => return Err(Error::new(e.description()))
        };

        Ok(doc)
    }
}

/*
 * Data structure to represent a network
 */
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Network {
    pub name: String,

    #[serde(default = "default_i32")]
    pub node: i32,

    #[serde(default = "String::new")]
    pub cidr: String, // CIDR network address (example: 192.168.1.0/24)

    #[serde(default = "String::new")]
    pub router: String, // DHCP: Exit router

    #[serde(default = "Vec::new")]
    pub dns: Vec<String>, // DHCP: List of available nameservers

    #[serde(default = "String::new")]
    pub interface: String, // Optional: Gateway interface
}

impl Network {
    pub fn from_json(s: &str) -> Result<Network> {
        match serde_json::from_str(s) {
            Ok(net) => Ok(net),
            Err(e) => Err(Error::new(format!("Failed to parse JSON into an Network structure: {}", e)))
        }
    }

    pub fn from_bson(doc: Document) -> Result<Network> {
        match bson::from_bson::<Network>(Bson::Document(doc)) {
            Ok(net) => Ok(net),
            Err(e) => Err(Error::new(e.description()))
        }
    }

    pub fn to_json(&self) -> Result<String> {
        let json = match serde_json::to_string(self) {
            Ok(json) => json,
            Err(e) => return Err(Error::new(e.description()))
        };

        Ok(json)
    }

    pub fn to_bson(&self) -> Result<Document> {
        let doc = match bson::to_bson(self) {
            Ok(bson) => try!(bson.as_document().ok_or(Error::new("Invalid document"))).clone(),
            Err(e) => return Err(Error::new(e.description()))
        };

        Ok(doc)
    }
}
