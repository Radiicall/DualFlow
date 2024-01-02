use hidapi::HidApi;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone)]
pub struct Controller {
    pub connection_type: ConnectionType,
    pub report_size: u8,
    pub out_report: Vec<u8>,
    pub left_trigger: Trigger,
    pub right_trigger: Trigger,
}

impl Controller {
    pub fn new(_: &HidApi) -> Result<Self, String> {

        let report_size: u8 = 64;

        let controller = Self {
            connection_type: ConnectionType::USB,
            report_size,
            out_report: vec![],
            left_trigger: Trigger::default(),
            right_trigger: Trigger::default(),
        };

        Ok(controller)
    }
    pub fn write(&mut self, _: &HidApi) -> Result<(), Box<dyn std::error::Error>> {  
        Ok(())
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
