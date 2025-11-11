use std::cmp::{max, min};

const DIGIT_SEGMENTS: [[bool; 7]; 10] = [
    [true, true, true, true, true, true, false],     // 0
    [false, true, true, false, false, false, false], // 1
    [true, true, false, true, true, false, true],    // 2
    [true, true, true, true, false, false, true],    // 3
    [false, true, true, false, false, true, true],   // 4
    [true, false, true, true, false, true, true],    // 5
    [true, false, true, true, true, true, true],     // 6
    [true, true, true, false, false, false, false],  // 7
    [true, true, true, true, true, true, true],      // 8
    [true, true, true, true, false, true, true],     // 9
];

pub fn render(time: &str, term_width: u16, term_height: u16) -> Vec<String> {
    let width = term_width as usize;
    let height = term_height as usize;
    if width == 0 || height == 0 {
        return Vec::new();
    }

    let mut canvas = vec![vec![' '; width]; height];

    if should_fallback(width, height) {
        render_fallback(&mut canvas, time);
        return canvas
            .into_iter()
            .map(|row| row.into_iter().collect())
            .collect();
    }

    let seg_thickness = max(1, min(height / 12, max(1, width / 96)));
    let digit_spacing = max(1, seg_thickness);
    let colon_width = max(1, seg_thickness);

    let elements = parse_elements(time);
    let digits_count = elements
        .iter()
        .filter(|el| matches!(el, Element::Digit(_)))
        .count();
    let colon_count = elements.len() - digits_count;

    let total_spacing = digit_spacing * (elements.len().saturating_sub(1));
    let total_colon_width = colon_count * colon_width;
    let available_for_digits = width.saturating_sub(total_spacing + total_colon_width);

    if digits_count == 0 {
        render_fallback(&mut canvas, time);
        return canvas
            .into_iter()
            .map(|row| row.into_iter().collect())
            .collect();
    }

    let digit_width = available_for_digits / digits_count;
    let min_digit_width = seg_thickness * 2 + max(1, seg_thickness);
    if digit_width < min_digit_width {
        render_fallback(&mut canvas, time);
        return canvas
            .into_iter()
            .map(|row| row.into_iter().collect())
            .collect();
    }

    let available_height = height.saturating_sub(seg_thickness * 2);
    let min_digit_height = seg_thickness * 3 + 2;
    if available_height < min_digit_height {
        render_fallback(&mut canvas, time);
        return canvas
            .into_iter()
            .map(|row| row.into_iter().collect())
            .collect();
    }

    let vertical_segment_height = max(1, (available_height - seg_thickness * 3) / 2);
    let digit_height = seg_thickness * 3 + vertical_segment_height * 2;
    let top_offset = (height.saturating_sub(digit_height)) / 2;

    let total_clock_width = digits_count * digit_width + colon_count * colon_width + total_spacing;
    let left_offset = (width.saturating_sub(total_clock_width)) / 2;

    let mut cursor_x = left_offset;
    for element in elements {
        match element {
            Element::Digit(value) => {
                draw_digit(
                    &mut canvas,
                    cursor_x,
                    top_offset,
                    digit_width,
                    digit_height,
                    seg_thickness,
                    vertical_segment_height,
                    value,
                );
                cursor_x += digit_width;
            }
            Element::Colon => {
                draw_colon(
                    &mut canvas,
                    cursor_x,
                    top_offset,
                    colon_width,
                    digit_height,
                    seg_thickness,
                    vertical_segment_height,
                );
                cursor_x += colon_width;
            }
        }
        cursor_x += digit_spacing;
    }

    canvas
        .into_iter()
        .map(|row| row.into_iter().collect())
        .collect()
}

fn should_fallback(width: usize, height: usize) -> bool {
    width < 48 || height < 10
}

fn render_fallback(canvas: &mut [Vec<char>], time: &str) {
    if canvas.is_empty() {
        return;
    }
    let height = canvas.len();
    let width = canvas[0].len();
    let text = format!("Retro Clock: {time}");
    let start_col = width.saturating_sub(text.len()) / 2;
    let start_row = height / 2;
    for (i, ch) in text.chars().enumerate() {
        if start_col + i < width {
            canvas[start_row][start_col + i] = ch;
        }
    }
}

#[derive(Clone, Copy)]
enum Element {
    Digit(u8),
    Colon,
}

fn parse_elements(time: &str) -> Vec<Element> {
    time.chars()
        .filter_map(|ch| match ch {
            '0'..='9' => Some(Element::Digit(ch.to_digit(10).unwrap() as u8)),
            ':' => Some(Element::Colon),
            _ => None,
        })
        .collect()
}

fn draw_digit(
    canvas: &mut [Vec<char>],
    left: usize,
    top: usize,
    width: usize,
    height: usize,
    seg_thickness: usize,
    vertical_segment_height: usize,
    value: u8,
) {
    if value > 9 {
        return;
    }
    let segments = DIGIT_SEGMENTS[value as usize];

    let inner_width = width.saturating_sub(seg_thickness * 2);
    let right = left + width;
    let middle_y = top + seg_thickness + vertical_segment_height;
    let bottom_y = top + seg_thickness * 2 + vertical_segment_height * 2;

    if segments[0] {
        draw_horizontal(canvas, left, top, width, seg_thickness);
    }
    if segments[1] {
        draw_vertical(
            canvas,
            left + seg_thickness + inner_width,
            top + seg_thickness,
            vertical_segment_height,
            seg_thickness,
        );
    }
    if segments[2] {
        draw_vertical(
            canvas,
            left + seg_thickness + inner_width,
            middle_y + seg_thickness,
            vertical_segment_height,
            seg_thickness,
        );
    }
    if segments[3] {
        draw_horizontal(canvas, left, bottom_y, width, seg_thickness);
    }
    if segments[4] {
        draw_vertical(
            canvas,
            left,
            middle_y + seg_thickness,
            vertical_segment_height,
            seg_thickness,
        );
    }
    if segments[5] {
        draw_vertical(
            canvas,
            left,
            top + seg_thickness,
            vertical_segment_height,
            seg_thickness,
        );
    }
    if segments[6] {
        draw_horizontal(canvas, left, middle_y, width, seg_thickness);
    }

    draw_corners(canvas, left, top, width, height, seg_thickness);
}

fn draw_corners(
    canvas: &mut [Vec<char>],
    left: usize,
    top: usize,
    width: usize,
    height: usize,
    seg_thickness: usize,
) {
    let right = left + width;
    let bottom = top + height;
    for dy in 0..seg_thickness {
        mark(canvas, left, top + dy, '+');
        mark(canvas, right.saturating_sub(1), top + dy, '+');
        if bottom > dy {
            mark(
                canvas,
                left,
                bottom.saturating_sub(1).saturating_sub(dy),
                '+',
            );
            mark(
                canvas,
                right.saturating_sub(1),
                bottom.saturating_sub(1).saturating_sub(dy),
                '+',
            );
        }
    }
}

fn draw_colon(
    canvas: &mut [Vec<char>],
    left: usize,
    top: usize,
    width: usize,
    digit_height: usize,
    seg_thickness: usize,
    vertical_segment_height: usize,
) {
    let center_x = left + width / 2;
    let top_dot_y = top + seg_thickness + vertical_segment_height / 2;
    let bottom_dot_y = top_dot_y + vertical_segment_height + seg_thickness;
    fill_block(canvas, center_x, top_dot_y, width, seg_thickness, 'o');
    fill_block(canvas, center_x, bottom_dot_y, width, seg_thickness, 'o');
    let bottom = top + digit_height;
    if bottom <= canvas.len() {
        // Draw a vertical divider hint to keep colon centered.
        for y in top..bottom {
            if (y + 1) % seg_thickness == 0 {
                mark(canvas, center_x, y, ':');
            }
        }
    }
}

fn draw_horizontal(
    canvas: &mut [Vec<char>],
    left: usize,
    top: usize,
    width: usize,
    thickness: usize,
) {
    for dy in 0..thickness {
        let y = top + dy;
        for x in left..left + width {
            mark(canvas, x, y, '=');
        }
    }
}

fn draw_vertical(canvas: &mut [Vec<char>], x: usize, top: usize, height: usize, thickness: usize) {
    for dx in 0..thickness {
        let col = x + dx;
        for y in top..top + height {
            mark(canvas, col, y, '|');
        }
    }
}

fn fill_block(
    canvas: &mut [Vec<char>],
    center_x: usize,
    center_y: usize,
    width: usize,
    thickness: usize,
    ch: char,
) {
    let half_w = width / 2;
    let start_x = center_x.saturating_sub(half_w);
    let end_x = min(center_x + half_w + 1, canvas[0].len());
    for y in center_y..min(center_y + thickness, canvas.len()) {
        for x in start_x..end_x {
            mark(canvas, x, y, ch);
        }
    }
}

fn mark(canvas: &mut [Vec<char>], x: usize, y: usize, ch: char) {
    if y < canvas.len() && x < canvas[y].len() {
        canvas[y][x] = ch;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fallback_when_too_small() {
        let frame = render("12:34:56", 30, 8);
        assert!(frame.iter().any(|line| line.contains("Retro Clock")));
    }

    #[test]
    fn renders_segments_for_large_size() {
        let frame = render("12:34:56", 120, 30);
        let has_vertical = frame.iter().any(|line| line.contains('|'));
        let has_horizontal = frame.iter().any(|line| line.contains('='));
        assert!(has_vertical && has_horizontal);
    }
}
