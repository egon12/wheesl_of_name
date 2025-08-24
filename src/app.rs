use crate::event::{AppEvent, Event, EventHandler};
use ratatui::{
    DefaultTerminal,
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
    style::Color,
};
use std::time::Instant;

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// Counter.
    pub counter: u8,
    /// Event handler.
    pub events: EventHandler,
    /// Wheel options
    pub wheel_options: Vec<String>,
    /// Wheel colors
    pub wheel_colors: Vec<Color>,
    /// Current rotation angle (0.0 to 360.0)
    pub rotation_angle: f64,
    /// Is the wheel spinning?
    pub is_spinning: bool,
    /// Spin speed (degrees per tick)
    pub spin_speed: f64,
    /// Last tick time for smooth animation
    pub last_tick: Instant,
    /// Selected option index
    pub selected_option: Option<usize>,
}

impl Default for App {
    fn default() -> Self {
        let wheel_options = vec![
            "Option 1".to_string(),
            "Option 2".to_string(),
            "Option 3".to_string(),
            "Option 4".to_string(),
            "Option 5".to_string(),
            "Option 6".to_string(),
        ];
        
        let wheel_colors = Self::generate_colors(wheel_options.len());
        
        Self {
            running: true,
            counter: 0,
            events: EventHandler::new(),
            wheel_options,
            wheel_colors,
            rotation_angle: 0.0,
            is_spinning: false,
            spin_speed: 5.0,
            last_tick: Instant::now(),
            selected_option: None,
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Run the application's main loop.
    pub fn run(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        while self.running {
            terminal.draw(|frame| frame.render_widget(&self, frame.area()))?;
            self.handle_events()?;
        }
        Ok(())
    }

    pub fn handle_events(&mut self) -> color_eyre::Result<()> {
        match self.events.next()? {
            Event::Tick => self.tick(),
            Event::Crossterm(event) => match event {
                crossterm::event::Event::Key(key_event) => self.handle_key_event(key_event)?,
                _ => {}
            },
            Event::App(app_event) => match app_event {
                AppEvent::Increment => self.increment_counter(),
                AppEvent::Decrement => self.decrement_counter(),
                AppEvent::Quit => self.quit(),
                AppEvent::ToggleSpin => self.toggle_spin(),
                AppEvent::StopAndSelect => self.stop_and_select(),
            },
        }
        Ok(())
    }

    /// Handles the key events and updates the state of [`App`].
    pub fn handle_key_event(&mut self, key_event: KeyEvent) -> color_eyre::Result<()> {
        match key_event.code {
            KeyCode::Esc | KeyCode::Char('q') => self.events.send(AppEvent::Quit),
            KeyCode::Char('c' | 'C') if key_event.modifiers == KeyModifiers::CONTROL => {
                self.events.send(AppEvent::Quit)
            }
            KeyCode::Right => self.events.send(AppEvent::Increment),
            KeyCode::Left => self.events.send(AppEvent::Decrement),
            KeyCode::Char(' ') => self.events.send(AppEvent::ToggleSpin),
            KeyCode::Enter => self.events.send(AppEvent::StopAndSelect),
            // Other handlers you could add here.
            _ => {}
        }
        Ok(())
    }

    /// Handles the tick event of the terminal.
    ///
    /// The tick event is where you can update the state of your application with any logic that
    /// needs to be updated at a fixed frame rate. E.g. polling a server, updating an animation.
    pub fn tick(&mut self) {
        let now = Instant::now();
        let delta = now.duration_since(self.last_tick).as_secs_f64();
        self.last_tick = now;
        
        if self.is_spinning {
            self.rotation_angle += self.spin_speed * delta * 60.0; // 60 FPS normalization
            if self.rotation_angle >= 360.0 {
                self.rotation_angle -= 360.0;
            }
        }
    }

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn increment_counter(&mut self) {
        self.counter = self.counter.saturating_add(1);
    }

    pub fn decrement_counter(&mut self) {
        self.counter = self.counter.saturating_sub(1);
    }

    pub fn toggle_spin(&mut self) {
        self.is_spinning = !self.is_spinning;
        if self.is_spinning {
            self.selected_option = None;
            self.spin_speed = 180.0; // Start with fast spin
        }
    }

    pub fn stop_and_select(&mut self) {
        if self.is_spinning {
            self.is_spinning = false;
            // Calculate which option is selected based on rotation angle
            let option_count = self.wheel_options.len() as f64;
            let angle_per_option = 360.0 / option_count;
            let selected_index = ((360.0 - self.rotation_angle) / angle_per_option) as usize % self.wheel_options.len();
            self.selected_option = Some(selected_index);
        }
    }
    
    fn generate_colors(count: usize) -> Vec<Color> {
        let base_colors = vec![
            Color::Red,
            Color::Blue,
            Color::Green,
            Color::Yellow,
            Color::Magenta,
            Color::Cyan,
            Color::LightRed,
            Color::LightBlue,
            Color::LightGreen,
            Color::LightYellow,
            Color::LightMagenta,
            Color::LightCyan,
        ];
        
        let mut colors = Vec::new();
        for i in 0..count {
            colors.push(base_colors[i % base_colors.len()]);
        }
        colors
    }
}
