pub struct ConfigurationResource {
    pub device: Option<Device>,
    pub selected_device_channels: Vec<u16>,
    pub approach_rate: f32 // Meters/units per second
}