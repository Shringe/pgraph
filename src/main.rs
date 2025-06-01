pub mod device;
pub mod textbox;
pub mod timespan;
pub mod wattage;

use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{
        Constraint::{self},
        Layout, Rect,
    },
    style::{Color, Stylize},
    widgets::{Block, Paragraph, Row, Table, Widget},
};

use device::Device;
use std::io::Result;
use textbox::Textbox;
use wattage::Wattage;

fn main() -> Result<()> {
    let mut terminal = ratatui::init();
    let mut app = App::new();
    let result = app.run(&mut terminal);
    ratatui::restore();
    result
}

enum Highlighted {
    ElectricityRate,
    InitialCost,
    Wattage,
}

impl Highlighted {
    pub fn up(&self) -> Self {
        match self {
            Highlighted::ElectricityRate => Self::Wattage,
            Highlighted::InitialCost => Self::ElectricityRate,
            Highlighted::Wattage => Self::InitialCost,
        }
    }

    pub fn down(&self) -> Self {
        match self {
            Highlighted::ElectricityRate => Self::InitialCost,
            Highlighted::InitialCost => Self::Wattage,
            Highlighted::Wattage => Self::ElectricityRate,
        }
    }
}

struct App {
    devices: Vec<Device>,
    exit: bool,

    electricity_rate: Textbox,
    initial_cost: Textbox,
    wattage: Textbox,
    highlighted: Highlighted,
}

impl App {
    pub fn new() -> Self {
        Self {
            devices: Vec::new(),
            exit: false,

            electricity_rate: Textbox::new(),
            initial_cost: Textbox::new(),
            wattage: Textbox::new(),
            highlighted: Highlighted::ElectricityRate,
        }
    }

    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key: event::KeyEvent) {
        let textbox = match self.highlighted {
            Highlighted::ElectricityRate => &mut self.electricity_rate,
            Highlighted::InitialCost => &mut self.initial_cost,
            Highlighted::Wattage => &mut self.wattage,
        };

        match key.code {
            KeyCode::Esc => self.exit(),
            KeyCode::Up => self.highlighted = self.highlighted.up(),
            KeyCode::Down => self.highlighted = self.highlighted.down(),
            KeyCode::Left => textbox.move_cursor_left(),
            KeyCode::Right => textbox.move_cursor_right(),
            KeyCode::Char(c) => textbox.enter_char(c),
            KeyCode::Backspace => textbox.delete_char(),
            KeyCode::Enter => self.add_device(),

            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    pub fn add_device(&mut self) {
        let initial_cost = match self.initial_cost.input.parse() {
            Ok(x) => x,
            Err(_) => todo!(),
        };

        let average_wattage = match self.wattage.input.parse() {
            Ok(x) => Wattage::new(x),
            Err(_) => todo!(),
        };

        let electricity_rate = match self.electricity_rate.input.parse() {
            Ok(x) => x,
            Err(_) => todo!(),
        };

        let d = Device {
            initial_cost,
            average_wattage,
            electricity_rate,
        };

        if !self.devices.contains(&d) {
            self.devices.push(d);
        }
    }

    fn render_equations(&self, area: Rect, buf: &mut Buffer) {
        let equation_layout = Layout::vertical([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Percentage(100),
        ]);

        let [rate_area, cost_area, wattage_area, devices_area] = equation_layout.areas(area);

        let highlighted_color = Color::Red;
        let unhighlighted_color = Color::Gray;

        let mut rate_color = unhighlighted_color;
        let mut cost_color = unhighlighted_color;
        let mut wattage_color = unhighlighted_color;

        match self.highlighted {
            Highlighted::ElectricityRate => rate_color = highlighted_color,
            Highlighted::InitialCost => cost_color = highlighted_color,
            Highlighted::Wattage => wattage_color = highlighted_color,
        }

        Paragraph::new(self.electricity_rate.input.clone())
            .block(Block::bordered().title("Electrity Rate in kWh/$"))
            .fg(rate_color)
            .render(rate_area, buf);

        Paragraph::new(self.initial_cost.input.clone())
            .block(Block::bordered().title("Upfront Cost of the Device"))
            .fg(cost_color)
            .render(cost_area, buf);

        Paragraph::new(self.wattage.input.clone())
            .block(Block::bordered().title("Average Wattage of the Device"))
            .fg(wattage_color)
            .render(wattage_area, buf);

        let mut rows = Vec::new();
        for d in &self.devices {
            rows.push(Row::new(vec![
                format!("{}", d.electricity_rate),
                format!("{}", d.initial_cost),
                format!("{}", d.average_wattage.watts),
            ]));
        }

        Table::new(
            rows,
            [
                Constraint::Ratio(1, 3),
                Constraint::Ratio(1, 3),
                Constraint::Ratio(1, 3),
            ],
        )
        .header(Row::new(vec![
            "Rate (kWh/$)",
            "Upfront ($)",
            "Average Wattage (W)",
        ]))
        .block(Block::bordered())
        .render(devices_area, buf);
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let main_layout =
            Layout::horizontal([Constraint::Percentage(30), Constraint::Percentage(70)]);

        let [equation_area, graph_area] = main_layout.areas(area);

        self.render_equations(equation_area, buf);
    }
}
