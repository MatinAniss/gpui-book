use std::time::Duration;

use gpui::{Animation, ease_in_out};

fn main() {
    Animation::new(Duration::from_secs(1)).with_easing(ease_in_out);
}
