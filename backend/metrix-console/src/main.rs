use std::io;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use std::time::Instant;
use tui::backend::CrosstermBackend;
use tui::style::{Color, Style, Modifier};
use tui::symbols::{Marker};
use tui::layout::{Layout, Constraint, Direction};
use tui::widgets::{Block, Borders, Axis, Chart, Dataset};
use tui::Terminal;
use tui::text::Span;

enum Event<I> {
    Input(I),
    Tick,
}

fn main() -> Result<(), io::Error> {
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let (tx, _rx) = mpsc::channel();
    let tick_rate = Duration::from_millis(200);
    terminal.clear()?;
    thread::spawn(move || {
        let mut last_tick = Instant::now();
        loop {
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if crossterm::event::poll(timeout).expect("poll works") {
                if let crossterm::event::Event::Key(key) = crossterm::event::read().expect("can read events") {
                    tx.send(Event::Input(key)).expect("can send events");
                }
            }

            if last_tick.elapsed() >= tick_rate {
                if let Ok(_) = tx.send(Event::Tick) {
                    last_tick = Instant::now();
                }
            }
        }
    });
    let mut data = vec![(0., 0.), (1., 1.), (2., 2.)];
    loop {
        terminal.draw(|f| {
            data.push((1., 1.));
            let size = f.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Ratio(1, 3),
                    ]
                    .as_ref(),
                )
                .split(size);
            let x_labels = vec![
                Span::styled(
                    format!("{}", "Set 1"),
                    Style::default().add_modifier(Modifier::BOLD),
                ),
            ];
            let datasets = vec![
                Dataset::default()
                    .name("data1")
                    .marker(Marker::Dot)
                    .style(Style::default().fg(Color::Cyan))
                    .data(&data)
            ];

            let chart = Chart::new(datasets)
                .block(
                    Block::default()
                        .title(Span::styled(
                            "Chart 1",
                            Style::default()
                                .fg(Color::Cyan)
                                .add_modifier(Modifier::BOLD),
                        ))
                        .borders(Borders::ALL),
                )
                .x_axis(
                    Axis::default()
                        .title("X Axis")
                        .style(Style::default().fg(Color::Gray))
                        .labels(x_labels)
                        .bounds([-20.0, 20.0]),
                )
                .y_axis(
                    Axis::default()
                        .title("Y Axis")
                        .style(Style::default().fg(Color::Gray))
                        .labels(vec![
                            Span::styled("-20", Style::default().add_modifier(Modifier::BOLD)),
                            Span::raw("0"),
                            Span::styled("20", Style::default().add_modifier(Modifier::BOLD)),
                        ])
                        .bounds([-20.0, 20.0]),
                );
            f.render_widget(chart, chunks[0]);
        })?;
    }
    Ok(())
}
