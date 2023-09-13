#![no_std]
use core::arch::global_asm;

use riscv::register::mtvec;


#[no_mangle]
pub unsafe fn _setup_interrupts() {
    mtvec::write(
        &_VECTOR_TABLE.handler0 as *const _ as usize,
        mtvec::TrapMode::Vectored,
    )
}
#[repr(C)]
struct VectorTable {
    pub handler0: unsafe extern "C" fn(),
    pub handler1: unsafe extern "C" fn(),
    pub handler2: unsafe extern "C" fn(),
    pub handler3: unsafe extern "C" fn(),
}

extern "C" {
    fn handler_0();
    fn handler_1();
    fn handler_2();
    fn handler_3();
}

//must be accessible over data bus, for now have it here since .text isn't
#[link_section = ".data"]
static _VECTOR_TABLE: VectorTable = VectorTable {
    handler0: handler_0,
    handler1: handler_1,
    handler2: handler_2,
    handler3: handler_3,
};

//ENEST traps the user is expected to provide _interruptx symbols for all x
global_asm!("
.text
handler_0:
        addi    sp, sp, -0x4c   # allocate space for the context on the stack
        sw      a0, 0x10(sp)    # start by pushing a0 we it to stack CSRs and set threshold
        csrrs   a0, mstatus, x0 # read and stack mstatus 
        sw      a0, 0x00(sp)      
        csrrs   a0, mepc, x0    # read and stack mepc
        sw      a0, 0x04(sp)
        #_STORE_PRIO SUBROUTINE
		csrr	a0, 0x347	# load current threshold
    		sw      a0, 0x08(sp)    # store old threshold on stack
		li     	a0, 0x1000   	# base address for the CLIC MMIO
            	lb 	a0, 3(a0)	# load prio config register of interrupt 0
            	csrw    0x347, a0   	# set the priority
            	csrrsi  x0, mstatus, 8  # enable interrupts (end of critical section)
        #END
        sw      ra, 0x0c(sp)    # stack the caller saved registers
		sw      a1, 0x14(sp)
        sw      a2, 0x18(sp)
        sw      a3, 0x1c(sp)
        sw      a4, 0x20(sp)
        sw      a5, 0x24(sp)
        sw      a6, 0x28(sp)
        sw      a7, 0x2c(sp)
        sw      t0, 0x30(sp)
        sw      t1, 0x34(sp)
        sw      t2, 0x38(sp)
        sw      t3, 0x3c(sp)
        sw      t4, 0x40(sp)
        sw      t5, 0x44(sp)
        sw      t6, 0x48(sp)
        jal     ra, _interrupt0   # call into the user defined handler

	#RETURN PRIO SUBROUTINE
            lw      a0, 0x08(sp)    	# load the old threshold from the stack
            csrw    0x347, a0   	# set the priority
        #END
    
        lw      a0, 0x00(sp)        # restore CSRs and caller saved registers
        csrrw   x0, mstatus, a0
        lw      a0, 0x04(sp)      
        csrrw   x0, mepc, a0
        lw      ra, 0x0c(sp)
        lw      a0, 0x10(sp)
        lw      a1, 0x14(sp)
        lw      a2, 0x18(sp)
        lw      a3, 0x1c(sp)
        lw      a4, 0x20(sp)
        lw      a5, 0x24(sp)
        lw      a6, 0x28(sp)
        lw      a7, 0x2c(sp)
        lw      t0, 0x30(sp)
        lw      t1, 0x34(sp)
        lw      t2, 0x38(sp)
        lw      t3, 0x3c(sp)
        lw      t4, 0x40(sp)
        lw      t5, 0x44(sp)
        lw      t6, 0x48(sp)
        addi    sp, sp, 0x4c      
        mret                        # return from interrupt

handler_1:
        addi    sp, sp, -0x4c   # allocate space for the context on the stack
        sw      a0, 0x10(sp)    # start by pushing a0 we it to stack CSRs and set threshold
        csrrs   a0, mstatus, x0 # read and stack mstatus 
        sw      a0, 0x00(sp)      
        csrrs   a0, mepc, x0    # read and stack mepc
        sw      a0, 0x04(sp)
        #_STORE_PRIO SUBROUTINE
		csrr	a0, 0x347	# load current threshold
        	sw      a0, 0x08(sp)    # store old threshold on stack
		li     	a0, 0x1000   	# base address for the CLIC MMIO
            	lb 	a0, 7(a0)	# load prio config register of interrupt 1
            	csrw    0x347, a0   	# set the priority
            	csrrsi  x0, mstatus, 8  # enable interrupts (end of critical section)
        #END
        sw      ra, 0x0c(sp)    # stack the caller saved registers
		sw      a1, 0x14(sp)
        sw      a2, 0x18(sp)
        sw      a3, 0x1c(sp)
        sw      a4, 0x20(sp)
        sw      a5, 0x24(sp)
        sw      a6, 0x28(sp)
        sw      a7, 0x2c(sp)
        sw      t0, 0x30(sp)
        sw      t1, 0x34(sp)
        sw      t2, 0x38(sp)
        sw      t3, 0x3c(sp)
        sw      t4, 0x40(sp)
        sw      t5, 0x44(sp)
        sw      t6, 0x48(sp)
        jal     ra, _interrupt1   	# call into the user defined handler

	#RETURN PRIO SUBROUTINE
            	lw      a0, 0x08(sp)    # load the old threshold from the stack
		csrw    0x347, a0   	# set the priority
        #END
    
        lw      a0, 0x00(sp)	# restore CSRs and caller saved registers
        csrrw   x0, mstatus, a0
        lw      a0, 0x04(sp)      
        csrrw   x0, mepc, a0
        lw      ra, 0x0c(sp)
        lw      a0, 0x10(sp)
        lw      a1, 0x14(sp)
        lw      a2, 0x18(sp)
        lw      a3, 0x1c(sp)
        lw      a4, 0x20(sp)
        lw      a5, 0x24(sp)
        lw      a6, 0x28(sp)
        lw      a7, 0x2c(sp)
        lw      t0, 0x30(sp)
        lw      t1, 0x34(sp)
        lw      t2, 0x38(sp)
        lw      t3, 0x3c(sp)
        lw      t4, 0x40(sp)
        lw      t5, 0x44(sp)
        lw      t6, 0x48(sp)
        addi    sp, sp, 0x4c      
        mret                        # return from interrupt

handler_2:
        addi    sp, sp, -0x4c   # allocate space for the context on the stack
        sw      a0, 0x10(sp)    # start by pushing a0 we it to stack CSRs and set threshold
        csrrs   a0, mstatus, x0 # read and stack mstatus 
        sw      a0, 0x00(sp)      
        csrrs   a0, mepc, x0    # read and stack mepc
        sw      a0, 0x04(sp)
        #_STORE_PRIO SUBROUTINE
		csrr	a0, 0x347	# load current threshold
            	sw      a0, 0x08(sp)    # store old threshold on stack
		li     	a0, 0x1000   	# base address for the CLIC MMIO
            	lb 	a0, 11(a0)	# load prio config register of interrupt 1
            	csrw    0x347, a0   	# set the priority
            	csrrsi  x0, mstatus, 8  # enable interrupts (end of critical section)
        #END
        sw      ra, 0x0c(sp)    # stack the caller saved registers
		sw      a1, 0x14(sp)
        sw      a2, 0x18(sp)
        sw      a3, 0x1c(sp)
        sw      a4, 0x20(sp)
        sw      a5, 0x24(sp)
        sw      a6, 0x28(sp)
        sw      a7, 0x2c(sp)
        sw      t0, 0x30(sp)
        sw      t1, 0x34(sp)
        sw      t2, 0x38(sp)
        sw      t3, 0x3c(sp)
        sw      t4, 0x40(sp)
        sw      t5, 0x44(sp)
        sw      t6, 0x48(sp)
        jal     ra, _interrupt2   	# call into the user defined handler

	#RETURN PRIO SUBROUTINE
            lw      a0, 0x08(sp)    	# load the old threshold from the stack
            csrw    0x347, a0   	# set the priority
        #END
    
        lw      a0, 0x00(sp)        # restore CSRs and caller saved registers
        csrrw   x0, mstatus, a0
        lw      a0, 0x04(sp)      
        csrrw   x0, mepc, a0
        lw      ra, 0x0c(sp)
        lw      a0, 0x10(sp)
        lw      a1, 0x14(sp)
        lw      a2, 0x18(sp)
        lw      a3, 0x1c(sp)
        lw      a4, 0x20(sp)
        lw      a5, 0x24(sp)
        lw      a6, 0x28(sp)
        lw      a7, 0x2c(sp)
        lw      t0, 0x30(sp)
        lw      t1, 0x34(sp)
        lw      t2, 0x38(sp)
        lw      t3, 0x3c(sp)
        lw      t4, 0x40(sp)
        lw      t5, 0x44(sp)
        lw      t6, 0x48(sp)
        addi    sp, sp, 0x4c      
        mret                        # return from interrupt
handler_3:
        addi    sp, sp, -0x4c   # allocate space for the context on the stack
        sw      a0, 0x10(sp)    # start by pushing a0 we it to stack CSRs and set threshold
        csrrs   a0, mstatus, x0 # read and stack mstatus 
        sw      a0, 0x00(sp)      
        csrrs   a0, mepc, x0    # read and stack mepc
        sw      a0, 0x04(sp)
        #_STORE_PRIO SUBROUTINE
		csrr	a0, 0x347	# load current threshold
            	sw      a0, 0x08(sp)    # store old threshold on stack
		li     	a0, 0x1000   	# base address for the CLIC MMIO
            	lb 	a0, 11(a0)	# load prio config register of interrupt 1
            	csrw    0x347, a0   	# set the priority
            	csrrsi  x0, mstatus, 8  # enable interrupts (end of critical section)
        #END
        sw      ra, 0x0c(sp)    # stack the caller saved registers
		sw      a1, 0x14(sp)
        sw      a2, 0x18(sp)
        sw      a3, 0x1c(sp)
        sw      a4, 0x20(sp)
        sw      a5, 0x24(sp)
        sw      a6, 0x28(sp)
        sw      a7, 0x2c(sp)
        sw      t0, 0x30(sp)
        sw      t1, 0x34(sp)
        sw      t2, 0x38(sp)
        sw      t3, 0x3c(sp)
        sw      t4, 0x40(sp)
        sw      t5, 0x44(sp)
        sw      t6, 0x48(sp)
        jal     ra, _interrupt3  	# call into the user defined handler

	#RETURN PRIO SUBROUTINE
            lw      a0, 0x08(sp)    	# load the old threshold from the stack
            csrw    0x347, a0   	# set the priority
        #END
    
        lw      a0, 0x00(sp)        # restore CSRs and caller saved registers
        csrrw   x0, mstatus, a0
        lw      a0, 0x04(sp)      
        csrrw   x0, mepc, a0
        lw      ra, 0x0c(sp)
        lw      a0, 0x10(sp)
        lw      a1, 0x14(sp)
        lw      a2, 0x18(sp)
        lw      a3, 0x1c(sp)
        lw      a4, 0x20(sp)
        lw      a5, 0x24(sp)
        lw      a6, 0x28(sp)
        lw      a7, 0x2c(sp)
        lw      t0, 0x30(sp)
        lw      t1, 0x34(sp)
        lw      t2, 0x38(sp)
        lw      t3, 0x3c(sp)
        lw      t4, 0x40(sp)
        lw      t5, 0x44(sp)
        lw      t6, 0x48(sp)
        addi    sp, sp, 0x4c      
        mret                        # return from interrupt
");