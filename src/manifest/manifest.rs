extern crate serde_json;

use std::fs::File;
use std::io::BufReader;

use super::data_set_sli_manifest::DataSetSLIManifest;

#[derive(Debug, Serialize, Deserialize)]
pub struct Manifest {
    #[serde(rename = "dataSetSLIManifest")]
    pub manifest: Option<DataSetSLIManifest>
}

impl Manifest {
    pub fn from_file(path: &str) -> Manifest {
        let br = BufReader::new(File::open(path).unwrap());
        let manifest: Manifest = serde_json::from_reader(br).unwrap();
        manifest
    }
}
