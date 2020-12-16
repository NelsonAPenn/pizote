use crate::bounds::{Bounds};
use std::sync::mpsc::{Sender, Receiver};

pub enum DrawAction
{
    Noop,

    Clear,

    // (x_0, y_0), (x_1, y_1)
    Line((f64, f64), (f64, f64)),

    // center, w, h
    Ellipse((f64, f64), f64, f64),
    Image(),

    // uuid, z-index, origin, w, h
    NewComponent(String, i8, (f64, f64), f64, f64),

    // uuid, action
    NestedAction(String, Box<DrawAction>)
    

}

impl DrawAction
{
    pub fn transform(action: DrawAction, delta: (f64, f64)) -> DrawAction
    {
        let (dx, dy) = delta;
        match action
        {
            DrawAction::Line( (x_0, y_0), (x_1, y_1) )=>
            {
                DrawAction::Line( (x_0 + dx, y_0 + dy), (x_1 + dx, y_1 + dy) )
            }
            DrawAction::Noop | DrawAction::Clear => { action },
            _ => {
                panic!("Not implemented.");
            }

        }
    }
}

pub trait StaticDrawableComponent
{
    // For initial drawing. Should be joined if used
    fn spawn(&mut self, sender: Sender<DrawAction>);
}

pub trait DynamicDrawableComponent
{
    // fn get_receiver(&mut self) -> Receiver<DrawAction>;
    fn spawn(&mut self, sender: Sender<DrawAction>);
}

pub trait Draw
{
    fn spawn(&mut self, initial_bounds: Bounds) -> Sender<DrawAction>;
}
