use core::fmt;
use std::{thread::sleep, time::Duration};

use hidapi::{HidApi, HidDevice, HidError};

#[repr(u16)]
pub enum WLMouseProduct {
    BeastX8KReceiver = 0xA883,
    BeastX8K = 0xA884,
    BeastX = 0xA888,
    BeastXReceiver = 0xA887,
    Unknown = 0x0,
}

impl TryFrom<u16> for WLMouseProduct {
    type Error = ();

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0xA883 => Ok(Self::BeastX8KReceiver),
            0xA884 => Ok(Self::BeastX8K),
            0xA888 => Ok(Self::BeastX),
            0xA887 => Ok(Self::BeastXReceiver),
            _ => Ok(Self::Unknown),
        }
    }
}

pub struct WLMouse {
    pub vendor_id: u16,
    pub product_id: u16,
    pub battery: u8,
    pub polling_rate: u16,
    pub product: WLMouseProduct,
    device: HidDevice,
}

impl WLMouse {
    pub fn new(vendor_id: u16, product_id: u16) -> Result<WLMouse, HidError> {
        let api: HidApi = match HidApi::new() {
            Ok(api) => api,
            Err(error) => return Err(error),
        };
        let product: WLMouseProduct =
            WLMouseProduct::try_from(product_id).unwrap_or(WLMouseProduct::Unknown);
        let device: HidDevice = match product {
            WLMouseProduct::BeastX8K | WLMouseProduct::BeastX8KReceiver => {
                match api.open(vendor_id, product_id) {
                    Ok(device) => device,
                    Err(error) => return Err(error),
                }
            }
            WLMouseProduct::BeastX | WLMouseProduct::BeastXReceiver => {
                let mut device: Option<HidDevice> = None;
                for device_info in api.device_list() {
                    if product_id == device_info.product_id() {
                        let _device = device_info.open_device(&api).unwrap();
                        let mut buff = [0x00_u8; 40];
                        _device.get_report_descriptor(&mut buff).unwrap();
                        if buff[0] == 0x06_u8 {
                            device = Some(_device);
                            break;
                        }
                    }
                }
                if device.is_none() {
                    return Err(HidError::HidApiError {
                        message: "Nowhere to read/write, everything is protected".to_string(),
                    });
                }
                device.unwrap()
            }
            _ => {
                return Err(HidError::HidApiError {
                    message: "Unknown product id".to_string(),
                });
            }
        };
        Ok(WLMouse {
            vendor_id,
            product_id,
            battery: 0,
            polling_rate: 0,
            product,
            device,
        })
    }

    /**
    Reads battery from a device. All bytes were gathered by inspecting packets.
    Device can sometimes output incorrect values, possibly repeat report multiple times?
    */
    pub fn get_battery(&mut self) -> () {
        match WLMouseProduct::try_from(self.product_id).unwrap() {
            WLMouseProduct::BeastX8KReceiver | WLMouseProduct::BeastX8K => {
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
            WLMouseProduct::BeastXReceiver | WLMouseProduct::BeastX => {
                let mut input_buffer = [0x00_u8; 64];
                input_buffer[0] = 0x04_u8;
                input_buffer[3] = 0xaa_u8;
                // input_buffer[3] = 0x1a_u8;
                self.device.write(&input_buffer).unwrap();
                let mut buf = [0x00_u8; 64];
                self.device.read(&mut buf).unwrap();
                sleep(Duration::from_millis(100));
                input_buffer[3] = 0x1a_u8;
                self.device.write(&input_buffer).unwrap();
                self.device.read(&mut buf).unwrap();
                self.battery = buf[8];
            }
            _ => (),
        }
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
        let product = match self.product {
            WLMouseProduct::BeastX8KReceiver => "WLMouse Beast X 8K Receiver",
            WLMouseProduct::BeastX8K => "WLMouse Beast X 8K",
            WLMouseProduct::BeastXReceiver => "WLMouse Beast X Receiver",
            WLMouseProduct::BeastX => "WLMouse Beast X",
            _ => "Unknown",
        };
        write!(f, "{}", product)
    }
}
