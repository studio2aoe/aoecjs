#[macro_use]
extern crate lazy_static;
use parking_lot::Mutex;

use aoec::soundchips::BuiltIn;
use aoec::traits::Play;
use aoec::traits::Control;

const DEFAULT_SAMPLE_RATE: u32 = 48000_u32;
const SAMPLE_BUFSIZE: usize = 128;

// Init the mutex has aoec builtin chip
lazy_static! {
    static ref AOEC_BUILTIN: Mutex<BuiltIn>
        = Mutex::new(BuiltIn::new(DEFAULT_SAMPLE_RATE));
}

#[no_mangle]
pub extern "C" fn outbuf_alloc(size: usize) -> *mut f32 {
    let mut buffer = Vec::<f32>::with_capacity(size);
    let ptr = buffer.as_mut_ptr();
    std::mem::forget(buffer);
    return ptr as *mut f32
}

#[no_mangle]
pub extern "C" fn process(
    outptr0: *mut f32, outptr1: *mut f32, _sample_count: u32
)
{
    let mut aoec = AOEC_BUILTIN.lock();

    let outbuf0: &mut [f32] = unsafe {
        std::slice::from_raw_parts_mut(outptr0, SAMPLE_BUFSIZE)
    };
    let outbuf1: &mut [f32] = unsafe {
        std::slice::from_raw_parts_mut(outptr1, SAMPLE_BUFSIZE)
    };

    for i in 0..SAMPLE_BUFSIZE {
        aoec.clock();
        outbuf0[i] = aoec.read_sample(0);
        outbuf1[i] = aoec.read_sample(1);
    }

}

#[no_mangle]
pub extern "C" fn setparam(key: u32, value: u32) {
    let mut aoec = AOEC_BUILTIN.lock();
    aoec.setparam(key, value);
}
