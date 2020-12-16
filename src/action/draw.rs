use crate::bounds::{Bounds, ArbitratedBounds};
use std::sync::mpsc::{Sender, Receiver};
use crate::cartesian::Cartesian;

pub enum DrawAction
{
    Clear(Color),

    // (x_0, y_0), (x_1, y_1)
    Line( Color, Cartesian, Cartesian),

    // center, w, h
    Ellipse(Style, Cartesian, f64, f64),
    Image(),

    // uuid, z-index, origin, w, h
    AddArbitratedBounds(String, ArbitratedBounds),

    // uuid, action
    NestedAction(String, Box<DrawAction>)
    

}

pub struct Style
{
    pub stroke_color: Color,
    // font: Box<dyn Font>
    pub fill_color: Color
}


pub struct Color(pub u8, pub u8, pub u8, pub u8);

impl Color
{
    // pub fn from_rgba()
    // pub fn from_hsla()
    // ...
}


impl DrawAction { }

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
