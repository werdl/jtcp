use serde::{Deserialize, Serialize};
use serde_json::value::Serializer;
use serde_json;

#[derive(Serialize, Deserialize)]
pub struct JtcpData {
    pub data: Vec<u8>,
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = vec![0x1b, 0x2b].serialize(Serializer);

        println!("result: {:?}", result);
    }
}
