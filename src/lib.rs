use wasm_bindgen::prelude::*;

use encoding::{ByteWriter, DecoderTrap, EncoderTrap, Encoding, RawEncoder};
use encoding::all::{UTF_8, WINDOWS_31J};

#[allow(dead_code)]
fn inspect(_encoder: &mut dyn RawEncoder, input: &str, output: &mut dyn ByteWriter) -> bool {
    println!("{:?}", input);
    println!("{:x?}", input.chars().next().unwrap() as u32);
    println!("{:x?}", input.as_bytes());
    output.write_bytes(input.as_bytes());
    true
}

fn fix_private_use_ibm_ext(_encoder: &mut dyn RawEncoder, input: &str, output: &mut dyn ByteWriter) -> bool {
    for c in input.chars() {
        let c = c as u32;
        macro_rules! priv_range_to_cp932 {
            ($($start:expr,$end:expr => $base:expr)+) => {
                match c {
                    $($start..=$end => {
                        let o = (c - $start + $base) as u16;
                        output.write_bytes(&o.to_be_bytes());
                    },)*
                    _ => {
                        // copied from EncoderTrap::Replace 
                        if _encoder.is_ascii_compatible() { // optimization!
                            output.write_bytes(input.as_bytes());
                        } else {
                            let (_, err) = _encoder.raw_feed(input, output);
                            if err.is_some() {
                                panic!("{} cannot reencode a replacement string", "Call(fix_private_use_ibm_ext)");
                            }
                        }
                    }
                }
            };
        }
        priv_range_to_cp932! {
            0xE000, 0xE03E => 0xF040
            0xE03F, 0xE0BB => 0xF080
            0xE0BC, 0xE0FA => 0xF140
            0xE0FB, 0xE177 => 0xF180
            0xE178, 0xE1B6 => 0xF240
            0xE1B7, 0xE233 => 0xF280
            0xE234, 0xE272 => 0xF340
            0xE273, 0xE2EF => 0xF380
            0xE2F0, 0xE32E => 0xF440
            0xE32F, 0xE3AB => 0xF480
            0xE3AC, 0xE3EA => 0xF540
            0xE3EB, 0xE467 => 0xF580
            0xE468, 0xE4A6 => 0xF640
            0xE4A7, 0xE523 => 0xF680
            0xE524, 0xE562 => 0xF740
            0xE563, 0xE5DF => 0xF780
            0xE5E0, 0xE61E => 0xF840
            0xE61F, 0xE69B => 0xF880
            0xE69C, 0xE6DA => 0xF940
            0xE6DB, 0xE757 => 0xF980
        }
    }
    true
}

#[wasm_bindgen]
pub fn bakefix(bake: &str) -> String {
    let buf = WINDOWS_31J.encode(&bake, EncoderTrap::Call(fix_private_use_ibm_ext)).unwrap();
    // println!("{:x?}", buf);
    let fixed: String = UTF_8.decode(&buf, DecoderTrap::Replace).unwrap();
    return fixed;
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn it_works() {
        let bake = "繝舌ち縺阪＠縺ｦ";
        assert_eq!(bakefix(bake), "バタきして");
    }

    #[test]
    fn case1() {
        let bake = "套縺ｫ";
        // println!("{:x?}", bake.as_bytes());
        let text = "📅に";
        // println!("{:x?}", text.as_bytes());
        assert_eq!(bakefix(bake), text);
    }

    #[test]
    fn case2() {
        let bake = "繩Å據樞攅繻ｹ繧ｯ繝ｩ繧､";
        // println!("{:X?}", bake.chars().map(|c| c as u32).collect::<Vec<_>>());
        let text = "㊁𝟞❷㌹クライ";
        // println!("{:x?}", text.as_bytes());
        assert_eq!(bakefix(bake), text);
    }

    #[test]
    fn case3() {
        let bake = "縺薙■繧峨°繧峨?繝｡繝ｼ繝ｫ繧偵??蜿励￠蜿悶▲縺溘ｉ莉悶??縺ｮ蜈ｱ蜑ｵ閠???驕斐→蜈ｱ譛峨＠縺ｦ谺ｲ縺励＞縲";
        println!("{}", bakefix(bake));
    }

}
