/// Trait that descibes a bus where not every connection is necissarily mapped to a device
pub trait OpenBus {
    fn read(&self, addr: u16) -> Option<u8>;
    ///true if the addr is mapped to a device, false otherwise
    fn write(&mut self, addr: u16, byte: u8) -> bool;
}

pub trait Bus {
    fn read(&self, addr: u16) -> u8;
    fn write(&mut self, addr: u16, byte: u8);
}

pub trait BusDevice<FuckyGenericContstrain> {
    type Bus: Bus;
    fn cycle(&mut self, bus: &mut Self::Bus) -> Option<u8>;
}

pub trait OpenBusDevice<B: OpenBus> {
    fn cycle(&mut self, bus: &mut B) -> Option<u8>;
}

impl<B: Bus> OpenBus for B {
    fn read(&self, addr: u16) -> Option<u8> {
        Some(self.read(addr))
    }

    fn write(&mut self, addr: u16, byte: u8) -> bool {
        self.write(addr, byte);
        //A Bus is guranteed to have every addr mapped to a device
        true
    }
}

impl<D: BusDevice<()>> OpenBusDevice<D::Bus> for D {
    fn cycle(&mut self, bus: &mut D::Bus) -> Option<u8> {
        self.cycle(bus)
    }
}

impl Bus for [u8; u16::MAX as usize] {
    fn read(&self, addr: u16) -> u8 {
        self[addr as usize]
    }

    fn write(&mut self, addr: u16, byte: u8) {
        self[addr as usize] = byte
    }
}
