use crate::x86::hlt;
use crate::x86::write_io_port_u8;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QemuExitCode {
    Success = 0x1,
    Fail = 0x2,
}
pub fn exit_qemu(exit_code: QemuExitCode) -> ! {
    const QEMU_EXIT_IO_PORT: u16 = 0xf4;
    write_io_port_u8(QEMU_EXIT_IO_PORT, exit_code as u8);
    loop {
        hlt();
    }
}
