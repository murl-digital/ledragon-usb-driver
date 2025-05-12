use core::str;
use std::{
    fs::File,
    io::{BufReader, Read},
    task, thread,
    time::{Duration, Instant},
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
    device.reset().unwrap();

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

    let file = BufReader::new(File::open("/home/draconium/Downloads/5mb.txt").unwrap());
    let bytes = file.bytes().collect::<Result<Vec<_>, _>>().unwrap();
    println!("{}", bytes.len());

    for chunk in bytes.as_slice().chunks(1024) {
        let now = Instant::now();
        let write_amt = device
            .write_bulk(endpoint.address(), chunk, Duration::from_millis(0))
            .unwrap();
        //println!("{:?}", now.elapsed());
        let mut data = [0; 64];
        let mut read_amt = 0;

        // match device.read_bulk(read_endpoint.address(), &mut data, Duration::from_millis(0)) {
        //     Ok(amt) => read_amt = amt,
        //     Err(rusb::Error::Timeout) => {
        //         println!("timed out");
        //         continue;
        //     }
        //     Err(e) => panic!("{e}"),
        // }

        //println!("{write_amt}");
        //println!("{:?}", str::from_utf8(&data[..read_amt]));
    }
}
