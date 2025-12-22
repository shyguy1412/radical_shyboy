use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TestCase<T> {
    pub name: String,
    pub initial: State<T>,
    #[serde(rename = "final")]
    pub target: State<T>,
    pub cycles: Vec<(u16, u8, String)>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct State<T> {
    #[serde(flatten)]
    pub cpu: T,
    pub ram: Vec<(u16, u8)>,
}

impl crate::bus::OpenBus for Vec<(u16, u8)> {
    fn read(&self, addr: u16) -> Option<u8> {
        self.iter()
            .find_map(|(ram_addr, val)| match *ram_addr == addr {
                true => Some(*val),
                false => None,
            })
    }

    fn write(&mut self, addr: u16, byte: u8) -> Option<()> {
        for (ram_addr, val) in self.iter_mut() {
            if *ram_addr != addr {
                continue;
            }
            *val = byte;
            return Some(());
        }
        self.push((addr, byte));
        Some(())
    }
}
