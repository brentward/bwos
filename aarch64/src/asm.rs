/// Wait for event not to burn CPU.
#[inline(always)]
pub fn wfe() {
    unsafe { asm!("wfe", options(nomem, nostack)) };
}

/// Wait for interrupt not to burn CPU.
#[inline(always)]
pub fn wfi() {
    unsafe { asm!("wfi", options(nomem, nostack)) };
}

/// A NOOP that won't be optimized out.
#[inline(always)]
pub fn nop() {
    unsafe { asm!("nop", options(nomem, nostack)) };
}

/// Transition to a lower level
#[inline(always)]
pub unsafe fn eret() {
    asm!("eret", options(nomem, nostack));
}

/// Instruction Synchronization Barrier
#[inline(always)]
pub fn isb() {
    unsafe { asm!("isb", options(nostack)) };
}

/// Set Event
#[inline(always)]
pub fn sev() {
    unsafe { asm!("sev", options(nomem, nostack)) };
}

/// Enable (unmask) interrupts
#[inline(always)]
pub fn enable_irq_interrupt() {
    unsafe {
        asm!("msr DAIFClr, {}", const 0b0010, options(nomem, nostack));
    }
}

/// Disable (mask) interrupt
#[inline(always)]
pub fn disable_irq_interrupt() {
    unsafe {
        asm!("msr DAIFSet, {}", const 0b0010, options(nomem, nostack));
    }
}

/// Enable (unmask) FIQ
#[inline(always)]
pub fn enable_fiq_interrupt() {
    unsafe {
        asm!("msr DAIFClr, {}", const 0b0001, options(nomem, nostack));
    }
}

/// Disable (mask) FIQ
#[inline(always)]
pub fn disable_fiq_interrupt() {
    unsafe {
        asm!("msr DAIFSet, {}", const 0b0001, options(nomem, nostack));
    }
}

pub fn get_interrupt_mask() -> u64 {
    unsafe {
        let mut mask: u64;
        asm!("mrs {}, DAIF", out(reg) mask, options(nomem, nostack));
        mask
    }
}

pub fn set_interrupt_mask(mask: u64) {
    unsafe {
        asm!("msr DAIF, {}", in(reg) mask, options(nomem, nostack));
    }
}
