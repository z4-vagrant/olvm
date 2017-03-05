use std::error::Error as StdError;
use std::collections::HashMap;

use serde_json::{self};
use bson::{self, Document, Bson};

use common::{Result, Error};

fn default_node() -> i32 {
    0
}

/*
 * Data structure to represent an image
 */
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Image {
    pub name: String,

    #[serde(default = "default_node")]
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

    pub fn to_bson(&self) -> Result<Document> {
        let doc = match bson::to_bson(self) {
            Ok(bson) => try!(bson.as_document().ok_or(Error::new("Invalid document"))).clone(),
            Err(e) => return Err(Error::new(e.description()))
        };

        Ok(doc)
    }
}

/*
 * Data structure to represent a vm in database
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct VM {
    pub name: String,

    #[serde(default = "default_node")]
    pub node: i32,

    pub backend: String,

    #[serde(default = "String::new")]
    pub image: String, // Name of the image the VM is based on (if any)

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

    pub fn to_bson(&self) -> Result<Document> {
        let doc = match bson::to_bson(self) {
            Ok(bson) => try!(bson.as_document().ok_or(Error::new("Invalid document"))).clone(),
            Err(e) => return Err(Error::new(e.description()))
        };

        Ok(doc)
    }
}