use std::sync::mpsc::{channel, Sender, TryRecvError};
use std::thread;
use crate::action::draw::{DrawAction};
use std::time::Duration;

// pub struct Dimension
// {
//     pub invariant: f64
// }

// pub trait Bounds
// {
//     fn arbitrate(&mut self, portion: Box<dyn Bounds>) -> Result<(), String>;
//     fn clip(&self, action: DrawAction) -> DrawAction;
// }



#[derive(Clone)]
pub struct Bounds // treat as two triangular bounds... eventually
{
    arbitrated_bounds: Vec<ArbitratedBounds>,
    pub width: f64,
    pub height: f64
}

#[derive(Clone)]
pub struct ArbitratedBounds
{
    pub z_index: i8,
    pub offset: (f64, f64),
    pub bounds: Bounds
}

impl ArbitratedBounds
{
    pub fn new( z_index: i8, offset: (f64, f64), width: f64, height: f64) -> ArbitratedBounds
    {
        ArbitratedBounds
        {
            z_index,
            offset,
            bounds: Bounds::new(width, height)
        }
    }
}

impl Bounds
{
    pub fn new(width: f64, height: f64) -> Bounds
    {
        Bounds
        {
            width,
            height,
            arbitrated_bounds: Vec::<ArbitratedBounds>::new()
        }
    }

    pub fn contains_point(&self, point: &(f64, f64)) -> bool
    {
        let (x, y) = point.clone();
        x >= 0. && y >= 0. && x <= self.width && y <= self.height
    }


    pub fn arbitrate(&mut self, portion: ArbitratedBounds, sender: Sender<DrawAction>) -> Result<(Bounds, Sender<DrawAction>), String>
    {
        let (x, y) = portion.offset;
        let (w, h) = (portion.bounds.width, portion.bounds.height);
        // if exits parent, no
        if x < 0. || y < 0. || x + w > self.width || y + h > self.height
        {
            return Err(String::from("Exceeds parent bounds."));
        }
        

        self.arbitrated_bounds.push(portion.clone());

        let uuid = String::from("thing");
        sender.send(DrawAction::AddArbitratedBounds(uuid.clone(), portion.clone())).unwrap();

        let (tx, rx) = channel::<DrawAction>();

        // spawn clip middleware
        thread::spawn( move || {
            'recv: loop
            {
                match rx.try_recv()
                {
                    Ok(action) =>
                    {
                        sender.send(DrawAction::NestedAction(uuid.clone(), Box::new(action))).unwrap();
                    },
                    Err(TryRecvError::Disconnected) => {
                        break 'recv;
                    },
                    _ => {
                        thread::sleep(Duration::from_millis(crate::constants::WAIT_ON_EMPTY_MS));
                    }
                }
            }
        });

        Ok( (portion.bounds, tx) )
    }
}

// pub struct TriangularBounds(
//     (Dimension, Dimension),
//     (Dimension, Dimension),
//     (Dimension, Dimension)
// );

pub fn bounds_overlapping(bounds_list: Vec<ArbitratedBounds>) -> bool
{
    false
}