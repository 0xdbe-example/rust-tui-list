use std::cmp;

use tui::{
    widgets::{ListState, Paragraph, Block, BorderType, ListItem, Borders, List},
    backend::Backend, 
    Terminal, 
    layout::{Layout, Rect, Direction, Constraint, Alignment},
    style::{Color, Style, Modifier}
};

#[derive(Debug)]
/// Static, mutable application configuration
pub struct UserInterface {
    pub list_item: Vec<String>,
    list_state: ListState,
}

impl UserInterface {

  pub fn new(items: Vec<String>) -> UserInterface{
    let mut ui = UserInterface{
      list_item:items,
      list_state: ListState::default()
    };
    ui.list_state.select(Some(0));
    ui
  }

  // pub fn init(&mut self){
  //   self.list_state.select(Some(0));
  // }

  pub fn draw<B: Backend>(&mut self, terminal: &mut Terminal<B>){

        terminal.draw(|frame| {

          let main_layout: Layout = Layout::default();
          let chunks: Vec<Rect> = main_layout.direction(Direction::Vertical)
              .constraints([Constraint::Length(3),Constraint::Min(2),].as_ref())
              .split(frame.size());

          let title_widget = Paragraph::new("TUI Demo")
              .style(Style::default().fg(Color::LightCyan))
              .alignment(Alignment::Center)
              .block(
                  Block::default()
                      .borders(Borders::ALL)
                      .style(Style::default().fg(Color::White))
                      .border_type(BorderType::Plain),
              );

          let items: Vec<ListItem> = self.list_item.iter().map(|s| ListItem::new(s.as_ref())).collect();

          let list_block: Block = Block::default()
              .title("List")
              .borders(Borders::ALL)
              .border_type(BorderType::Plain)
              .style(Style::default().fg(Color::White));
          
          let list_widget = List::new(items)
              .block(list_block)
              .style(Style::default().fg(Color::White))
              .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
              .highlight_symbol(">>");

          frame.render_widget(title_widget, chunks[0]);
          frame.render_stateful_widget(list_widget, chunks[1], &mut self.list_state );

      }).unwrap_or_else(|e| {
        eprintln!("Fatal error writing to terminal: {e}");
        std::process::exit(1);
    });

  }

  pub fn list_down(&mut self){
    let curr = self.list_state.selected().unwrap();
    let next = cmp::min(curr + 1, self.list_item.len() - 1);
    self.list_state.select(Some(next));
  }

  pub fn list_up(&mut self){
    let curr = self.list_state.selected().unwrap();
    let next = if curr > 0 { curr - 1 } else { curr };
    self.list_state.select(Some(next));
  }

}