use std::{thread, time};

use bitflags::bitflags;
use rusb::{DeviceHandle, GlobalContext};

#[allow(dead_code)]
#[repr(u8)]
enum CmsisDapCommandId {
    Info = 0x00,
    HostStatus = 0x01,
    Connect = 0x02,
    Delay = 0x09,
    ResetTarget = 0x0a,
    SWJPins = 0x10,
    SWJClock = 0x11,
    SWJSequence = 0x12,
    SWDConfigure = 0x13,
    SWDSequence = 0x1d,
    JTAGSequence = 0x14,
    JTAGConfigure = 0x15,
    JTAGIdcode = 0x16,
}

bitflags! {
    struct SWJPins: u8 {
        const TCK_SWDCLK = 1;
        const TMS_SWDIO = 1 << 1;
        const TDI = 1 << 2;
        const TDO = 1 << 3;
        const N_TRST = 1 << 5;
        const N_RESET = 1 << 7;
    }
}

struct CmsisDapInterface {
    device: DeviceHandle<GlobalContext>,
    endpoint_out: u8,
    endpoint_in: u8,
}

const MAX_PACKET_SIZE: usize = 512;
const TIMEOUT_MS: time::Duration = time::Duration::from_millis(1000);

impl CmsisDapInterface {
    pub fn new(vendor_id: u16, product_id: u16) -> Self {
        let device = rusb::open_device_with_vid_pid(vendor_id, product_id).unwrap();
        // TODO: fix magic number
        CmsisDapInterface {
            device,
            endpoint_out: 0x02,
            endpoint_in: 0x83,
        }
    }

    fn write(&self, buf: &[u8]) {
        self.device
            .write_bulk(self.endpoint_out, buf, TIMEOUT_MS)
            .unwrap();
    }

    fn read(&self) -> Vec<u8> {
        let mut buf = [0; MAX_PACKET_SIZE];
        let length = self
            .device
            .read_bulk(self.endpoint_in, &mut buf, TIMEOUT_MS)
            .unwrap();
        buf[..length].to_vec()
    }

    pub fn info(&self) {
        // get product name
        self.write(&[CmsisDapCommandId::Info as u8, 0x02]);
        let buf = self.read();
        println!("{:x?}", buf);
    }

    pub fn connect(&self) {
        self.write(&[CmsisDapCommandId::Connect as u8, 2]);
        let buf = self.read();
        println!("{:x?}", buf);
    }

    pub fn swj_pins(&self, pin_output: SWJPins, pin_select: SWJPins, wait_us: u32) {
        let mut commands = vec![
            CmsisDapCommandId::SWJPins as u8,
            pin_output.bits(),
            pin_select.bits(),
        ];
        commands.extend_from_slice(&wait_us.to_le_bytes());
        println!("{:x?}", commands);
        self.write(&commands);
        let response = self.read();
        assert!(response[0] == (CmsisDapCommandId::SWJPins as u8));
        assert!(response.len() == 2);
        println!("{:x}", response[1]);
    }

    pub fn 
}

fn main() {
    let dap = CmsisDapInterface::new(0x6666, 0x4444);
    // dap info
    // dap.info();
    dap.connect();
    // loop {
    //     let pin = SWJPins::N_RESET;
    //     dap.swj_pins(pin, pin, 0);
    //     thread::sleep(time::Duration::from_secs(1));
    //     dap.swj_pins(SWJPins::empty(), pin, 0);
    //     thread::sleep(time::Duration::from_secs(1));
    // }
}
