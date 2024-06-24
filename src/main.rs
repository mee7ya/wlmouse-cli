pub mod wlmouse;

use std::collections::HashMap;

use config::Config;
use wlmouse::WLMouse;

fn main() -> Result<(), String> {
    let config = match Config::builder()
        .add_source(config::File::with_name("./config"))
        .build()
    {
        Ok(config) => config,
        Err(error) => panic!("{:?}", error),
    };
    let config: HashMap<String, u16> = match config.try_deserialize::<HashMap<String, u16>>() {
        Ok(config) => config,
        Err(error) => panic!("{:?}", error),
    };

    let mut wlmouse: WLMouse = match WLMouse::new(
        *config.get("vendor_id").unwrap_or(&0),
        *config.get("product_id").unwrap_or(&0),
    ) {
        Ok(wlmouse) => wlmouse,
        Err(error) => panic!("{:?}", error),
    };

    println!("{wlmouse}");

    wlmouse.get_battery();
    println!("Battery: {}%", wlmouse.battery);

    // wlmouse.get_polling_rate();
    // println!("Polling rate: {}Hz", wlmouse.polling_rate);

    Ok(())
}
