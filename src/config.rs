#![allow(non_snake_case)] // Until this module can be constexpr.

#[inline] pub fn FFMPEG_BINARY() -> &'static str {
	option_env!("FFMPEG_BINARY").unwrap_or("ffmpeg")
}
#[inline] pub fn FFPROBE_BINARY() -> &'static str {
	option_env!("FFPROBE_BINARY").unwrap_or("ffprobe")
}
