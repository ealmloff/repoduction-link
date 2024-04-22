
pub static BENCHMARKS: ::linkme::DistributedSlice<[u8]> = {
    #[cfg(
        any(
            target_os = "none",
            target_os = "linux",
            target_os = "macos",
            target_os = "ios",
            target_os = "tvos",
            target_os = "android",
            target_os = "fuchsia",
            target_os = "illumos",
            target_os = "freebsd",
            target_os = "psp",
        )
    )]
    extern "Rust" {
        #[link_name = "\u{1}section$start$__DATA$manganis"]
        static LINKME_START: u8;
        #[link_name = "\u{1}section$end$__DATA$manganis"]
        static LINKME_STOP: u8;
    }
    unsafe {
        ::linkme::DistributedSlice::private_new(
            "BENCHMARKS",
            & LINKME_START,
            & LINKME_STOP,
            & 0,
            & 0,
        )
    }
};

pub mod x {
    pub const ROBOTO_FONT_LIGHT_FONT: &str = {
        #[link_section = "__DATA,manganis,regular,no_dead_strip"]
        #[used]
        static ASSET: [u8; 216usize] = *b"{\"File\":{\"location\":{\"unique_name\":\"css263687b6c2c072aa4.css\",\"source\":{\"Remote\":\"https://fonts.googleapis.com/css2?family=Roboto&weight=200&text=hello+world\"}},\"options\":{\"Css\":{\"minify\":true}},\"url_encoded\":false}}";
        "/css263687b6c2c072aa4.css"
    };
}