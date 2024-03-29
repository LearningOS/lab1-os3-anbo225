
use riscv::register::sstatus::{self, Sstatus, SPP};

#[repr(C)]
// trap context structure containing sstatus, sepc and registers
pub struct TrapContext{
    pub x:[usize;32],
    pub sstatus: Sstatus,
    pub sepc: usize,
}

// todo： trapContext是哪里初始化的设置
impl TrapContext{
    pub fn set_sp(&mut self, sp: usize) {
        self.x[2] = sp;
    }

    // 这个函数做了什么呢？
    pub fn app_init_context(entry: usize, sp: usize) -> Self {
        let mut sstatus = sstatus::read();
        sstatus.set_spp(SPP::User);
        let mut cx = Self {
            x: [0; 32],
            sstatus,
            sepc: entry,
        };
        cx.set_sp(sp);
        cx
    }

}
