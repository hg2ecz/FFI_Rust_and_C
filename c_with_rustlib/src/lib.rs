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
    let mut tmp_in = Vec::with_capacity(din_len as usize);
    unsafe {
        std::ptr::copy(din, tmp_in.as_mut_ptr(), din_len as usize);
        tmp_in.set_len(din_len as usize);
    }

    let tmp_out = vecpow_rs(&tmp_in);

    if dout_maxlen >= tmp_out.len() as i32 {
        unsafe {
            std::ptr::copy(tmp_out.as_ptr(), dout, tmp_out.len());
        }
        tmp_out.len() as i32
    } else {
        panic!("vecpow: len >= out_maxlen");
        //0
    }
}
