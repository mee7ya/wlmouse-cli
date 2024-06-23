# wlmouse-cli
CLI for diplaying information about wlmouse using `hidapi`

## Supported Devices
* [❔] **Beast X Mini** - I don't own it. There is a good chance it also uses feature reports and potentially compatible with Max/BeastX 8K
* [❌] **Beast X** - requires direct reading from interrupt endpoint, Windows security measures prohibit from opening mice virtual file for read/write,  currently looking for ways to bypass that
* [✔️] **Beast X 8K** - originally developed with it in mind. Done through feature reports
* [❔] **Beast X Max** - I don't own it. There is a good chance it also uses feature reports and potentially compatible with Mini/BeastX 8K

## Vendor and Product IDs
| WLMouse product | Vendor ID | Product ID |
| :------------- | :-------: | :--------: |
| Beast X Mini Receiver                |`0x36A7`|? |
| Beast X Mini | `0x36A7`|? |
| Beast X | `0x36A7` | `0xA888` |
| Beast X Receiver | `0x36A7` | `0xA887` |
| Beast X 8K | `0x36A7` | `0xA884` |
| Beast X 8K Receiver | `0x36A7` | `0xA883` |
| Beast X Max | `0x36A7` | ? |
| Beast X Max Receiver | `0x36A7` | ? |

## Packets Disclaimer
All bytes sent and read from feature reports were sniffed using [**Wireshark**](https://www.wireshark.org/) and [**usbpcap**](https://desowin.org/usbpcap/) between actual device and official WLMouse software. Since there is no official structure of all the bytes sent/read, I had to guess what they do mean. This means it MAY NOT be 100% accurate

## Feature Reports
Summary of how to set/get feature reports
### Windows
1. List all devices ([GetRawInputDeviceList](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getrawinputdevicelist))
2. Get device info for each one ([GetRawInputDeviceInfoW](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getrawinputdeviceinfow) with `uiCommand == RIDI_DEVICEINFO`)
3. Get the one with required Vendor and Product ID
4. Get device virtual file ([GetRawInputDeviceInfoW](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getrawinputdeviceinfow) with `uiCommand == RIDI_DEVICENAME`)
5. Open virtual file ([CreateFileW](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-createfilew))
6. Set required feature to poll from device ([HidD_SetFeature](https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/hidsdi/nf-hidsdi-hidd_setfeature))
7. Wait for a small delay for device to process your request
8. Grab result of your request ([HidD_GetFeature](https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/hidsdi/nf-hidsdi-hidd_getfeature))