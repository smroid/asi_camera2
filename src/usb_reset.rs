pub mod usb_reset {
    use log::info;
    use rusb;

    /// Reset USB device(s) that match the `vendor_id` and the optional
    /// `product_id`.
    pub fn reset_usb_device(vendor_id: u16, product_id: Option<u16>)
                            -> Result<(), rusb::Error> {
        for device in rusb::devices().unwrap().iter() {
            let device_desc = device.device_descriptor().unwrap();
            if device_desc.vendor_id() != vendor_id {
                continue;
            }
            if product_id.is_some() && device_desc.product_id() != product_id.unwrap() {
                continue;
            }
            info!("Resetting USB device: Bus {:03} Addr {:03} ID {:04x}:{:04x}",
                  device.bus_number(),
                  device.address(),
                  device_desc.vendor_id(),
                  device_desc.product_id());
            let handle = device.open()?;
            handle.reset()?;
        }

        Ok(())
    }
}
