pub fn format_with_units(mut qtty: u64) -> String {
    if qtty < 1024 {
        return qtty.to_string();
    }
    qtty /= 1024;

    if qtty < 1024 {
        return format!("{} KB", qtty);
    }
    qtty /= 1024;

    if qtty < 1024 {
        return format!("{} MB", qtty);
    }
    qtty /= 1024;

    format!("{} GB", qtty)
}
