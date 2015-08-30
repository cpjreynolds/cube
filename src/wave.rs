use time;

use num::{
    One,
    Zero,
};

use gel::PI_2;

#[derive(Debug, Clone)]
pub struct Builder {
    inner: Wave,
}

impl Builder {
    pub fn new() -> Builder {
        Builder {
            inner: Wave::default(),
        }
    }

    pub fn amplitude(&mut self, amp: f32) -> &mut Builder {
        self.inner.set_amplitude(amp);
        self
    }

    pub fn period(&mut self, period: f32) -> &mut Builder {
        self.inner.set_period(period);
        self
    }

    pub fn pshift(&mut self, pshift: f32) -> &mut Builder {
        self.inner.set_pshift(pshift);
        self
    }

    pub fn vshift(&mut self, vshift: f32) -> &mut Builder {
        self.inner.set_vshift(vshift);
        self
    }

    pub fn build(&self) -> Wave {
        self.inner.clone()
    }
}

// f(x) = A * func(Bx - C) + D
#[derive(Debug, Clone)]
pub struct Wave {
    a: f32, // Amplitude
    b: f32, // Period = (2 * PI) / b
    c: f32, // Phase shift = c / b
    d: f32, // Vertical shift
}

impl Wave {
    pub fn set_amplitude(&mut self, amp: f32) -> &mut Wave {
        self.a = amp;
        self
    }

    pub fn amplitude(&self) -> f32 {
        self.a
    }

    pub fn set_period(&mut self, period: f32) -> &mut Wave {
        self.b = PI_2 / period;
        // Update phase shift to reflect new period.
        let p = self.pshift();
        self.set_pshift(p);
        self
    }

    pub fn period(&self) -> f32 {
        PI_2 / self.b
    }

    pub fn set_pshift(&mut self, pshift: f32) -> &mut Wave {
        self.c = pshift * self.b.abs();
        self
    }

    pub fn pshift(&self) -> f32 {
        self.c / self.b.abs()
    }

    pub fn set_vshift(&mut self, vshift: f32) -> &mut Wave {
        self.d = vshift;
        self
    }

    pub fn vshift(&self) -> f32 {
        self.d
    }

    pub fn eval_with(&self, x: f32) -> f32 {
        self.a * f32::sin(self.b * x - self.c) + self.d
    }

    pub fn eval(&self) -> f32 {
        let t = time::precise_time_s() as f32;
        self.eval_with(t)
    }
}

impl Default for Wave {
    fn default() -> Wave {
        Wave {
            a: f32::one(),
            b: f32::one(),
            c: f32::zero(),
            d: f32::zero(),
        }
    }
}

