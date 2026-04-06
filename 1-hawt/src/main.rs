#![no_std]
#![no_main]

mod fractal_feedback;
mod moisture_accumulator;

use core::arch::asm;
use core::panic::PanicInfo;
use crate::fractal_feedback::FractalManifold;
use crate::moisture_accumulator::MoistureAccumulator;

static mut MANIFOLD: FractalManifold = FractalManifold::new();
static mut ACCUMULATOR: MoistureAccumulator = MoistureAccumulator::new();

#[inline(always)]
unsafe fn sync_msr(msr: u32, value: u64) {
    let low = (value & 0xFFFFFFFF) as u32;
    let high = (value >> 32) as u32;
    asm!("wrmsr", in("ecx") msr, in("eax") low, in("edx") high);
}

#[inline(always)]
unsafe fn read_thermal() -> f64 {
    let eax: u32;
    asm!("rdmsr", in("ecx") 0x19C, out("eax") eax, out("edx") _);
    (eax as f64) / 127.0
}

#[no_mangle]
pub unsafe extern "efiapi" fn efi_main(_handle: *const core::ffi::c_void, 
_st: *const core::ffi::c_void) -> usize {
    asm!("cli");
    sync_msr(0x199, 0x13D4);
    
    loop {
        let entropy = read_thermal();
        let resonance = MANIFOLD.calculate_fractal_step(entropy);
        let _water = ACCUMULATOR.track_condensation(entropy, &mut 
MANIFOLD.drift);
        
        let v_out = (resonance * 1000.0) as u32;
        asm!("out dx, eax", in("dx") 0xCF8_u16, in("eax") v_out);
        
        if MANIFOLD.drift.abs() < 0.0000000001 {
            asm!("wbinvd");
        }
        asm!("pause");
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop { unsafe { asm!("pause"); } }
}

