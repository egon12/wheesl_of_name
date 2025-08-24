use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Style, Stylize},
    symbols::Marker,
    text::{Line, Span},
    widgets::{
        canvas::{Canvas, Circle, Line as CanvasLine, Points},
        Block, BorderType, Paragraph, Widget,
    },
};
use std::f64::consts::PI;

use crate::app::App;

impl Widget for &App {
    /// Renders the user interface widgets.
    fn render(self, area: Rect, buf: &mut Buffer) {
        let chunks = Layout::vertical([
            Constraint::Length(4), // Instructions area
            Constraint::Min(0),    // Canvas area
        ])
        .split(area);

        // Render instructions
        self.render_instructions(chunks[0], buf);
        
        // Render wheel canvas
        self.render_wheel_canvas(chunks[1], buf);
    }
}

impl App {
    fn render_instructions(&self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered()
            .title("🎡 Wheel of Names 🎡")
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Rounded);

        let mut lines = Vec::new();
        
        // Instructions
        lines.push(Line::from(vec![
            Span::styled("Controls: ", Style::default().fg(Color::Yellow).bold()),
            Span::styled("[SPACE] Toggle Spin | [ENTER] Stop & Select | [Q] Quit", Style::default().fg(Color::White)),
        ]));

        // Wheel status
        let status = if self.is_spinning {
            Span::styled("🌀 SPINNING... 🌀", Style::default().fg(Color::Green).bold())
        } else if let Some(selected) = self.selected_option {
            Span::styled(
                format!("🎯 Selected: {}", self.wheel_options[selected]),
                Style::default().fg(Color::Cyan).bold()
            )
        } else {
            Span::styled("Press SPACE to spin!", Style::default().fg(Color::Gray))
        };
        lines.push(Line::from(status));

        let paragraph = Paragraph::new(lines)
            .block(block)
            .alignment(Alignment::Center);

        paragraph.render(area, buf);
    }

    fn render_wheel_canvas(&self, area: Rect, buf: &mut Buffer) {
        let chunks = Layout::horizontal([
            Constraint::Percentage(70), // Wheel area
            Constraint::Percentage(30), // Legend area
        ])
        .split(area);

        // Render wheel
        let wheel_canvas = Canvas::default()
            .block(Block::bordered().title("Wheel").title_alignment(Alignment::Center))
            .marker(Marker::Braille)
            .x_bounds([-30.0, 30.0])
            .y_bounds([-20.0, 20.0])
            .paint(|ctx| {
                let option_count = self.wheel_options.len();
                let angle_per_option = 2.0 * PI / option_count as f64;
                let wheel_radius = 18.0;
                
                // Draw colored sections
                for (i, &color) in self.wheel_colors.iter().enumerate() {
                    let base_angle = i as f64 * angle_per_option;
                    let current_angle = base_angle + (self.rotation_angle * PI / 180.0);
                    
                    // Draw filled section using points
                    let mut points = Vec::new();
                    let steps = 50; // Number of points to create smooth arc
                    
                    for step in 0..=steps {
                        let angle_offset = (step as f64 / steps as f64) * angle_per_option;
                        let angle = current_angle + angle_offset;
                        
                        // Create points from center to edge
                        for radius_step in 0..=20 {
                            let radius = (radius_step as f64 / 20.0) * wheel_radius;
                            let x = radius * angle.cos();
                            let y = radius * angle.sin();
                            points.push((x, y));
                        }
                    }
                    
                    // Highlight selected section
                    let section_color = if let Some(selected) = self.selected_option {
                        if selected == i { Color::White } else { color }
                    } else {
                        color
                    };
                    
                    ctx.draw(&Points {
                        coords: &points,
                        color: section_color,
                    });
                }
                
                // Draw section divider lines
                for i in 0..option_count {
                    let base_angle = i as f64 * angle_per_option;
                    let current_angle = base_angle + (self.rotation_angle * PI / 180.0);
                    
                    let line_end_x = wheel_radius * current_angle.cos();
                    let line_end_y = wheel_radius * current_angle.sin();
                    
                    ctx.draw(&CanvasLine {
                        x1: 0.0,
                        y1: 0.0,
                        x2: line_end_x,
                        y2: line_end_y,
                        color: Color::Black,
                    });
                }
                
                // Draw outer circle border
                ctx.draw(&Circle {
                    x: 0.0,
                    y: 0.0,
                    radius: wheel_radius,
                    color: Color::White,
                });
                
                // Draw center dot
                ctx.draw(&Circle {
                    x: 0.0,
                    y: 0.0,
                    radius: 1.0,
                    color: Color::Black,
                });
                
                // Draw pointer at top
                ctx.draw(&CanvasLine {
                    x1: 0.0,
                    y1: wheel_radius + 3.0,
                    x2: -2.0,
                    y2: wheel_radius + 1.0,
                    color: Color::Red,
                });
                ctx.draw(&CanvasLine {
                    x1: 0.0,
                    y1: wheel_radius + 3.0,
                    x2: 2.0,
                    y2: wheel_radius + 1.0,
                    color: Color::Red,
                });
            });

        wheel_canvas.render(chunks[0], buf);
        
        // Render legend
        self.render_legend(chunks[1], buf);
    }
    
    fn render_legend(&self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered()
            .title("Legend")
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Rounded);

        let mut lines = Vec::new();
        
        for (i, (option, &color)) in self.wheel_options.iter().zip(self.wheel_colors.iter()).enumerate() {
            let style = if let Some(selected) = self.selected_option {
                if selected == i {
                    Style::default().fg(color).bold().bg(Color::DarkGray)
                } else {
                    Style::default().fg(color)
                }
            } else {
                Style::default().fg(color)
            };
            
            lines.push(Line::from(vec![
                Span::styled("■ ", style),
                Span::styled(option.clone(), style),
            ]));
        }

        let paragraph = Paragraph::new(lines)
            .block(block)
            .alignment(Alignment::Left);

        paragraph.render(area, buf);
    }
}
