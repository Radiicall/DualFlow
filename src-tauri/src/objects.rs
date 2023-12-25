//use crate::checksum;
use hidapi::{DeviceInfo, HidApi, HidDevice};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone)]
pub struct Controller {
    pub device_info: DeviceInfo,
    pub connection_type: ConnectionType,
    pub report_size: u8,
    pub out_report: Vec<u8>,
    pub left_trigger: Trigger,
    pub right_trigger: Trigger,
}

impl Controller {
    pub fn new(api: &HidApi) -> Result<Self, String> {
        let device_info = Self::find_device(api)?;

        let device = device_info
            .open_device(api)
            .ok()
            .ok_or("Can't open device")?;

        let connection_type = ConnectionType::check_connection_type(&device)?;

        let mut report_size: u8 = 0;

        if connection_type == ConnectionType::USB {
            report_size = 64
        } else if connection_type == ConnectionType::Bluetooth {
            report_size = 78
        }

        let controller = Self {
            device_info,
            connection_type,
            report_size,
            out_report: vec![],
            left_trigger: Trigger::default(),
            right_trigger: Trigger::default(),
        };

        Ok(controller)
    }

    fn device(&self, api: &HidApi) -> Result<HidDevice, &str> {
        self.device_info
            .open_device(api)
            .ok()
            .ok_or("Can't open device")
    }

    fn find_device(api: &HidApi) -> Result<DeviceInfo, String> {
        for device in api.device_list() {
            if device.vendor_id() == 0x054c && device.product_id() == 0x0CE6 {
                return Ok(device.to_owned());
            }
        }
        Err("DualSense not found".to_string())
    }

    pub fn write(&mut self, api: &HidApi) -> Result<(), Box<dyn std::error::Error>> {        
        let device = self.device(api)?;

        self.prepare_report();

        device.write(&self.out_report)?;
        Ok(())
    }

    fn prepare_report(&mut self) {
        let mut out_report: Vec<u8> = vec![];

        for _ in 0..=self.report_size {
            out_report.push(0)
        }

        out_report[1] = self.connection_type.into();

        if self.connection_type == ConnectionType::USB {
            out_report[2] = 0xff;
            out_report[3] = 0x1 | 0x2 | 0x4 | 0x10 | 0x40;

            out_report[12] = self.right_trigger.mode.into();
            out_report[13] = self.right_trigger.strength[0];
            out_report[14] = self.right_trigger.strength[1];
            out_report[15] = self.right_trigger.strength[2];
            out_report[16] = self.right_trigger.strength[3];
            out_report[17] = self.right_trigger.strength[4];
            out_report[18] = self.right_trigger.strength[5];
            out_report[21] = self.right_trigger.strength[6];

            out_report[23] = self.left_trigger.mode.into();
            out_report[24] = self.left_trigger.strength[0];
            out_report[25] = self.left_trigger.strength[1];
            out_report[26] = self.left_trigger.strength[2];
            out_report[27] = self.left_trigger.strength[3];
            out_report[28] = self.left_trigger.strength[4];
            out_report[29] = self.left_trigger.strength[5];
            out_report[32] = self.left_trigger.strength[6];
        } else if self.connection_type == ConnectionType::Bluetooth {
            out_report[2] = ConnectionType::USB.into();
            out_report[3] = 0xff;
            out_report[4] = 0x1 | 0x2 | 0x4 | 0x10 | 0x40;

            out_report[13] = self.right_trigger.mode.into();
            out_report[14] = self.right_trigger.strength[0];
            out_report[15] = self.right_trigger.strength[1];
            out_report[16] = self.right_trigger.strength[2];
            out_report[17] = self.right_trigger.strength[3];
            out_report[18] = self.right_trigger.strength[4];
            out_report[19] = self.right_trigger.strength[5];
            out_report[22] = self.right_trigger.strength[6];

            out_report[24] = self.left_trigger.mode.into();
            out_report[25] = self.left_trigger.strength[0];
            out_report[26] = self.left_trigger.strength[1];
            out_report[27] = self.left_trigger.strength[2];
            out_report[28] = self.left_trigger.strength[3];
            out_report[29] = self.left_trigger.strength[4];
            out_report[30] = self.left_trigger.strength[5];
            out_report[33] = self.left_trigger.strength[6];

            /*
            TODO: Fix this
            let crc_checksum = checksum::compute(&out_report);

            out_report[75] = (crc_checksum & 0x000000FF) as u8;
            out_report[76] = ((crc_checksum & 0x0000FF00) >> 8) as u8;
            out_report[77] = ((crc_checksum & 0x00FF0000) >> 16) as u8;
            out_report[78] = ((crc_checksum & 0xFF000000) >> 24) as u8;
            */
        }

        self.out_report = out_report
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ConnectionType {
    Bluetooth,
    USB,
    Unknown,
}

impl Into<u8> for ConnectionType {
    fn into(self) -> u8 {
        match self {
            Self::Bluetooth => 0x31,
            Self::USB => 0x02,
            Self::Unknown => 0x0,
        }
    }
}

impl ConnectionType {
    fn check_connection_type(device: &hidapi::HidDevice) -> Result<Self, String> {
        let report_length = device
            .read(&mut [0u8; 100])
            .ok()
            .ok_or("Can't read device")?;

        if report_length == 64 {
            return Ok(Self::USB);
        } else if report_length == 78 {
            return Ok(Self::Bluetooth);
        }
        Ok(Self::Unknown)
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub struct Trigger {
    pub mode: TriggerMode,
    pub strength: [u8; 7],
}

impl Default for Trigger {
    fn default() -> Self {
        Self {
            mode: TriggerMode::default(),
            strength: [0, 0, 0, 0, 0, 0, 0],
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy)]
#[serde(rename_all = "PascalCase")]
pub enum TriggerMode {
    Off,
    Rigid,
    Pulse,
    RigidA,
    RigidB,
    RigidAB,
    PulseA,
    PulseB,
    PulseAB,
    Calibration,
}

impl From<&str> for TriggerMode {
    fn from(s: &str) -> Self {
        match s {
            "Rigid" => TriggerMode::Rigid,
            "Pulse" => TriggerMode::Pulse,
            "RigidA" => TriggerMode::RigidA,
            "RigidB" => TriggerMode::RigidB,
            "RigidAB" => TriggerMode::RigidAB,
            "PulseA" => TriggerMode::PulseA,
            "PulseB" => TriggerMode::PulseB,
            "PulseAB" => TriggerMode::PulseAB,
            "Calibration" => TriggerMode::Calibration,
            _ => TriggerMode::Off,
        }
    }
}

impl Into<u8> for TriggerMode {
    fn into(self) -> u8 {
        match self {
            TriggerMode::Off => 0x0,
            TriggerMode::Rigid => 0x1,
            TriggerMode::Pulse => 0x2,
            TriggerMode::RigidA => 0x1 | 0x20,
            TriggerMode::RigidB => 0x1 | 0x04,
            TriggerMode::RigidAB => 0x1 | 0x20 | 0x04,
            TriggerMode::PulseA => 0x2 | 0x20,
            TriggerMode::PulseB => 0x2 | 0x04,
            TriggerMode::PulseAB => 0x2 | 0x20 | 0x04,
            TriggerMode::Calibration => 0xFC,
        }
    }
}

impl Default for TriggerMode {
    fn default() -> Self {
        Self::Off
    }
}
