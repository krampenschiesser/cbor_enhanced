use bytes::BytesMut;

pub struct SerializePrimitiveSeq {
    pub bytes_buffer: BytesMut,
    pub f32array: Vec<f32>,
    pub f64array: Vec<f64>,
    pub u8array: Vec<u8>,
    pub u16array: Vec<u16>,
    pub u32array: Vec<u32>,
    pub u64array: Vec<u64>,
    pub i8array: Vec<i8>,
    pub i16array: Vec<i16>,
    pub i32array: Vec<i32>,
    pub i64array: Vec<i64>,
}
impl SerializePrimitiveSeq {
    pub fn new() -> Self {
        Self {
            bytes_buffer: BytesMut::new(),
            f32array: Vec::new(),
            f64array: Vec::new(),
            u8array: Vec::new(),
            u16array: Vec::new(),
            u32array: Vec::new(),
            u64array: Vec::new(),
            i8array: Vec::new(),
            i16array: Vec::new(),
            i32array: Vec::new(),
            i64array: Vec::new(),
        }
    }
    pub fn is_same_kind(&self) -> bool {
        let callback = |len| {
            if len > 0 {
                1
            } else {
                0
            }
        };
        let mut total = 0;
        total += callback(self.u8array.len());
        total += callback(self.u16array.len());
        total += callback(self.u32array.len());
        total += callback(self.u64array.len());
        total += callback(self.i8array.len());
        total += callback(self.i16array.len());
        total += callback(self.i32array.len());
        total += callback(self.i64array.len());
        total += callback(self.f32array.len());
        total += callback(self.f64array.len());
        total <= 1
    }
    pub fn write_f32(&mut self, value: f32) {
        self.f32array.push(value);
    }
    pub fn write_f64(&mut self, value: f64) {
        self.f64array.push(value);
    }
    pub fn write_u8(&mut self, value: u8) {
        self.u8array.push(value);
    }
    pub fn write_u16(&mut self, value: u16) {
        self.u16array.push(value);
    }
    pub fn write_u32(&mut self, value: u32) {
        self.u32array.push(value);
    }
    pub fn write_u64(&mut self, value: u64) {
        self.u64array.push(value);
    }
    pub fn write_i8(&mut self, value: i8) {
        self.i8array.push(value);
    }
    pub fn write_i16(&mut self, value: i16) {
        self.i16array.push(value);
    }
    pub fn write_i32(&mut self, value: i32) {
        self.i32array.push(value);
    }
    pub fn write_i64(&mut self, value: i64) {
        self.i64array.push(value);
    }
}
