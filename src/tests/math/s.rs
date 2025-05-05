// Written on 2025/3/23
use crate::math::phys::sound::speed_of_sound_f32;

#[allow(non_snake_case)]
#[test]
fn S449() {
    let S = speed_of_sound_f32(196.666666);

    assert_eq!(S, 449.0);
}
