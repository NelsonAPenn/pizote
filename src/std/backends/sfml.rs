extern crate sfml;

use std::thread;
use std::sync::mpsc::{channel, Sender, TryRecvError};
use crate::action::draw::{Draw, DrawAction};
use crate::bounds::Bounds;
use std::time::Duration;

use sfml::{
    graphics::{
        CircleShape, Color, RenderWindow, RenderTarget, Transformable, Shape, Image, Texture, Sprite, IntRect, VertexArray, PrimitiveType
    },
    system::{Clock, Time, sleep, Vector2f},
    window::{ContextSettings, Event, Style}
};


pub struct Sfml
{
}

// window mutex? May be needed for getting other senders
impl Draw for Sfml
{
    fn spawn(&mut self, initial_bounds: Bounds) -> Sender<DrawAction>
    {
        let (tx, rx) = channel::<DrawAction>();
        thread::spawn( move || {
            // spin up resources

            let context_settings = ContextSettings {
                antialiasing_level: 0,
                ..Default::default()
            };
            let mut window = RenderWindow::new(
                (initial_bounds.width as u32, initial_bounds.height as u32), // TODO: change this
                "Application",
                Style::CLOSE,
                &context_settings,
            );
            // window.clear(Color::rgb(255, 255, 255));
            window.display();
            
            'render: loop
            {
                while let Some(event) = window.poll_event()
                {
                    match event
                    {
                        Event::Closed => 
                        {
                            window.close();
                            break 'render;
                        },
                        _ => {}
                    }
                }
                match rx.try_recv()
                {


                    Ok(action) => {
                        // draw the action
                        match action
                        {
                            DrawAction::Noop => {
                                // do nothing!
                            },
                            DrawAction::Line( (x_0, y_0), (x_1, y_1) ) => {
                                let mut line = VertexArray::new(PrimitiveType::Lines, 2);
                                line[0].position = Vector2f::new(x_0 as f32, y_0 as f32);
                                line[1].position = Vector2f::new(x_1 as f32, y_1 as f32);
                                window.draw(&line);
                            },
                            DrawAction::Clear => {
                                window.clear(Color::rgb(0, 0, 0));
                            },
                            _ => {
                                panic!("Not implemented.");
                            }
                        }

                        window.display();
                    }
                    Err(TryRecvError::Disconnected) => {
                        break 'render;

                    },
                    Err(TryRecvError::Empty) => {
                        // do nothing
                        thread::sleep(Duration::from_millis(crate::constants::WAIT_ON_EMPTY_MS));
                    }
                }
            }
        });

        tx
    }

}