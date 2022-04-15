#[macro_use]
extern crate lazy_static;
use parking_lot::Mutex;
mod soundchip;

const DEFAULT_SAMPLE_RATE: f32 = 44100_f32;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// Init the mutex has SoundChip instance
lazy_static! {
    static ref SOUNDCHIP: Mutex<soundchip::SoundChip> =
        Mutex::new(soundchip::SoundChip::new(DEFAULT_SAMPLE_RATE));
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
    outptr0: *mut f32, outptr1: *mut f32, sample_count: u32
)
{
    let mut soundchip = SOUNDCHIP.lock();
    soundchip.process(outptr0, outptr1, sample_count);
}

#[no_mangle]
pub extern "C" fn set_sample_rate(sample_rate: f32) {
    let mut soundchip = SOUNDCHIP.lock();
    soundchip.set_sample_rate(sample_rate);
}

#[no_mangle]
pub extern "C" fn set_freq(id: usize, freq: f32) {
    let mut soundchip = SOUNDCHIP.lock();
    soundchip.set_freq(id, freq);
}

#[no_mangle]
pub extern "C" fn set_vol(id: usize, ch: usize, vol: u8) {
    let mut soundchip = SOUNDCHIP.lock();
    soundchip.set_vol(id, ch, vol);
}

#[no_mangle]
pub extern "C" fn set_mute(id: usize, mute: usize) {
    let mut soundchip = SOUNDCHIP.lock();
    let mute_bool = match mute % 2 {
        0 => false,
        1 => true,
        _ => unreachable!()
    };
    soundchip.set_mute(id, mute_bool);
}

#[no_mangle]
pub extern "C" fn set_param(id: usize, key: usize, value: u32) {
    let mut soundchip = SOUNDCHIP.lock();
    soundchip.set_param(id, key, value);
}
