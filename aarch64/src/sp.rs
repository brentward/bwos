pub struct _SP;
impl _SP {
    /// Returns the current stack pointer.
    #[inline(always)]
    pub fn get(&self) -> usize {
        let rtn: usize;
        unsafe {
            asm!("mov {}, sp", out(reg) rtn, options(nomem, nostack));
        }
        rtn
    }

    /// Set the current stack pointer with an passed argument.
    #[inline(always)]
    pub unsafe fn set(&self, stack: usize) {
        asm!("mov sp, {}", in(reg) stack,  options(nomem, nostack));
    }
}
pub static SP: _SP = _SP {};
