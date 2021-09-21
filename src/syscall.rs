pub unsafe extern "C" fn sys_write(_fd: usize, _buf: *const u8, _count: usize) {
    asm!("li a7, 1", "ecall");
}

pub unsafe extern "C" fn sys_create_window(_title: *const u8, _title_len: usize, _x: usize, _y: usize, _width: usize, _height: usize) -> usize {
    let ret: usize;
    asm!("li a7, 1000", "ecall", "mv {}, a0", out(reg)ret);
    ret
}

pub unsafe extern "C" fn sys_map_window(_window_id: usize, _vaddr: usize) {
    asm!("li a7, 1001", "ecall");
}

pub unsafe extern "C" fn sys_sync_window(_window_id: usize) {
    asm!("li a7, 1002", "ecall");
}

