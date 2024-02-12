#![no_std]
use core::arch::global_asm;

use riscv::register::mtvec;
use clic::register::{super_mtvec, stack_depth};
#[no_mangle]
pub unsafe fn _setup_interrupts() {
    mtvec::write(
        &_VECTOR_TABLE.handler0 as *const _ as usize,
        mtvec::TrapMode::Vectored,
    );
    super_mtvec::write(&_SUPER_VECTOR_TABLE as *const _  as usize);
    //set register stack depth
    stack_depth::write(2);
}
#[repr(C, align(4))]
struct VectorTable {
    pub handler0: unsafe extern "C" fn(),
    pub handler1: unsafe extern "C" fn(),
    pub handler2: unsafe extern "C" fn(),
    pub handler3: unsafe extern "C" fn(),
    pub handler4: unsafe extern "C" fn(),
    pub handler5: unsafe extern "C" fn(),
    pub handler6: unsafe extern "C" fn(),
    pub handler7: unsafe extern "C" fn(),
    pub handler8: unsafe extern "C" fn(),
    pub handler9: unsafe extern "C" fn(),
}

extern "C" {
    fn handler_0();
    fn handler_1();
    fn handler_2();
    fn handler_3();
    fn handler_4();
    fn handler_5();
    fn handler_6();
    fn handler_7();
    fn handler_8();
    fn handler_9();
}

extern "C" {
    fn Interrupt0();
    fn Interrupt1();
    fn Interrupt2();
    fn Interrupt3();
    fn Interrupt4();
    fn Interrupt5();
    fn Interrupt6();
    fn Interrupt7();
    fn Interrupt8();
    fn Interrupt9();
}

//must be accessible over data bus, for now have it here since .text isn't
#[link_section = ".vector_table"]
static _VECTOR_TABLE: VectorTable = VectorTable {
    handler0: handler_0,
    handler1: handler_1,
    handler2: handler_2,
    handler3: handler_3,
    handler4: handler_4,
    handler5: handler_5,
    handler6: handler_6,
    handler7: handler_7,
    handler8: handler_8,
    handler9: handler_9,
};

#[link_section = ".vector_table"]
static _SUPER_VECTOR_TABLE: VectorTable = VectorTable {
    handler0: Interrupt0,
    handler1: Interrupt1,
    handler2: Interrupt2,
    handler3: Interrupt3,
    handler4: Interrupt4,
    handler5: Interrupt5,
    handler6: Interrupt6,
    handler7: Interrupt7,
    handler8: Interrupt8,
    handler9: Interrupt9,
};

global_asm!(
    "
.text
handler_0:
    addi     sp, sp, -0x4c       # allocate space for the context on the stack
    sw       a0, 0x10(sp)        # start by pushing a0 we it to stack CSRs and set threshold
    csrrs    a0, mstatus, x0     # read and stack mstatus
    sw       a0, 0x00(sp)
    csrrs    a0, mepc, x0        # read and stack mepc
    sw       a0, 0x04(sp)
#_STORE_PRIO SUBROUTINE
    csrr     a0, 0x347           # load current threshold
    sw       a0, 0x08(sp)        # store old threshold on stack
    li       a0, 0x1000          # base address for the CLIC MMIO
    lb       a0, 3(a0)           # load prio config register of interrupt 0
    csrw     0x347, a0           # set the priority
    csrrsi   x0, mstatus, 8      # enable interrupts (end of critical section)
#END
    sw       ra, 0x0c(sp)        # stack the caller saved registers
    sw       a1, 0x14(sp)
    sw       a2, 0x18(sp)
    sw       a3, 0x1c(sp)
    sw       a4, 0x20(sp)
    sw       a5, 0x24(sp)
    sw       a6, 0x28(sp)
    sw       a7, 0x2c(sp)
    sw       t0, 0x30(sp)
    sw       t1, 0x34(sp)
    sw       t2, 0x38(sp)
    sw       t3, 0x3c(sp)
    sw       t4, 0x40(sp)
    sw       t5, 0x44(sp)
    sw       t6, 0x48(sp)

    jal      ra, Interrupt0           # call into the user defined handler

    lw       a0, 0x00(sp)        # restore CSRs and caller saved registers
    csrrw    x0, mstatus, a0
    lw       a0, 0x04(sp)
    csrrw    x0, mepc, a0
    lw       ra, 0x0c(sp)
    lw       a0, 0x10(sp)
    lw       a1, 0x14(sp)
    lw       a2, 0x18(sp)
    lw       a3, 0x1c(sp)
    lw       a4, 0x20(sp)
    lw       a5, 0x24(sp)
    lw       a6, 0x28(sp)
    lw       a7, 0x2c(sp)
    lw       t0, 0x30(sp)
    lw       t1, 0x34(sp)
    lw       t2, 0x38(sp)
    lw       t3, 0x3c(sp)
    lw       t4, 0x40(sp)
    lw       t5, 0x44(sp)
    #CS
    csrci    mstatus, 0x8     
    lw       t6, 0x08(sp)        # load the old threshold from the stack
    csrw     0x347, t6           # set the priority
    lw       t6, 0x48(sp)
    addi     sp, sp, 0x4c
    mret                         # return from interrupt

handler_1:
    addi     sp, sp, -0x4c       # allocate space for the context on the stack
    sw       a0, 0x10(sp)        # start by pushing a0 we it to stack CSRs and set threshold
    csrrs    a0, mstatus, x0     # read and stack mstatus
    sw       a0, 0x00(sp)
    csrrs    a0, mepc, x0        # read and stack mepc
    sw       a0, 0x04(sp)
#_STORE_PRIO SUBROUTINE
    csrr     a0, 0x347           # load current threshold
    sw       a0, 0x08(sp)        # store old threshold on stack
    li       a0, 0x1000          # base address for the CLIC MMIO
    lb       a0, 7(a0)           # load prio config register of interrupt 1
    csrw     0x347, a0           # set the priority
    csrrsi   x0, mstatus, 8      # enable interrupts (end of critical section)
#END
    sw       ra, 0x0c(sp)        # stack the caller saved registers
    sw       a1, 0x14(sp)
    sw       a2, 0x18(sp)
    sw       a3, 0x1c(sp)
    sw       a4, 0x20(sp)
    sw       a5, 0x24(sp)
    sw       a6, 0x28(sp)
    sw       a7, 0x2c(sp)
    sw       t0, 0x30(sp)
    sw       t1, 0x34(sp)
    sw       t2, 0x38(sp)
    sw       t3, 0x3c(sp)
    sw       t4, 0x40(sp)
    sw       t5, 0x44(sp)
    sw       t6, 0x48(sp)
    jal      ra, Interrupt1           # call into the user defined handler

    lw       a0, 0x00(sp)        # restore CSRs and caller saved registers
    csrrw    x0, mstatus, a0
    lw       a0, 0x04(sp)
    csrrw    x0, mepc, a0
    lw       ra, 0x0c(sp)
    lw       a0, 0x10(sp)
    lw       a1, 0x14(sp)
    lw       a2, 0x18(sp)
    lw       a3, 0x1c(sp)
    lw       a4, 0x20(sp)
    lw       a5, 0x24(sp)
    lw       a6, 0x28(sp)
    lw       a7, 0x2c(sp)
    lw       t0, 0x30(sp)
    lw       t1, 0x34(sp)
    lw       t2, 0x38(sp)
    lw       t3, 0x3c(sp)
    lw       t4, 0x40(sp)
    lw       t5, 0x44(sp)
    #CS
    csrci    mstatus, 0x8     
    lw       t6, 0x08(sp)        # load the old threshold from the stack
    csrw     0x347, t6           # set the priority
    lw       t6, 0x48(sp)
    addi     sp, sp, 0x4c
    mret                       # return from interrupt

handler_2:
    addi     sp, sp, -0x4c       # allocate space for the context on the stack
    sw       a0, 0x10(sp)        # start by pushing a0 we it to stack CSRs and set threshold
    csrrs    a0, mstatus, x0     # read and stack mstatus
    sw       a0, 0x00(sp)
    csrrs    a0, mepc, x0        # read and stack mepc
    sw       a0, 0x04(sp)
#_STORE_PRIO SUBROUTINE
    csrr     a0, 0x347           # load current threshold
    sw       a0, 0x08(sp)        # store old threshold on stack
    li       a0, 0x1000          # base address for the CLIC MMIO
    lb       a0, 11(a0)          # load prio config register of interrupt 2
    csrw     0x347, a0           # set the priority
    csrrsi   x0, mstatus, 8      # enable interrupts (end of critical section)
#END
    sw       ra, 0x0c(sp)        # stack the caller saved registers
    sw       a1, 0x14(sp)
    sw       a2, 0x18(sp)
    sw       a3, 0x1c(sp)
    sw       a4, 0x20(sp)
    sw       a5, 0x24(sp)
    sw       a6, 0x28(sp)
    sw       a7, 0x2c(sp)
    sw       t0, 0x30(sp)
    sw       t1, 0x34(sp)
    sw       t2, 0x38(sp)
    sw       t3, 0x3c(sp)
    sw       t4, 0x40(sp)
    sw       t5, 0x44(sp)
    sw       t6, 0x48(sp)

    jal      ra, Interrupt2           # call into the user defined handler

    lw       a0, 0x00(sp)        # restore CSRs and caller saved registers
    csrrw    x0, mstatus, a0
    lw       a0, 0x04(sp)
    csrrw    x0, mepc, a0
    lw       ra, 0x0c(sp)
    lw       a0, 0x10(sp)
    lw       a1, 0x14(sp)
    lw       a2, 0x18(sp)
    lw       a3, 0x1c(sp)
    lw       a4, 0x20(sp)
    lw       a5, 0x24(sp)
    lw       a6, 0x28(sp)
    lw       a7, 0x2c(sp)
    lw       t0, 0x30(sp)
    lw       t1, 0x34(sp)
    lw       t2, 0x38(sp)
    lw       t3, 0x3c(sp)
    lw       t4, 0x40(sp)
    lw       t5, 0x44(sp)
    #CS
    csrci    mstatus, 0x8     
    lw       t6, 0x08(sp)        # load the old threshold from the stack
    csrw     0x347, t6           # set the priority
    lw       t6, 0x48(sp)
    addi     sp, sp, 0x4c
    mret                       # return from interrupt
handler_3:
    addi     sp, sp, -0x4c       # allocate space for the context on the stack
    sw       a0, 0x10(sp)        # start by pushing a0 we it to stack CSRs and set threshold
    csrrs    a0, mstatus, x0     # read and stack mstatus
    sw       a0, 0x00(sp)
    csrrs    a0, mepc, x0        # read and stack mepc
    sw       a0, 0x04(sp)
#_STORE_PRIO SUBROUTINE
    csrr     a0, 0x347           # load current threshold
    sw       a0, 0x08(sp)        # store old threshold on stack
    li       a0, 0x1000          # base address for the CLIC MMIO
    lb       a0, 15(a0)          # load prio config register of interrupt 3
    csrw     0x347, a0           # set the priority
    csrrsi   x0, mstatus, 8      # enable interrupts (end of critical section)
#END
    sw       ra, 0x0c(sp)        # stack the caller saved registers
    sw       a1, 0x14(sp)
    sw       a2, 0x18(sp)
    sw       a3, 0x1c(sp)
    sw       a4, 0x20(sp)
    sw       a5, 0x24(sp)
    sw       a6, 0x28(sp)
    sw       a7, 0x2c(sp)
    sw       t0, 0x30(sp)
    sw       t1, 0x34(sp)
    sw       t2, 0x38(sp)
    sw       t3, 0x3c(sp)
    sw       t4, 0x40(sp)
    sw       t5, 0x44(sp)
    sw       t6, 0x48(sp)

    jal      ra, Interrupt3           # call into the user defined handler

    lw       a0, 0x00(sp)        # restore CSRs and caller saved registers
    csrrw    x0, mstatus, a0
    lw       a0, 0x04(sp)
    csrrw    x0, mepc, a0
    lw       ra, 0x0c(sp)
    lw       a0, 0x10(sp)
    lw       a1, 0x14(sp)
    lw       a2, 0x18(sp)
    lw       a3, 0x1c(sp)
    lw       a4, 0x20(sp)
    lw       a5, 0x24(sp)
    lw       a6, 0x28(sp)
    lw       a7, 0x2c(sp)
    lw       t0, 0x30(sp)
    lw       t1, 0x34(sp)
    lw       t2, 0x38(sp)
    lw       t3, 0x3c(sp)
    lw       t4, 0x40(sp)
    lw       t5, 0x44(sp)
    #CS
    csrci    mstatus, 0x8     
    lw       t6, 0x08(sp)        # load the old threshold from the stack
    csrw     0x347, t6           # set the priority
    lw       t6, 0x48(sp)
    addi     sp, sp, 0x4c
    mret                       # return from interrupt

handler_4:
    addi     sp, sp, -0x4c       # allocate space for the context on the stack
    sw       a0, 0x10(sp)        # start by pushing a0 we it to stack CSRs and set threshold
    csrrs    a0, mstatus, x0     # read and stack mstatus
    sw       a0, 0x00(sp)
    csrrs    a0, mepc, x0        # read and stack mepc
    sw       a0, 0x04(sp)
#_STORE_PRIO SUBROUTINE
    csrr     a0, 0x347           # load current threshold
    sw       a0, 0x08(sp)        # store old threshold on stack
    li       a0, 0x1000          # base address for the CLIC MMIO
    lb       a0, 19(a0)          # load prio config register of interrupt 4
    csrw     0x347, a0           # set the priority
    csrrsi   x0, mstatus, 8      # enable interrupts (end of critical section)
#END
    sw       ra, 0x0c(sp)        # stack the caller saved registers
    sw       a1, 0x14(sp)
    sw       a2, 0x18(sp)
    sw       a3, 0x1c(sp)
    sw       a4, 0x20(sp)
    sw       a5, 0x24(sp)
    sw       a6, 0x28(sp)
    sw       a7, 0x2c(sp)
    sw       t0, 0x30(sp)
    sw       t1, 0x34(sp)
    sw       t2, 0x38(sp)
    sw       t3, 0x3c(sp)
    sw       t4, 0x40(sp)
    sw       t5, 0x44(sp)
    sw       t6, 0x48(sp)

    jal      ra, Interrupt4           # call into the user defined handler

    lw       a0, 0x00(sp)        # restore CSRs and caller saved registers
    csrrw    x0, mstatus, a0
    lw       a0, 0x04(sp)
    csrrw    x0, mepc, a0
    lw       ra, 0x0c(sp)
    lw       a0, 0x10(sp)
    lw       a1, 0x14(sp)
    lw       a2, 0x18(sp)
    lw       a3, 0x1c(sp)
    lw       a4, 0x20(sp)
    lw       a5, 0x24(sp)
    lw       a6, 0x28(sp)
    lw       a7, 0x2c(sp)
    lw       t0, 0x30(sp)
    lw       t1, 0x34(sp)
    lw       t2, 0x38(sp)
    lw       t3, 0x3c(sp)
    lw       t4, 0x40(sp)
    lw       t5, 0x44(sp)
    #CS
    csrci    mstatus, 0x8     
    lw       t6, 0x08(sp)        # load the old threshold from the stack
    csrw     0x347, t6           # set the priority
    lw       t6, 0x48(sp)
    addi     sp, sp, 0x4c
    mret                       # return from interrupt
handler_5:
    addi     sp, sp, -0x4c       # allocate space for the context on the stack
    sw       a0, 0x10(sp)        # start by pushing a0 we it to stack CSRs and set threshold
    csrrs    a0, mstatus, x0     # read and stack mstatus
    sw       a0, 0x00(sp)
    csrrs    a0, mepc, x0        # read and stack mepc
    sw       a0, 0x04(sp)
#_STORE_PRIO SUBROUTINE
    csrr     a0, 0x347           # load current threshold
    sw       a0, 0x08(sp)        # store old threshold on stack
    li       a0, 0x1000          # base address for the CLIC MMIO
    lb       a0, 23(a0)          # load prio config register of interrupt 5
    csrw     0x347, a0           # set the priority
    csrrsi   x0, mstatus, 8      # enable interrupts (end of critical section)
#END
    sw       ra, 0x0c(sp)        # stack the caller saved registers
    sw       a1, 0x14(sp)
    sw       a2, 0x18(sp)
    sw       a3, 0x1c(sp)
    sw       a4, 0x20(sp)
    sw       a5, 0x24(sp)
    sw       a6, 0x28(sp)
    sw       a7, 0x2c(sp)
    sw       t0, 0x30(sp)
    sw       t1, 0x34(sp)
    sw       t2, 0x38(sp)
    sw       t3, 0x3c(sp)
    sw       t4, 0x40(sp)
    sw       t5, 0x44(sp)
    sw       t6, 0x48(sp)
    jal      ra, Interrupt5           # call into the user defined handler

    lw       a0, 0x00(sp)        # restore CSRs and caller saved registers
    csrrw    x0, mstatus, a0
    lw       a0, 0x04(sp)
    csrrw    x0, mepc, a0
    lw       ra, 0x0c(sp)
    lw       a0, 0x10(sp)
    lw       a1, 0x14(sp)
    lw       a2, 0x18(sp)
    lw       a3, 0x1c(sp)
    lw       a4, 0x20(sp)
    lw       a5, 0x24(sp)
    lw       a6, 0x28(sp)
    lw       a7, 0x2c(sp)
    lw       t0, 0x30(sp)
    lw       t1, 0x34(sp)
    lw       t2, 0x38(sp)
    lw       t3, 0x3c(sp)
    lw       t4, 0x40(sp)
    lw       t5, 0x44(sp)
    #CS
    csrci    mstatus, 0x8     
    lw       t6, 0x08(sp)        # load the old threshold from the stack
    csrw     0x347, t6           # set the priority
    lw       t6, 0x48(sp)
    addi     sp, sp, 0x4c
    mret                       # return from interrupt

handler_6:
    addi     sp, sp, -0x4c       # allocate space for the context on the stack
    sw       a0, 0x10(sp)        # start by pushing a0 we it to stack CSRs and set threshold
    csrrs    a0, mstatus, x0     # read and stack mstatus
    sw       a0, 0x00(sp)
    csrrs    a0, mepc, x0        # read and stack mepc
    sw       a0, 0x04(sp)
#_STORE_PRIO SUBROUTINE
    csrr     a0, 0x347           # load current threshold
    sw       a0, 0x08(sp)        # store old threshold on stack
    li       a0, 0x1000          # base address for the CLIC MMIO
    lb       a0, 27(a0)          # load prio config register of interrupt 6
    csrw     0x347, a0           # set the priority
    csrrsi   x0, mstatus, 8      # enable interrupts (end of critical section)
#END
    sw       ra, 0x0c(sp)        # stack the caller saved registers
    sw       a1, 0x14(sp)
    sw       a2, 0x18(sp)
    sw       a3, 0x1c(sp)
    sw       a4, 0x20(sp)
    sw       a5, 0x24(sp)
    sw       a6, 0x28(sp)
    sw       a7, 0x2c(sp)
    sw       t0, 0x30(sp)
    sw       t1, 0x34(sp)
    sw       t2, 0x38(sp)
    sw       t3, 0x3c(sp)
    sw       t4, 0x40(sp)
    sw       t5, 0x44(sp)
    sw       t6, 0x48(sp)

    jal      ra, Interrupt6           # call into the user defined handler

    lw       a0, 0x00(sp)        # restore CSRs and caller saved registers
    csrrw    x0, mstatus, a0
    lw       a0, 0x04(sp)
    csrrw    x0, mepc, a0
    lw       ra, 0x0c(sp)
    lw       a0, 0x10(sp)
    lw       a1, 0x14(sp)
    lw       a2, 0x18(sp)
    lw       a3, 0x1c(sp)
    lw       a4, 0x20(sp)
    lw       a5, 0x24(sp)
    lw       a6, 0x28(sp)
    lw       a7, 0x2c(sp)
    lw       t0, 0x30(sp)
    lw       t1, 0x34(sp)
    lw       t2, 0x38(sp)
    lw       t3, 0x3c(sp)
    lw       t4, 0x40(sp)
    lw       t5, 0x44(sp)
    #CS
    csrci    mstatus, 0x8     
    lw       t6, 0x08(sp)        # load the old threshold from the stack
    csrw     0x347, t6           # set the priority
    lw       t6, 0x48(sp)
    addi     sp, sp, 0x4c
    mret                       # return from interrupt
handler_7:
    addi     sp, sp, -0x4c       # allocate space for the context on the stack
    sw       a0, 0x10(sp)        # start by pushing a0 we it to stack CSRs and set threshold
    csrrs    a0, mstatus, x0     # read and stack mstatus
    sw       a0, 0x00(sp)
    csrrs    a0, mepc, x0        # read and stack mepc
    sw       a0, 0x04(sp)
#_STORE_PRIO SUBROUTINE
    csrr     a0, 0x347           # load current threshold
    sw       a0, 0x08(sp)        # store old threshold on stack
    li       a0, 0x1000          # base address for the CLIC MMIO
    lb       a0, 31(a0)          # load prio config register of interrupt 7
    csrw     0x347, a0           # set the priority
    csrrsi   x0, mstatus, 8      # enable interrupts (end of critical section)
#END
    sw       ra, 0x0c(sp)        # stack the caller saved registers
    sw       a1, 0x14(sp)
    sw       a2, 0x18(sp)
    sw       a3, 0x1c(sp)
    sw       a4, 0x20(sp)
    sw       a5, 0x24(sp)
    sw       a6, 0x28(sp)
    sw       a7, 0x2c(sp)
    sw       t0, 0x30(sp)
    sw       t1, 0x34(sp)
    sw       t2, 0x38(sp)
    sw       t3, 0x3c(sp)
    sw       t4, 0x40(sp)
    sw       t5, 0x44(sp)
    sw       t6, 0x48(sp)

    jal      ra, Interrupt7          # call into the user defined handler

    lw       a0, 0x00(sp)        # restore CSRs and caller saved registers
    csrrw    x0, mstatus, a0
    lw       a0, 0x04(sp)
    csrrw    x0, mepc, a0
    lw       ra, 0x0c(sp)
    lw       a0, 0x10(sp)
    lw       a1, 0x14(sp)
    lw       a2, 0x18(sp)
    lw       a3, 0x1c(sp)
    lw       a4, 0x20(sp)
    lw       a5, 0x24(sp)
    lw       a6, 0x28(sp)
    lw       a7, 0x2c(sp)
    lw       t0, 0x30(sp)
    lw       t1, 0x34(sp)
    lw       t2, 0x38(sp)
    lw       t3, 0x3c(sp)
    lw       t4, 0x40(sp)
    lw       t5, 0x44(sp)
    #CS
    csrci    mstatus, 0x8     
    lw       t6, 0x08(sp)        # load the old threshold from the stack
    csrw     0x347, t6           # set the priority
    lw       t6, 0x48(sp)
    addi     sp, sp, 0x4c
    mret                       # return from interrupt
handler_8:
    addi     sp, sp, -0x4c       # allocate space for the context on the stack
    sw       a0, 0x10(sp)        # start by pushing a0 we it to stack CSRs and set threshold
    csrrs    a0, mstatus, x0     # read and stack mstatus
    sw       a0, 0x00(sp)
    csrrs    a0, mepc, x0        # read and stack mepc
    sw       a0, 0x04(sp)
#_STORE_PRIO SUBROUTINE
    csrr     a0, 0x347           # load current threshold
    sw       a0, 0x08(sp)        # store old threshold on stack
    li       a0, 0x1000          # base address for the CLIC MMIO
    lb       a0, 35(a0)          # load prio config register of interrupt 8
    csrw     0x347, a0           # set the priority
    csrrsi   x0, mstatus, 8      # enable interrupts (end of critical section)
#END
    sw       ra, 0x0c(sp)        # stack the caller saved registers
    sw       a1, 0x14(sp)
    sw       a2, 0x18(sp)
    sw       a3, 0x1c(sp)
    sw       a4, 0x20(sp)
    sw       a5, 0x24(sp)
    sw       a6, 0x28(sp)
    sw       a7, 0x2c(sp)
    sw       t0, 0x30(sp)
    sw       t1, 0x34(sp)
    sw       t2, 0x38(sp)
    sw       t3, 0x3c(sp)
    sw       t4, 0x40(sp)
    sw       t5, 0x44(sp)
    sw       t6, 0x48(sp)

    jal      ra, Interrupt8           # call into the user defined handler

    lw       a0, 0x00(sp)        # restore CSRs and caller saved registers
    csrrw    x0, mstatus, a0
    lw       a0, 0x04(sp)
    csrrw    x0, mepc, a0
    lw       ra, 0x0c(sp)
    lw       a0, 0x10(sp)
    lw       a1, 0x14(sp)
    lw       a2, 0x18(sp)
    lw       a3, 0x1c(sp)
    lw       a4, 0x20(sp)
    lw       a5, 0x24(sp)
    lw       a6, 0x28(sp)
    lw       a7, 0x2c(sp)
    lw       t0, 0x30(sp)
    lw       t1, 0x34(sp)
    lw       t2, 0x38(sp)
    lw       t3, 0x3c(sp)
    lw       t4, 0x40(sp)
    lw       t5, 0x44(sp)
    #CS
    csrci    mstatus, 0x8     
    lw       t6, 0x08(sp)        # load the old threshold from the stack
    csrw     0x347, t6           # set the priority
    lw       t6, 0x48(sp)
    addi     sp, sp, 0x4c
    mret                       # return from interrupt
handler_9:
    addi     sp, sp, -0x4c       # allocate space for the context on the stack
    sw       a0, 0x10(sp)        # start by pushing a0 we it to stack CSRs and set threshold
    csrrs    a0, mstatus, x0     # read and stack mstatus
    sw       a0, 0x00(sp)
    csrrs    a0, mepc, x0        # read and stack mepc
    sw       a0, 0x04(sp)
#_STORE_PRIO SUBROUTINE
    csrr     a0, 0x347           # load current threshold
    sw       a0, 0x08(sp)        # store old threshold on stack
    li       a0, 0x1000          # base address for the CLIC MMIO
    lb       a0, 35(a0)          # load prio config register of interrupt 8
    csrw     0x347, a0           # set the priority
    csrrsi   x0, mstatus, 8      # enable interrupts (end of critical section)
#END
    sw       ra, 0x0c(sp)        # stack the caller saved registers
    sw       a1, 0x14(sp)
    sw       a2, 0x18(sp)
    sw       a3, 0x1c(sp)
    sw       a4, 0x20(sp)
    sw       a5, 0x24(sp)
    sw       a6, 0x28(sp)
    sw       a7, 0x2c(sp)
    sw       t0, 0x30(sp)
    sw       t1, 0x34(sp)
    sw       t2, 0x38(sp)
    sw       t3, 0x3c(sp)
    sw       t4, 0x40(sp)
    sw       t5, 0x44(sp)
    sw       t6, 0x48(sp)

    jal      ra, Interrupt9           # call into the user defined handler

    lw       a0, 0x00(sp)        # restore CSRs and caller saved registers
    csrrw    x0, mstatus, a0
    lw       a0, 0x04(sp)
    csrrw    x0, mepc, a0
    lw       ra, 0x0c(sp)
    lw       a0, 0x10(sp)
    lw       a1, 0x14(sp)
    lw       a2, 0x18(sp)
    lw       a3, 0x1c(sp)
    lw       a4, 0x20(sp)
    lw       a5, 0x24(sp)
    lw       a6, 0x28(sp)
    lw       a7, 0x2c(sp)
    lw       t0, 0x30(sp)
    lw       t1, 0x34(sp)
    lw       t2, 0x38(sp)
    lw       t3, 0x3c(sp)
    lw       t4, 0x40(sp)
    lw       t5, 0x44(sp)
    #CS
    csrci    mstatus, 0x8     
    lw       t6, 0x08(sp)        # load the old threshold from the stack
    csrw     0x347, t6           # set the priority
    lw       t6, 0x48(sp)
    addi     sp, sp, 0x4c
    mret                       # return from interrupt
"
);
