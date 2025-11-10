pub fn format_time(total_ms: u128) -> String {
    let total_seconds = total_ms / 1000;
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;
    return format!("{:02}:{:02}:{:02}", hours, minutes, seconds);
}
