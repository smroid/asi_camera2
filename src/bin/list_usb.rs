use rusb;

fn main() {
    for device in rusb::devices().unwrap().iter() {
        let device_desc = device.device_descriptor().unwrap();

        println!("Bus {:03} Device addr {:03} ID {{vend:prod}} {:04x}:{:04x}",
            device.bus_number(),
            device.address(),
            device_desc.vendor_id(),
            device_desc.product_id());
    }
}
