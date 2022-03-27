use mongodb::sync::Client;

pub fn new() -> Client {
    let uri = "mongodb://root:72e827d2-505e-4e4d-bd02-a2d2d05d3f41@192.168.10.102:27017/iotDB?authSource=admin";
    let client = Client::with_uri_str(uri)
        .expect("failed to connect");
    return client;
}