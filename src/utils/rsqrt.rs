#[inline(always)]
pub fn rsqrt(x: f32) -> f32 {
    // i dont get it, but you know. inverse square root.
    let x2 = x * 0.5;
    let y = f32::from_bits(0x5f3759df_u32.wrapping_sub(x.to_bits() >> 1));
    y * (1.5 - x2 * y * y)
}
