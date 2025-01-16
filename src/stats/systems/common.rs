pub fn format_duration_adaptive(seconds: f32) -> String {
    let total_millis = (seconds * 1000.0) as u64;
    let millis = total_millis % 1000;
    let total_seconds = total_millis / 1000;
    let seconds = total_seconds % 60;
    let total_minutes = total_seconds / 60;
    let minutes = total_minutes % 60;
    let total_hours = total_minutes / 60;
    let hours = total_hours % 24;
    let days = total_hours / 24;

    let mut result = String::new();

    if days > 0 {
        result.push_str(&format!("{}d ", days));
    }
    if hours > 0 || days > 0 {
        result.push_str(&format!("{:02}:", hours));
    }
    if minutes > 0 || hours > 0 || days > 0 {
        result.push_str(&format!("{:02}:", minutes));
    }

    // Always show at least seconds and milliseconds
    result.push_str(&format!("{:02}.{:03}", seconds, millis));

    result
}
