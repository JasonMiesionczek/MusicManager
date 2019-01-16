use super::video_info;
use super::video_info::VideoInfo;

pub fn download(id: &str) -> Option<VideoInfo> {
    info!("fetching video info...");
    if let Ok(v) = video_info::get_video_info(id) {
        Some(v)
    } else {
        None
    }
}
