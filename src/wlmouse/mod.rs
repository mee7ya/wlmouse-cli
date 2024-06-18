use core::fmt;
use std::{thread::sleep, time::Duration};

use hidapi::{HidApi, HidDevice, HidError};

pub struct WLMouse {
    pub vendor_id: u16,
    pub product_id: u16,
    pub battery: u8,
    pub polling_rate: u16,
    pub manufacturer: String,
    pub product: String,
    device: HidDevice,
}

impl WLMouse {
    pub fn new(vendor_id: u16, product_id: u16) -> Result<WLMouse, HidError> {
        let api: HidApi = match HidApi::new() {
            Ok(api) => api,
            Err(error) => return Err(error),
        };
        let device: HidDevice = match api.open(vendor_id, product_id) {
            Ok(device) => device,
            Err(error) => return Err(error),
        };
        let manufacturer = match device.get_manufacturer_string() {
            Ok(manufacturer) => manufacturer.unwrap_or(String::new()),
            Err(error) => return Err(error),
        };
        let product = match device.get_product_string() {
            Ok(product) => product.unwrap_or(String::new()),
            Err(error) => return Err(error),
        };
        Ok(WLMouse {
            vendor_id,
            product_id,
            battery: 0,
            polling_rate: 0,
            manufacturer,
            product,
            device,
        })
    }

    /**
    Reads battery from a device. All bytes were gathered by inspecting packets.
    Device can sometimes output incorrect values, possibly repeat report multiple times?
    */
    pub fn get_battery(&mut self) -> () {
        let mut input_buffer = [0x00_u8; 65];
        input_buffer[3] = 0x02_u8;
        input_buffer[4] = 0x02_u8;
        input_buffer[6] = 0x83_u8; // battery byte

        let mut output_buffer = [0x00_u8; 65];

        let _ = self.device.send_feature_report(&input_buffer);
        sleep(Duration::from_millis(100)); // We have to sleep a bit, so device can prepare results
        let _ = self.device.get_feature_report(&mut output_buffer);
        self.battery = output_buffer[8];
    }

    /**
    Reads polling rate from a device. All bytes were gathered by inspecting packets.
    Device can sometimes output incorrect values, possibly repeat report multiple times?
    */
    pub fn get_polling_rate(&mut self) -> () {
        let mut input_buffer = [0x00_u8; 65];
        input_buffer[3] = 0x02_u8;
        input_buffer[4] = 0x02_u8;
        input_buffer[5] = 0x01_u8; // polling rate byte
        input_buffer[6] = 0x80_u8; // polling rate byte
        input_buffer[7] = 0x01_u8; // polling rate byte

        let mut output_buffer = [0x00_u8; 65];

        let _ = self.device.send_feature_report(&input_buffer);
        sleep(Duration::from_millis(100)); // We have to sleep a bit, so device can prepare results
        let _ = self.device.get_feature_report(&mut output_buffer);
        self.polling_rate = match output_buffer[8] {
            0x08 => 125,
            0x04 => 250,
            0x02 => 500,
            0x01 => 1000,
            0x20 => 2000,
            0x40 => 4000,
            0x80 => 8000,
            _ => 0,
        };
    }
}

impl fmt::Display for WLMouse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", { &self.product })
    }
}
