int vecpow_c(float *dout, int dout_maxlen, const float *din, int din_len) {
    // check memory allocation error ...
    if (dout_maxlen < din_len) {
        return 0;
    }

    // func
    for (int i=0; i<din_len; i++) {
        dout[i] = din[i]*din[i];
    }
    return din_len;
}
