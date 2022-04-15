const SAMPLE_BUFSIZE: usize = 128;

use aoec::soundchips::BuiltIn;
use aoec::traits::Play;
use aoec::traits::Control;

// Sound chip example. it generates GB style white noise.
pub struct SoundChip {
    builtin_0: BuiltIn,
    builtin_1: BuiltIn,
    builtin_2: BuiltIn,
}

impl SoundChip {
    pub fn new(sample_rate: f32) -> SoundChip {
        let mut new = SoundChip {
            builtin_0: BuiltIn::new(sample_rate),
            builtin_1: BuiltIn::new(sample_rate),
            builtin_2: BuiltIn::new(sample_rate),
        };
        new.builtin_0.set_freq(440_f32);
        new
    }
    pub fn process(&mut self,
        outptr0: *mut f32, outptr1: *mut f32, _sample_count: u32
    )
    {
        // borrow output buffers from outptr pointers.
        let outbuf0: &mut [f32] = unsafe {
            std::slice::from_raw_parts_mut(outptr0, SAMPLE_BUFSIZE)
        };
        let outbuf1: &mut [f32] = unsafe {
            std::slice::from_raw_parts_mut(outptr1, SAMPLE_BUFSIZE)
        };

        for i in 0..SAMPLE_BUFSIZE {
            // Clock all soundchips
            self.builtin_0.clock();
            self.builtin_1.clock();
            self.builtin_2.clock();

            // Merge left samples from all soundchips
            let mut sample0 = 0_f32;
            sample0 += self.builtin_0.read_sample(0);
            sample0 += self.builtin_1.read_sample(0);
            sample0 += self.builtin_2.read_sample(0);

            // Merge right samples from all soundchips
            let mut sample1 = 0_f32;
            sample1 += self.builtin_0.read_sample(1);
            sample1 += self.builtin_1.read_sample(1);
            sample1 += self.builtin_2.read_sample(1);

            // Output merged samples
            outbuf0[i] = sample0;
            outbuf1[i] = sample1;
        };
    }

    pub fn set_sample_rate(&mut self, sample_rate: f32) {
        self.builtin_0.set_sample_rate(sample_rate);
        self.builtin_1.set_sample_rate(sample_rate);
        self.builtin_2.set_sample_rate(sample_rate);
    }

    pub fn set_freq(&mut self, id: usize, freq: f32) {
        match id {
            0 => self.builtin_0.set_freq(freq),
            1 => self.builtin_1.set_freq(freq),
            2 => self.builtin_2.set_freq(freq),
            _ => (),
        }
    }

    pub fn set_vol(&mut self, id: usize, ch: usize, vol: u8) {
        match id {
            0 => self.builtin_0.set_vol(ch, vol),
            1 => self.builtin_1.set_vol(ch, vol),
            2 => self.builtin_2.set_vol(ch, vol),
            _ => (),
        }
    }

    pub fn set_mute(&mut self, id: usize, mute: bool) {
        match id {
            0 => self.builtin_0.set_mute(mute),
            1 => self.builtin_1.set_mute(mute),
            2 => self.builtin_2.set_mute(mute),
            _ => (),
        }
    }

    pub fn set_param(&mut self, id: usize, key: usize, value: u32) {
        match id {
            0 => self.builtin_0.set_param(key, value),
            1 => self.builtin_1.set_param(key, value),
            2 => self.builtin_2.set_param(key, value),
            _ => (),
        }
    }
}
