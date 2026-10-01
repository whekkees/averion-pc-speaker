use crate::arch::x86::io::inb;
use crate::arch::x86::io::outb;

pub const PIT_FREQURENCY : u32 = 1_193_180;


pub unsafe fn set_channel_2 (frequrency : u32) {
    let divider : u32  = PIT_FREQURENCY / frequrency;

    outb(0x43, 0xB6);

    outb(0x42, divider as u8);
    outb(0x42, ( divider >> 8 ) as u8);
}

/*
10110110
1011 - 0xB6
0110 - 6
 */