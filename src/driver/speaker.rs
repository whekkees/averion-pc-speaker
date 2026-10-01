use crate::pit::set_channel_2;
use crate::arch::x86::io::inb;
use crate::arch::x86::io::outb;

pub unsafe fn beep (frequrency : u32 ) { 
    set_channel_2(frequrency);

    let port_tmp = inb(0x61);

    if port_tmp & 0b00000011 != 0b00000011  {
        outb(0x61, port_tmp | 0b00000011);
    }
}



pub unsafe fn stop_beep() {
    let port_tmp = inb(0x61);

    outb(0x61, port_tmp & !0b00000011);
}