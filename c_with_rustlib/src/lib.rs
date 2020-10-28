fn vecpow_rs(din: &[f32]) -> Vec<f32> {
    let mut dout = Vec::with_capacity(din.len());
    for d in din {
        dout.push(d * d);
    }
    dout
}

#[no_mangle]
pub extern "C" fn vecpow_rs_ffi(
    dout: &mut f32,
    dout_maxlen: i32,
    din: &mut f32,
    din_len: i32,
) -> i32 {
    let tmp_in = unsafe { std::slice::from_raw_parts(din, din_len as usize).to_vec() };

    let tmp_out = vecpow_rs(&tmp_in);

    if dout_maxlen < tmp_out.len() as i32 {
        panic!("vecpow: len >= out_maxlen");
    }

    unsafe {
        std::ptr::copy(tmp_out.as_ptr(), dout, tmp_out.len());
    }
    tmp_out.len() as i32
}
