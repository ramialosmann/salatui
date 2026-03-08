use chrono::Local;
use ratatui::prelude::*;
use ratatui::widgets::Paragraph;

use crate::app::{App, View};
use crate::digits::{self, DIGIT_HEIGHT, DIGIT_WIDTH};

/// Main draw function dispatching to date line, main content, and countdown.
pub fn draw(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),  // date line
            Constraint::Fill(1),   // main content (clock or schedule)
            Constraint::Length(1), // countdown line
        ])
        .split(frame.area());

    draw_date_line(frame, chunks[0]);
    match app.view {
        View::Clock => draw_clock(frame, chunks[1], app),
        View::Schedule => draw_schedule(frame, chunks[1], app),
    }
    draw_countdown(frame, chunks[2], app);
}

/// Renders the Hijri + Gregorian date line centered at top.
fn draw_date_line(frame: &mut Frame, area: Rect) {
    let text = App::format_date_line();
    let paragraph = Paragraph::new(text).alignment(Alignment::Center);
    frame.render_widget(paragraph, area);
}

/// Renders large ASCII clock digits (HH:MM) centered in the area.
fn draw_clock(frame: &mut Frame, area: Rect, app: &App) {
    let now = Local::now();
    let time_str = match app.time_format.as_str() {
        "12h" => now.format("%I:%M").to_string(),
        _ => now.format("%H:%M").to_string(),
    };

    // Calculate total width: each char is DIGIT_WIDTH wide, with 1 col gap between chars
    let char_count = time_str.len() as u16;
    let gap = 1u16;
    let total_width = char_count * DIGIT_WIDTH + (char_count.saturating_sub(1)) * gap;
    let total_height = DIGIT_HEIGHT;

    // Check if area is too small
    if area.width < total_width || area.height < total_height {
        let msg = Paragraph::new("Terminal too small")
            .alignment(Alignment::Center);
        let v = Layout::vertical([
            Constraint::Fill(1),
            Constraint::Length(1),
            Constraint::Fill(1),
        ])
        .split(area);
        frame.render_widget(msg, v[1]);
        return;
    }

    // Center horizontally and vertically
    let x_offset = area.x + (area.width.saturating_sub(total_width)) / 2;
    let y_offset = area.y + (area.height.saturating_sub(total_height)) / 2;

    let buf = frame.buffer_mut();
    let mut cursor_x = x_offset;

    for ch in time_str.chars() {
        if let Some(idx) = digits::digit_index(ch) {
            let pattern = &digits::DIGITS[idx];
            for (row, cols) in pattern.iter().enumerate() {
                for (col, &filled) in cols.iter().enumerate() {
                    let px = cursor_x + col as u16;
                    let py = y_offset + row as u16;
                    if let Some(cell) = buf.cell_mut((px, py)) {
                        if filled {
                            cell.set_symbol("\u{2588}");
                        } else {
                            cell.set_symbol(" ");
                        }
                    }
                }
            }
            cursor_x += DIGIT_WIDTH + gap;
        }
    }
}

/// Renders the schedule view showing all 6 prayer times.
fn draw_schedule(frame: &mut Frame, area: Rect, app: &App) {
    let now = Local::now();
    let prayers = app.prayer_list();
    let next = app.next_prayer();

    let fmt = match app.time_format.as_str() {
        "12h" => "%I:%M %p",
        _ => "%H:%M",
    };

    let mut lines: Vec<Line> = Vec::new();
    lines.push(Line::from("Today's Prayer Times").alignment(Alignment::Center));
    lines.push(Line::from("──────────────────────").alignment(Alignment::Center));

    for (name, time) in &prayers {
        let time_str = time.format(fmt).to_string();
        let is_past = *time <= now;
        let is_next = next.as_ref().is_some_and(|(n, _)| *n == *name);

        let marker = if is_next { "\u{25B6} " } else { "  " };
        let style = if is_past {
            Style::default().fg(Color::DarkGray)
        } else {
            Style::default()
        };

        lines.push(Line::from(
            Span::styled(format!("{}{:<10} {}", marker, name, time_str), style),
        ));
    }

    let content_height = lines.len() as u16;
    let paragraph = Paragraph::new(lines).alignment(Alignment::Center);

    // Center vertically
    let vertical = Layout::vertical([
        Constraint::Fill(1),
        Constraint::Length(content_height),
        Constraint::Fill(1),
    ])
    .split(area);

    frame.render_widget(paragraph, vertical[1]);
}

/// Renders the countdown line centered at bottom.
fn draw_countdown(frame: &mut Frame, area: Rect, app: &App) {
    let text = app.format_countdown();
    let paragraph = Paragraph::new(text).alignment(Alignment::Center);
    frame.render_widget(paragraph, area);
}
