mod registers;

fn main() {
    //:0>8X lead 0 for 8 'bytes'
    let x = registers::EAX {eax: 123456789};

    /* let y = unsafe { &x.al as *const u8 }; */

    unsafe { println!("Address: {:?} {:?} {:?} {:?}", &x.eax as *const u32, &x.ax as *const u16, (&x.ah as *const u8).add(1), &x.al as *const u8) };
    unsafe { println!("Values: {:0>8X} {:0>4X} {:0>2X} {:0>2X}", *(&x.eax as *const u32), *(&x.ax as *const u16), *(&x.ah as *const u8).add(1), *(&x.al as *const u8)) };
    unsafe { println!("{:0>8X} {:0>4X} {:0>2X} {:0>2X}", x.eax, x.ax, x.ah, x.al) };
}