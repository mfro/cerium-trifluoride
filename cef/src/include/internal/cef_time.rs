// simple_struct!(Time, time, );

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct CefTime {
    pub raw: cef_sys::cef_time_t
}

impl From<CefTime> for cef_sys::cef_time_t {
    fn from(src: CefTime) -> cef_sys::cef_time_t {
        src.raw
    }
}

impl From<cef_sys::cef_time_t> for CefTime {
    fn from(raw: cef_sys::cef_time_t) -> CefTime {
        CefTime { raw }
    }
}

#[rustfmt::skip]
impl CefTime {
    pub fn year(&self) -> std::os::raw::c_int { self.raw.year }
    pub fn month(&self) -> std::os::raw::c_int { self.raw.month }
    pub fn day_of_week(&self) -> std::os::raw::c_int { self.raw.day_of_week }
    pub fn day_of_month(&self) -> std::os::raw::c_int { self.raw.day_of_month }
    pub fn hour(&self) -> std::os::raw::c_int { self.raw.hour }
    pub fn minute(&self) -> std::os::raw::c_int { self.raw.minute }
    pub fn second(&self) -> std::os::raw::c_int { self.raw.second }
    pub fn millisecond(&self) -> std::os::raw::c_int { self.raw.millisecond }

    pub fn set_year(&mut self, v: std::os::raw::c_int) -> &mut Self { self.raw.year = v; self }
    pub fn set_month(&mut self, v: std::os::raw::c_int) -> &mut Self { self.raw.month = v; self }
    pub fn set_day_of_week(&mut self, v: std::os::raw::c_int) -> &mut Self { self.raw.day_of_week = v; self }
    pub fn set_day_of_month(&mut self, v: std::os::raw::c_int) -> &mut Self { self.raw.day_of_month = v; self }
    pub fn set_hour(&mut self, v: std::os::raw::c_int) -> &mut Self { self.raw.hour = v; self }
    pub fn set_minute(&mut self, v: std::os::raw::c_int) -> &mut Self { self.raw.minute = v; self }
    pub fn set_second(&mut self, v: std::os::raw::c_int) -> &mut Self { self.raw.second = v; self }
    pub fn set_millisecond(&mut self, v: std::os::raw::c_int) -> &mut Self { self.raw.millisecond = v; self }
}
