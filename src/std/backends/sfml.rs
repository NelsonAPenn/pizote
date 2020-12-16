extern crate sfml;

use std::collections::HashMap;
use std::thread;
use std::sync::mpsc::{channel, Sender, TryRecvError};
use crate::action::draw::{Draw, DrawAction};
use crate::bounds::{Bounds, ArbitratedBounds};
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
    pub z_index: i8,
    pub offset: (f64, f64),
    pub texture: RenderTexture,
    pub children: HashMap<String, RenderNode>
}

impl RenderNode
{
    pub fn new(arbitrated_bounds: ArbitratedBounds) -> RenderNode
    {
        let texture = RenderTexture::new(arbitrated_bounds.bounds.width as u32, arbitrated_bounds.bounds.height as u32, false).unwrap();

        let z_index = arbitrated_bounds.z_index;
        let offset = arbitrated_bounds.offset;
        RenderNode
        {
            z_index,
            offset,
            texture,
            children: HashMap::<String, RenderNode>::new()
        }
    }

    pub fn add_child(&mut self, uuid: String, arbitrated_bounds: ArbitratedBounds)
    {
        let new_node = RenderNode::new(arbitrated_bounds);
        self.children.insert(uuid, new_node);
    }

    pub fn draw_children(&mut self)
    {
        let mut children = self.children.iter_mut().map( | (_uuid, child) | child ).collect::<Vec<&mut RenderNode>>();
        children.sort_by(|a, b| a.z_index.cmp( &b.z_index ) );

        for child in children
        {
            child.draw_children();
            let mut sprite = Sprite::new();
            sprite.set_texture(child.texture.texture(), true);
            sprite.set_position(Vector2f::new(child.offset.0 as f32, child.offset.1 as f32));
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
            let mut root = RenderNode::new(ArbitratedBounds::new(0, (0., 0.), initial_bounds.width, initial_bounds.height));
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
                                DrawAction::Line( color, point_0, point_1 ) => {
                                    let mut line = VertexArray::new(PrimitiveType::Lines, 2);
                                    line[0].position = Vector2f::new(point_0.0 as f32, point_0.1 as f32);
                                    line[1].position = Vector2f::new(point_1.0 as f32, point_1.1 as f32);
                                    line[0].color = sfml::graphics::Color
                                    {
                                        r: color.0,
                                        g: color.1,
                                        b: color.2,
                                        a: color.3
                                    };
                                    line[1].color = sfml::graphics::Color
                                    {
                                        r: color.0,
                                        g: color.1,
                                        b: color.2,
                                        a: color.3
                                    };
                                    working_node.texture.draw(&line);
                                    break 'unpack_action;
                                },
                                DrawAction::Clear(color) => {
                                    let sf_color = sfml::graphics::Color
                                    {
                                        r: color.0,
                                        g: color.1,
                                        b: color.1,
                                        a: color.3
                                    };

                                    working_node.texture.clear(sf_color);
                                    break 'unpack_action;
                                },
                                DrawAction::AddArbitratedBounds(uuid, arbitrated_bounds) => {
                                    working_node.add_child(uuid, arbitrated_bounds);
                                    break 'unpack_action;
                                },
                                DrawAction::NestedAction(uuid, next_action) => {
                                    working_node = working_node.children.get_mut(&uuid).unwrap();
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