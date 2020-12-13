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
    arbitrated_bounds: Vec<OffsetBounds>,
    pub width: f64,
    pub height: f64
}

#[derive(Clone)]
pub struct OffsetBounds
{
    offset: (f64, f64),
    bounds: Bounds
}

impl OffsetBounds
{
    pub fn new( offset: (f64, f64), width: f64, height: f64) -> OffsetBounds
    {
        OffsetBounds
        {
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
            arbitrated_bounds: Vec::<OffsetBounds>::new()
        }
    }

    pub fn contains_point(&self, point: &(f64, f64)) -> bool
    {
        let (x, y) = point.clone();
        x >= 0. && y >= 0. && x <= self.width && y <= self.height
    }


    pub fn clip(&self, action: DrawAction) -> DrawAction
    {
        let new_action = match action
        {
            DrawAction::Line( (x_0, y_0), (x_1, y_1) ) => {
                // TODO: fix this
                if !self.contains_point( &(x_0, y_0) ) || !self.contains_point( &(x_1, y_1) ){
                    DrawAction::Noop
                }
                else{
                    action
                }
            },
            _ => {
                action
            }
        };
        return new_action
    }

    pub fn arbitrate(&mut self, portion: OffsetBounds, sender: Sender<DrawAction>) -> Result<(Bounds, Sender<DrawAction>), String>
    {
        let (x, y) = portion.offset;
        let (w, h) = (portion.bounds.width, portion.bounds.height);
        // if exits parent, no
        if x < 0. || y < 0. || x + w > self.width || y + h > self.height
        {
            return Err(String::from("Exceeds parent bounds."));
        }
        

        self.arbitrated_bounds.push(portion.clone());

        let (tx, rx) = channel::<DrawAction>();

        let bounds_copy = portion.bounds.clone();
        // spawn clip middleware
        thread::spawn( move || {
            'recv: loop
            {
                match rx.try_recv()
                {
                    Ok(mut action) =>
                    {
                        // clip
                        action = bounds_copy.clip(action);
                        // transform
                        action = DrawAction::transform(action, (x, y) );
                        sender.send(action).unwrap();
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

pub fn bounds_overlapping(bounds_list: Vec<OffsetBounds>) -> bool
{
    false
}