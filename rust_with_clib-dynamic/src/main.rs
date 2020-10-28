#[link(name = "vecpow_dyn")]
extern "C" {
    // int vecpow_c(float *dout, int dout_maxlen, const float *din, int din_len)
    fn vecpow_c(dout: *mut f32, dout_maxlen: i32, din: *const f32, din_len: i32) -> i32;
}

pub fn vecpow(din: &[f32]) -> Vec<f32> {
    let mut dout: Vec<f32> = Vec::with_capacity(din.len());
    unsafe {
        let outlen = vecpow_c(
            dout.as_mut_ptr(),
            dout.capacity() as i32,
            din.as_ptr(),
            din.len() as i32,
        );
        dout.set_len(outlen as usize);
    }
    dout
}

fn main() {
    let din: Vec<_> = (0..32).map(|x| x as f32).collect();
    let dout = vecpow(&din);

    println!("Nums: {:?}", din);
    println!("Pows: {:?}", dout);
}
