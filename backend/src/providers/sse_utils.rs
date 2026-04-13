pub fn pop_sse_frame(buffer: &mut String) -> Option<String> {
    let (frame_end, delimiter_len) = if let Some(index) = buffer.find("\r\n\r\n") {
        (index, 4)
    } else {
        (buffer.find("\n\n")?, 2)
    };
    let frame = buffer[..frame_end].to_string();
    buffer.drain(..frame_end + delimiter_len);
    Some(frame)
}

pub fn extract_sse_data(frame: &str) -> Option<String> {
    let mut lines = Vec::new();

    for line in frame.lines() {
        if let Some(rest) = line.strip_prefix("data:") {
            lines.push(rest.trim_start().to_string());
        }
    }

    if lines.is_empty() {
        None
    } else {
        Some(lines.join("\n"))
    }
}
