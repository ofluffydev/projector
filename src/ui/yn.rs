use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Paragraph};
use ratatui::{
    crossterm::event::{self, Event as CrosstermEvent, KeyCode, KeyEventKind},
    layout::{Constraint, Layout},
    DefaultTerminal, Frame,
};

pub fn ask(question: &str) -> color_eyre::Result<bool> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let mut answer = None;
    let result = run(terminal, question, &mut answer);
    ratatui::restore();
    result?;
    Ok(answer.unwrap_or(false))
}

fn run(
    mut terminal: DefaultTerminal,
    question: &str,
    answer: &mut Option<bool>,
) -> color_eyre::Result<()> {
    loop {
        terminal.draw(|frame| render(frame, question, answer))?;
        if let CrosstermEvent::Key(key_event) = event::read()? {
            if key_event.kind == KeyEventKind::Press {
                match key_event.code {
                    KeyCode::Esc => {
                        *answer = Some(false);
                        break Ok(());
                    }
                    KeyCode::Char('y') | KeyCode::Enter => {
                        *answer = Some(true);
                        break Ok(());
                    }
                    KeyCode::Char('n') => {
                        *answer = Some(false);
                        break Ok(());
                    }
                    _ => {}
                }
            }
        }
    }
}

fn render(frame: &mut Frame, question: &str, _answer: &Option<bool>) {
    let vertical = Layout::vertical([
        Constraint::Length(3), // Adjusted to make the first box smaller
        Constraint::Min(1),    // Ensures the second box fits
        Constraint::Length(3), // Adjusted to make the third box smaller
    ])
    .split(frame.area());

    let header = Paragraph::new(Line::from("Projector - Yes/No Prompt"))
        .block(Block::default())
        .style(Style::default().fg(Color::LightBlue))
        .centered();
    frame.render_widget(header, vertical[0]);

    let question_paragraph = Paragraph::new(Line::from(question))
        .block(Block::bordered().border_type(BorderType::Rounded));
    frame.render_widget(question_paragraph, vertical[1]);

    let footer = Paragraph::new(Line::from(Span::from(
        "Press 'y' for Yes, 'n' for No, or Enter to confirm Yes",
    )))
    .block(Block::bordered().border_type(BorderType::Rounded));
    frame.render_widget(footer, vertical[2]);
}
