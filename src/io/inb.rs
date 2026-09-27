use core::arch::asm;


pub unsafe fn inb(port : u16) -> u8 { 
    let value : u8;

    asm!(
        "in al, dx",
        in("dx") port,
        out("al") value,
        options(nostack,preserves_flags),
    );

    value
}