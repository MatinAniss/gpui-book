use std::time::Duration;

use gpui::Animation;

fn main() {
    Animation::new(Duration::from_secs(1)).repeat();
}
