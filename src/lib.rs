pub(crate) const DEVICE_VID: u16 = 0x6942;
pub(crate) const DEVICE_PID: u16 = 0x6942;

pub(crate) struct UsbInternals<'usb> {
    context: rusb::Context,
    device_handle: rusb::DeviceHandle<rusb::Context>,
    interface: rusb::Interface<'usb>,
    interface_descriptor: rusb::InterfaceDescriptor<'usb>,
    read_endpoint: rusb::EndpointDescriptor<'usb>,
    write_endpoint: rusb::EndpointDescriptor<'usb>,
}

pub struct LEDragonContext<'usb> {
    internal: UsbInternals<'usb>,
}
