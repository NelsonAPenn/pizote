use crate::bounds::Bounds;
use std::sync::mpsc;

pub trait Component
{
    fn receive_information();
}
