use std::ops::Deref;

use testingdep::BENCHMARKS;

fn main() {
    println!("{}", testingdep::x::ROBOTO_FONT_LIGHT_FONT);

    println!("{}", BENCHMARKS.len());
    println!("{:?}", BENCHMARKS.deref());

    println!("{}", std::str::from_utf8(BENCHMARKS.deref()).unwrap());
}

// mod inner2 {
//     #[used]
//     #[link_section = "__DATA,manganis,regular,no_dead_strip"]
//     static BENCH_DESERIALIZE: [u8; 7] = {
//         *b"Find me"
//     };
// }

// const _: &str = {
//     #[link_section = "__DATA,manganis,regular,no_dead_strip"]
//     #[used]
//     static ASSET: [u8; 44usize] = *b"{\"Tailwind\":{\"classes\":\"flex flex-col p-2\"}}";
//     "flex flex-col p-2"
// };
