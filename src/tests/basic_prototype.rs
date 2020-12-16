use crate::std::backends::sfml::Sfml;
use std::sync::mpsc::Sender;
use crate::bounds::{Bounds, ArbitratedBounds};
use crate::action::draw::{Draw, DrawAction, DynamicDrawableComponent, Color};
use crate::backend::Backend;
use crate::cartesian::Cartesian;

fn rotate( point: (f64, f64), angle: f64) -> (f64, f64)
{
    let x = point.0 * angle.cos() - point.1 * angle.sin();
    let y = point.0 * angle.sin() + point.1 * angle.cos();

    (x, y)
}

struct DragonFractalComponent
{
    unit_width: f64,
    initial_bounds: Bounds
}

impl DragonFractalComponent
{
    // bounds + information
    pub fn new(initial_bounds: Bounds, unit_width: f64) -> DragonFractalComponent
    {
        DragonFractalComponent{
            unit_width,
            initial_bounds
        }
    }
}

impl DynamicDrawableComponent for DragonFractalComponent
{
    fn spawn(&mut self, sender: Sender<DrawAction>)
    {
        let mut n: i32 = 0;
        let mut previous_direction = (0., -self.unit_width);
        let mut location = Cartesian {
            0: self.initial_bounds.width / 2.,
            1: self.initial_bounds.height / 2.
        };

        loop
        {
            let left: bool = (((n & -n) << 1) & n) != 0;
            previous_direction =
                if left
                {
                    rotate(previous_direction, std::f64::consts::PI / 2.)
                }
                else
                {
                    rotate(previous_direction, -std::f64::consts::PI / 2.)
                };

            let next_location = Cartesian{
                0: location.0 + previous_direction.0,
                1: location.1 + previous_direction.1
            };

            sender.send(DrawAction::Line(
                Color{
                    0: n as u8,
                    1: 255,
                    2: 0,
                    3: 255
                },
                location.clone(),
                next_location.clone()
            )).unwrap();

            location = next_location;
            n += 1;
            std::thread::sleep(std::time::Duration::from_millis(8));
        }

    }
}

#[test]
fn basic_prototype()
{
    let mut backend = Box::new(Sfml{}) as Box<dyn Backend>;

    let mut initial_bounds = Bounds::new(1000., 1000.);
    

    let sender = backend.spawn(initial_bounds.clone());

    sender.send(DrawAction::Line{
        0: Color
        {
            0: 0,
            1: 255,
            2: 0,
            3: 255
        },
        1: Cartesian{
            0: 0.,
            1: 0.
        },
        2: Cartesian
        {
            0: initial_bounds.width,
            1: initial_bounds.height
        }

    }).unwrap();

    sender.send(DrawAction::Line{
        0: Color
        {
            0: 0,
            1: 0,
            2: 255,
            3: 255
        },
        1: Cartesian{
            0: 0.,
            1: initial_bounds.height
        },
        2: Cartesian
        {
            0: initial_bounds.width,
            1: 0.
        }

    }).unwrap();

    let bg_sender = sender.clone();

    std::thread::spawn(move || {
        let mut n: u8 = 0;
        loop
        {
            bg_sender.send(DrawAction::Clear(Color{
                0: n,
                1: n,
                2: n,
                3: 255
            })).unwrap();

            if n == 255
            {
                n = 0
            }
            else
            {
                n = n + 1
            }

            std::thread::sleep_ms(8);
        }
    });

    let (bounds, sender) = initial_bounds.arbitrate( ArbitratedBounds::new( 0, (100., 100.), 600., 600. ), sender).unwrap();

    sender.send(DrawAction::Line{
        0: Color
        {
            0: 255,
            1: 0,
            2: 0,
            3: 255
        },
        1: Cartesian{
            0: 0.,
            1: 0.
        },
        2: Cartesian
        {
            0: bounds.width,
            1: bounds.height
        }

    }).unwrap();

    sender.send(DrawAction::Line{
        0: Color
        {
            0: 255,
            1: 0,
            2: 0,
            3: 255
        },
        1: Cartesian{
            0: 0.,
            1: bounds.height
        },
        2: Cartesian
        {
            0: bounds.width,
            1: 0.
        }

    }).unwrap();

    let mut dragon = DragonFractalComponent::new(bounds, 10.);

    dragon.spawn(sender);
}