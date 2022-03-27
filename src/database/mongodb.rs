use mongodb::sync::Client;

pub fn new() -> Client {
    let uri = "mongodb://root:7fd7583b-f01e-4a5b-869c-01c06ea45cda@192.168.10.102:27017/iotDB?authSource=admin";
    let client = Client::with_uri_str(uri)
        .expect("failed to connect");
    return client;
}