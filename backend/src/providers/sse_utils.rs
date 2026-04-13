pub fn pop_sse_frame_bytes(buffer: &mut Vec<u8>) -> Option<Vec<u8>> {
    let crlf = find_pattern(buffer, b"\r\n\r\n").map(|i| (i, 4));
    let lf = find_pattern(buffer, b"\n\n").map(|i| (i, 2));
    let (frame_end, delimiter_len) = match (crlf, lf) {
        (Some(a), Some(b)) => {
            if a.0 <= b.0 { a } else { b }
        }
        (Some(a), None) => a,
        (None, Some(b)) => b,
        (None, None) => return None,
    };
    let frame = buffer[..frame_end].to_vec();
    buffer.drain(..frame_end + delimiter_len);
    Some(frame)
}

fn find_pattern(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack.windows(needle.len()).position(|w| w == needle)
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
