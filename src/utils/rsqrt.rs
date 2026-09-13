#[inline(always)]
pub fn rsqrt(x: f32) -> f32 {
    let x2 = x * 0.5; // half of input
    let y = f32::from_bits(0x5f3759df_u32.wrapping_sub(x.to_bits() >> 1)); // magic
    y * (1.5 - x2 * y * y) // newton refinement
}
