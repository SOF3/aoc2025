use std::time::{Duration, Instant};
use std::{env, fs};

use anyhow::Context;
use crossterm::event::KeyModifiers;
use ratatui::Frame;
use ratatui::layout::{self, Layout};
use ratatui::style::{Color, Style, Stylize};
use ratatui::symbols::Marker;
use ratatui::text::{self, Span};
use ratatui::widgets::canvas::{self, Canvas};
use ratatui::widgets::{Axis, Chart, Dataset, GraphType, Paragraph};

const RENDER_FREQ: Duration = Duration::from_millis(100);

struct RestoreGuard;
impl Drop for RestoreGuard {
    fn drop(&mut self) { ratatui::restore(); }
}

fn main() -> anyhow::Result<()> {
    let mut terminal = ratatui::init();
    let _restore_guard = RestoreGuard;

    let input = fs::read_to_string(
        env::args().nth(1).context("Usage: cargo run -p render-day4 -- inputs/day4.txt")?,
    )?;

    let mut running = true;

    let row_length = input.find('\n').context("no newline")?;
    let mut states = vec![State::new(); row_length * row_length];

    for x in 0..row_length {
        for y in 0..row_length {
            let state = &mut states[x + y * row_length];
            match input.as_bytes()[x + y * (row_length + 1)] {
                b'.' => {}
                b'@' => {
                    state.present = true;
                    state.next_present = true;
                }
                _ => unreachable!(),
            }
        }
    }

    let mut draw_state = DrawState::default();
    while running {
        terminal.draw(|f| draw(f, row_length, &states, &mut draw_state))?;
        let next_tick = Instant::now() + RENDER_FREQ;
        while let Some(remain) = next_tick.checked_duration_since(Instant::now()) {
            if crossterm::event::poll(remain)? {
                let event = crossterm::event::read()?;
                if let crossterm::event::Event::Key(key_event) = event {
                    match key_event.code {
                        crossterm::event::KeyCode::Char('q') => running = false,
                        crossterm::event::KeyCode::Char('c')
                            if key_event.modifiers.contains(KeyModifiers::CONTROL) =>
                        {
                            running = false
                        }
                        crossterm::event::KeyCode::Char(' ') => {
                            draw_state.paused = !draw_state.paused
                        }
                        _ => {}
                    }
                }
            }
        }
        if !draw_state.paused {
            simulate_once(row_length, &mut states);
        }
    }

    Ok(())
}

fn simulate_once(row_length: usize, states: &mut [State]) {
    for xy in 0..states.len() {
        if !states[xy].present {
            states[xy].age /= 2;
            continue;
        }

        let x = xy % row_length;
        let y = xy / row_length;

        let mut presence = 0;
        for idx in adjacencies(x, y, row_length) {
            if states[idx].present {}
            presence += states[idx].present as u32;
        }
        if presence < 4 {
            states[xy].next_present = false;
            states[xy].age = 128;
        }
    }

    for state in states {
        state.present = state.next_present;
    }
}

fn adjacencies(x: usize, y: usize, row_length: usize) -> impl Iterator<Item = usize> {
    [
        (x.checked_sub(1), y.checked_sub(1)),
        (x.checked_sub(1), Some(y)),
        (x.checked_sub(1), Some(y + 1).filter(|&y| y < row_length)),
        (Some(x), y.checked_sub(1)),
        (Some(x), Some(y + 1).filter(|&y| y < row_length)),
        (Some(x + 1).filter(|&x| x < row_length), y.checked_sub(1)),
        (Some(x + 1).filter(|&x| x < row_length), Some(y)),
        (Some(x + 1).filter(|&x| x < row_length), Some(y + 1).filter(|&y| y < row_length)),
    ]
    .into_iter()
    .filter_map(|(x_opt, y_opt)| Some((x_opt?, y_opt?)))
    .map(move |(adj_x, adj_y)| adj_x + adj_y * row_length)
}

#[derive(Clone)]
struct State {
    present:      bool,
    next_present: bool,
    age:          u8,
}

impl State {
    const fn new() -> Self { Self { present: false, next_present: false, age: 0 } }
}

#[derive(Default)]
struct DrawState {
    paused:       bool,
    prev_present: Option<usize>,
    spins:        usize,
    delta_log:    Vec<usize>,
}

fn draw(f: &mut Frame, row_length: usize, states: &[State], draw_state: &mut DrawState) {
    let [upper_area, text_area] = (*Layout::new(
        layout::Direction::Vertical,
        [layout::Constraint::Fill(1), layout::Constraint::Max(1)],
    )
    .split(f.area()))
    .try_into()
    .unwrap();

    let [mut main_area, chart_area] = (*Layout::new(
        layout::Direction::Horizontal,
        [layout::Constraint::Ratio(2, 3), layout::Constraint::Ratio(1, 3)],
    )
    .split(upper_area))
    .try_into()
    .unwrap();

    // want height * 2 == width
    if main_area.height * 2 > main_area.width {
        let height_delta = main_area.width - main_area.width / 2;
        main_area.y += height_delta / 2;
        main_area.height = main_area.width / 2;
    } else {
        let width_delta = main_area.width - main_area.height * 2;
        main_area.x += width_delta / 2;
        main_area.width = main_area.height * 2;
    }

    f.render_widget(
        Canvas::default()
            .x_bounds([0.0, row_length as f64])
            .y_bounds([0.0, row_length as f64])
            .paint(|ctx| {
                for (xy, state) in states.iter().enumerate() {
                    const DENSITY: u32 = 8;

                    let x = xy % row_length;
                    let y = row_length - 1 - xy / row_length;
                    let color = if state.present {
                        Color::Rgb(255, 255, 128)
                    } else {
                        Color::Rgb(state.age + 127, 0, 127 + state.age)
                    };
                    for i in 0..DENSITY {
                        let d = i as f64 / DENSITY as f64;
                        ctx.draw(&canvas::Rectangle {
                            x: x as f64 + d,
                            y: y as f64 + d,
                            width: 1.0 - d * 2.0,
                            height: 1.0 - d * 2.0,
                            color,
                        });
                    }
                }
            }),
        main_area,
    );

    let present = states.iter().filter(|state| state.present).count();
    let delta = draw_state.prev_present.unwrap_or(present) - present;

    let texts = [
        (Color::Gray, "Turn", draw_state.spins, false),
        (Color::LightRed, "Remaining", present, true),
        (Color::Magenta, "Delta", delta, true),
        (Color::LightGreen, "Clear", row_length * row_length - present, true),
    ];

    for (&(color, title, count, ratio), &area) in texts.iter().zip(
        Layout::new(
            layout::Direction::Horizontal,
            texts.map(|_| layout::Constraint::Ratio(1, texts.len() as u32)),
        )
        .split(text_area)
        .iter(),
    ) {
        f.render_widget(
            Paragraph::new(text::Line::from_iter(
                [
                    Some(Span::styled(format!("{title}: "), Style::new().fg(Color::White).bold())),
                    Some(Span::styled(count.to_string(), Style::new().fg(color))),
                    ratio.then(|| {
                        Span::styled(
                            format!(" ({:.1}%)", count as f64 / (row_length * row_length) as f64),
                            Style::new().fg(Color::Cyan),
                        )
                    }),
                ]
                .into_iter()
                .flatten(),
            )),
            area,
        );
    }

    if !draw_state.paused {
        draw_state.prev_present = Some(present);
        draw_state.delta_log.push(delta);
        draw_state.spins += 1;
    }

    let delta_chart_data: Vec<(f64, f64)> =
        draw_state.delta_log.iter().enumerate().map(|(x, &y)| (x as f64, y as f64)).collect();
    let max_delta = draw_state.delta_log.iter().copied().max().unwrap_or_default() as f64;
    f.render_widget(
        Chart::new(vec![
            Dataset::default()
                .name("delta")
                .data(&delta_chart_data)
                .marker(Marker::Braille)
                .graph_type(GraphType::Line)
                .style(Style::default().white()),
        ])
        .x_axis(Axis::default().bounds([0.0, delta_chart_data.len() as f64]))
        .y_axis(
            Axis::default()
                .bounds([0.0, max_delta])
                .labels([0.0, max_delta / 2.0, max_delta].map(|y| y.to_string())),
        ),
        chart_area,
    );
}
