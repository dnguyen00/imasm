#[repr(C)]
pub union EAX {
    pub al: u8,
    pub ah: u8,
    pub ax: u16,
    pub eax: u32,
}