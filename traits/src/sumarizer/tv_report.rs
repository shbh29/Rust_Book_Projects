
use super::summary::Summary;

pub struct TvReport {
    pub news_anchor: String,
    pub tv_channel: String,
    pub topic: String,
}

impl Summary for TvReport {}