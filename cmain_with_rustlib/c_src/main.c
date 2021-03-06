#include <stdio.h>

// pub fn vecpow_rs_ffi(dout: &mut f32, dout_maxlen: i32, din: &mut f32, din_len: i32) -> i32 {
int vecpow_rs_ffi(float *dout, int dout_maxlen, const float *din, int din_len);
int vecpow_rs_ffi_nocopy(float *dout, int dout_maxlen, const float *din, int din_len);

#define NLEN 32
int main() {
    float nums[NLEN];
    float pows[NLEN];
    for (int i=0; i<NLEN; i++) { nums[i] = i; }

    puts("Nums: ");
    for (int i=0; i<NLEN; i++) { printf("%.0f ", nums[i]); }
    puts("\n");

    int len = vecpow_rs_ffi(pows, NLEN, nums, NLEN);

    puts("Pows: ");
    for (int i=0; i<len; i++) { printf("%.0f ", pows[i]); }
    puts("\n");


    // --- nocopy fn test ---


    float pows_nocopy[NLEN];
    int len_nocopy = vecpow_rs_ffi_nocopy(pows_nocopy, NLEN, nums, NLEN);

    puts("Pows nocopy: ");
    for (int i=0; i<len_nocopy; i++) { printf("%.0f ", pows_nocopy[i]); }
    puts("\n");
}
