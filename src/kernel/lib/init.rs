use crate::kernel::lib::consoleio;

pub fn sys_init() {
    consoleio::print(b"TinosRSE initialization complete.\n");
}