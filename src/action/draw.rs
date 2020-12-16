use crate::bounds::{Bounds, ArbitratedBounds};
use std::sync::mpsc::{Sender, Receiver};
use crate::cartesian::Cartesian;

pub enum DrawAction
{
    Clear(Color),

    // (x_0, y_0), (x_1, y_1)
    Line( Color, Cartesian, Cartesian),

    // center, w, h
    Ellipse(ShapeStyle, Cartesian, f64, f64),

    Image(),

    Text(TextStyle, String),

    // uuid, z-index, origin, w, h
    AddArbitratedBounds(String, ArbitratedBounds),

    // uuid, action
    NestedAction(String, Box<DrawAction>)

    

}

pub struct TextStyle
{
    pub size: f64,
    pub color: Color
    // pub font:
    // pub variant:
}


pub struct StrokeStyle
{
    pub stroke_color: Color,
    // font: Box<dyn Font>
    pub stroke_width: f64
}

pub struct ShapeStyle
{
    pub stroke_style: StrokeStyle,
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
