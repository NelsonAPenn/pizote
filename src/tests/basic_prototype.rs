use crate::std::backends::sfml::Sfml;
use std::sync::mpsc::Sender;
use crate::bounds::{Bounds, OffsetBounds};
use crate::action::draw::{Draw, DrawAction, DynamicDrawableComponent};

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
        let mut n = 0;
        let mut previous_direction = (0., -self.unit_width);
        let mut location = (self.initial_bounds.width / 2., self.initial_bounds.height / 2.);

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

            let next_location = (location.0 + previous_direction.0, location.1 + previous_direction.1);
            sender.send(DrawAction::Line(
                location,
                next_location
            )).unwrap();

            location = next_location;
            n += 1;
            std::thread::sleep(std::time::Duration::from_millis(80));
        }

    }
}

#[test]
fn basic_prototype()
{
    let mut backend = Box::new(Sfml{}) as Box<dyn Draw>;

    let mut initial_bounds = Bounds::new(1000., 1000.);
    

    let sender = backend.spawn(initial_bounds.clone());

    let (bounds, sender) = initial_bounds.arbitrate( OffsetBounds::new( (100., 100.), 600., 600. ), sender).unwrap();

    let mut dragon = DragonFractalComponent::new(bounds, 10.);

    dragon.spawn(sender);
}