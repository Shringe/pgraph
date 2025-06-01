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
    style::{Color, Style, Styled, Stylize},
    symbols,
    widgets::{Axis, Block, Chart, Dataset, GraphType, Paragraph, Row, Table, Widget},
};

use device::Device;
use rand::Rng;
use std::io::Result;
use textbox::Textbox;
use timespan::Timespan;
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
    Name,
}

impl Highlighted {
    pub fn up(&self) -> Self {
        match self {
            Highlighted::ElectricityRate => Self::Name,
            Highlighted::InitialCost => Self::ElectricityRate,
            Highlighted::Wattage => Self::InitialCost,
            Highlighted::Name => Self::Wattage,
        }
    }

    pub fn down(&self) -> Self {
        match self {
            Highlighted::ElectricityRate => Self::InitialCost,
            Highlighted::InitialCost => Self::Wattage,
            Highlighted::Wattage => Self::Name,
            Highlighted::Name => Self::ElectricityRate,
        }
    }
}

struct App {
    devices: Vec<Device>,
    exit: bool,

    electricity_rate: Textbox,
    initial_cost: Textbox,
    wattage: Textbox,
    name: Textbox,
    highlighted: Highlighted,

    randomize_graph_colors: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            devices: Vec::new(),
            exit: false,

            electricity_rate: Textbox::new(),
            initial_cost: Textbox::new(),
            wattage: Textbox::new(),
            name: Textbox::new(),
            highlighted: Highlighted::ElectricityRate,

            randomize_graph_colors: true,
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
            Highlighted::Name => &mut self.name,
        };

        match key.code {
            KeyCode::Esc => self.exit(),
            KeyCode::Up => self.highlighted = self.highlighted.up(),
            KeyCode::Down => self.highlighted = self.highlighted.down(),
            KeyCode::Tab => self.highlighted = self.highlighted.down(),
            KeyCode::BackTab => self.highlighted = self.highlighted.up(),
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

        let name = self.name.input.clone();

        let color = if self.randomize_graph_colors {
            let mut rng = rand::rng();
            Color::Rgb(rng.random(), rng.random(), rng.random())
        } else {
            Color::Gray
        };

        let d = Device {
            initial_cost,
            average_wattage,
            electricity_rate,
            color,
            name,
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
            Constraint::Length(3),
            Constraint::Percentage(100),
        ]);

        let [rate_area, cost_area, wattage_area, name_area, devices_area] =
            equation_layout.areas(area);

        let highlighted_color = Color::Red;
        let unhighlighted_color = Color::Gray;

        let mut rate_color = unhighlighted_color;
        let mut cost_color = unhighlighted_color;
        let mut wattage_color = unhighlighted_color;
        let mut name_color = unhighlighted_color;

        match self.highlighted {
            Highlighted::ElectricityRate => rate_color = highlighted_color,
            Highlighted::InitialCost => cost_color = highlighted_color,
            Highlighted::Wattage => wattage_color = highlighted_color,
            Highlighted::Name => name_color = highlighted_color,
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

        Paragraph::new(self.name.input.clone())
            .block(Block::bordered().title("Optional Name"))
            .fg(name_color)
            .render(name_area, buf);

        let mut rows = Vec::new();
        for d in &self.devices {
            rows.push(Row::new(vec![
                format!("{}", d.name).set_style(Style::new().fg(d.color)),
                format!("{}", d.electricity_rate).set_style(Style::new().fg(d.color)),
                format!("{}", d.initial_cost).set_style(Style::new().fg(d.color)),
                format!("{}", d.average_wattage.watts).set_style(Style::new().fg(d.color)),
            ]));
        }

        Table::new(
            rows,
            [
                Constraint::Percentage(10),
                Constraint::Ratio(1, 3),
                Constraint::Ratio(1, 3),
                Constraint::Ratio(1, 3),
            ],
        )
        .header(Row::new(vec![
            "Name",
            "Rate (kWh/$)",
            "Upfront ($)",
            "Average Wattage (W)",
        ]))
        .block(Block::bordered())
        .render(devices_area, buf);
    }

    fn render_graph(&self, area: Rect, buf: &mut Buffer) {
        // Really gross memory leak
        let mut datasets = Vec::new();
        let mut max_cost = 0.0;
        for d in &self.devices {
            let cost = d.total_cost(&Timespan::from_months(36.0));
            let data_points: &'_ [(f64, f64)] =
                Box::leak(Box::new([(0.0, d.initial_cost), (36.0, cost)]));

            datasets.push(
                Dataset::default()
                    .marker(symbols::Marker::Braille)
                    .graph_type(GraphType::Line)
                    .fg(d.color)
                    .bg(Color::Black)
                    .data(data_points),
            );
            if cost > max_cost {
                max_cost = cost;
            }
        }

        let x_labels: Vec<String> = Self::create_spaced_labels(36.0, 8)
            .into_iter()
            .map(|l| format!("{:.0}", l))
            .collect();

        let y_labels: Vec<String> = Self::create_spaced_labels(max_cost, 8)
            .into_iter()
            .map(|l| format!("{:.0}", l))
            .collect();

        let x_axis = Axis::default()
            .title("Months".red())
            .bounds([0.0, 36.0])
            .labels(x_labels);

        let y_axis = Axis::default()
            .title("Cost".red())
            .bounds([0.0, max_cost])
            .labels(y_labels);

        Chart::new(datasets)
            .block(Block::bordered().title("Chart"))
            .x_axis(x_axis)
            .y_axis(y_axis)
            .render(area, buf);
    }

    fn create_spaced_labels(max: f64, steps: u8) -> Vec<f64> {
        let mut labels = Vec::new();
        let fraction = max / steps as f64;
        for i in 0..steps + 1 {
            labels.push((i) as f64 * fraction);
        }

        labels
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let main_layout =
            Layout::horizontal([Constraint::Percentage(30), Constraint::Percentage(70)]);

        let [equation_area, graph_area] = main_layout.areas(area);

        self.render_equations(equation_area, buf);
        self.render_graph(graph_area, buf);
    }
}
