extern crate sfml;

use std::collections::HashMap;
use std::thread;
use std::sync::mpsc::{channel, Sender, TryRecvError};
use crate::action::draw::{Draw, DrawAction};
use crate::bounds::Bounds;
use std::time::Duration;

use sfml::{
    graphics::{
        CircleShape, Color, RenderWindow, RenderTarget, RenderTexture, Transformable, Shape, Image, Texture, Sprite, IntRect, VertexArray, PrimitiveType
    },
    system::{Clock, Time, sleep, Vector2f},
    window::{ContextSettings, Event, Style}
};

struct RenderNode
{
    pub texture: RenderTexture,
    pub children: HashMap<String, ( (f64, f64), RenderNode)>
}

impl RenderNode
{
    pub fn new(w: f64, h: f64) -> RenderNode
    {
        let texture = RenderTexture::new(w as u32, h as u32, false).unwrap();

        RenderNode
        {
            texture,
            children: HashMap::<String, ((f64, f64), RenderNode)>::new()
        }
    }

    pub fn add_child(&mut self, uuid: String, z: i8, offset: (f64, f64), w: f64, h: f64)
    {
        let new_node = RenderNode::new(w, h);
        self.children.insert(uuid, (offset, new_node));
    }

    pub fn draw_children(&mut self)
    {
        for (_uuid, (_offset, child)) in self.children.iter_mut()
        {
            child.draw_children();
            let mut sprite = Sprite::new();
            sprite.set_texture(child.texture.texture(), true);
            self.texture.draw(&sprite);
        }
    }
}


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
            let mut root = RenderNode::new(initial_bounds.width, initial_bounds.height);
            {
                let mut sprite = Sprite::new();
                sprite.set_texture(root.texture.texture(), true);
                window.draw(&sprite);
            }

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


                    Ok(mut action) => {
                        // draw the action
                        let mut working_node = &mut root;
                        'unpack_action: loop{
                            match action
                            {
                                DrawAction::Noop => {
                                    // do nothing!
                                    break 'unpack_action;
                                },
                                DrawAction::Line( (x_0, y_0), (x_1, y_1) ) => {
                                    let mut line = VertexArray::new(PrimitiveType::Lines, 2);
                                    line[0].position = Vector2f::new(x_0 as f32, y_0 as f32);
                                    line[1].position = Vector2f::new(x_1 as f32, y_1 as f32);
                                    working_node.texture.draw(&line);
                                    break 'unpack_action;
                                },
                                DrawAction::Clear => {
                                    working_node.texture.clear(Color::rgb(0, 0, 0));
                                    break 'unpack_action;
                                },
                                DrawAction::NewComponent(uuid, z, offset, w, h) => {
                                    working_node.add_child(uuid, z, offset, w, h);
                                    break 'unpack_action;
                                },
                                DrawAction::NestedAction(uuid, next_action) => {
                                    working_node = &mut working_node.children.get_mut(&uuid).unwrap().1;
                                    action = *next_action;
                                },
                                _ => {
                                    panic!("Not implemented.");
                                }
                            }
                        }
                        root.draw_children();
                        let mut sprite = Sprite::new();
                        sprite.set_texture(root.texture.texture(), true);
                        window.draw(&sprite);

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