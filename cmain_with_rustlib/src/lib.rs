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
    let tmp_in = unsafe { std::slice::from_raw_parts(din, din_len as usize) }; // .to_vec();
    let tmp_out = unsafe { std::slice::from_raw_parts_mut(dout, dout_maxlen as usize) };
    let tmp_out_vp = vecpow_rs(&tmp_in);

    tmp_out.clone_from_slice(&tmp_out_vp);
    tmp_out.len() as i32
}

// --------------- nocopy version ------------------

fn vecpow_rs_nocopy(dout: &mut [f32], din: &[f32]) -> usize {
    if dout.len() < din.len() {
        panic!("vecpow: in.len() >= out.len()");
    }
    for i in 0..din.len() {
        dout[i] = din[i] * din[i];
    }
    din.len()
}

#[no_mangle]
pub extern "C" fn vecpow_rs_ffi_nocopy(
    dout: &mut f32,
    dout_maxlen: i32,
    din: &mut f32,
    din_len: i32,
) -> i32 {
    let tmp_in = unsafe { std::slice::from_raw_parts(din, din_len as usize) };
    let mut tmp_out = unsafe { std::slice::from_raw_parts_mut(dout, dout_maxlen as usize) };

    vecpow_rs_nocopy(&mut tmp_out, &tmp_in) as i32
}
