use rusb::{DeviceHandle, GlobalContext};
use std::time;

#[repr(u8)]
enum CmsisDapCommandId {
    Info = 0x00,
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

    pub fn info(&self, id: u8) {
        self.write(&[CmsisDapCommandId::Info as u8, id]);
        let buf = self.read();
        println!("{}", std::str::from_utf8(&buf).unwrap());
    }
}

fn main() {
    let dap = CmsisDapInterface::new(0x6666, 0x4444);
    // dap info
    dap.info(0x01);
}
