use crate::app::{App, AppResult};
use crossterm::event::KeyEvent;

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        crossterm::event::KeyCode::Char('q') => {
            app.running = false; // Quit the application when 'q' is pressed
        }
        crossterm::event::KeyCode::Up => {
            // Move up in the list of cities
            if app.selected_city > 0 {
                app.selected_city -= 1;
            }

        }
        crossterm::event::KeyCode::Down => {
            // Move down in the list of cities
            if app.selected_city < app.cities.len() - 1 {
                app.selected_city += 1;
            }
        }
        _ => {}
    }
    Ok(())
}

