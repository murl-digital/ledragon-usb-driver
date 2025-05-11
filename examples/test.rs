use core::str;
use std::{
    fs::File,
    io::{BufReader, Read},
    task,
    time::Duration,
};

use rusb::{Context, Direction, UsbContext};
use serialport::{DataBits, SerialPortBuilder, SerialPortType, StopBits, available_ports};

fn main() {
    let context = Context::new().unwrap();
    let device = context
        .devices()
        .unwrap()
        .iter()
        .find(|d| {
            let descriptor = d.device_descriptor().unwrap();
            descriptor.vendor_id() == 0x6942 && descriptor.product_id() == 0x6942
        })
        .map(|d| d.open().unwrap())
        .unwrap();

    let config_desc = device.device().config_descriptor(0).unwrap();
    let interface = config_desc.interfaces().next().unwrap();
    let interface_descriptors = interface.descriptors().next().unwrap();

    for e in interface_descriptors.endpoint_descriptors() {
        println!("{e:?}");
        println!("{:?}", e.direction());
    }

    let endpoint = interface_descriptors
        .endpoint_descriptors()
        .find(|e| e.direction() == Direction::Out)
        .unwrap();

    let read_endpoint = interface_descriptors
        .endpoint_descriptors()
        .find(|e| e.direction() == Direction::In)
        .unwrap();

    println!("read interval: {}", read_endpoint.interval());

    device
        .set_active_configuration(config_desc.number())
        .unwrap();
    device.claim_interface(interface.number()).unwrap();
    device
        .set_alternate_setting(interface.number(), interface_descriptors.setting_number())
        .unwrap();

    for i in 0..5 {
        println!("{i}");
        device
            .write_bulk(endpoint.address(), &[1, 2, 3, 4], Duration::from_millis(0))
            .unwrap();
        let mut data = [0; 64];
        let mut read_amt = 0;

        match device.read_interrupt(
            read_endpoint.address(),
            &mut data,
            Duration::from_millis(150),
        ) {
            Ok(amt) => read_amt = amt,
            Err(rusb::Error::Timeout) => {
                println!("timed out");
                continue;
            }
            Err(e) => panic!("{e}"),
        }

        println!("{:?}", str::from_utf8(&data[..read_amt]));
    }
}
