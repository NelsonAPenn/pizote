use crate::bounds::Bounds;
use crate::action::draw::DrawAction;
use std::sync::mpsc::{channel, Sender, Receiver, TryRecvError};

// pub struct ClipMiddleware<'a>
// {
//     // issue here: bounds cannot be mutated in the parent component
//     // most likely clip to the immutable bounds of the child object
//     // are they immutable? what happens when you resize the page?
//     // kill me
//     pub bounds: &'a Box<dyn Bounds>, 
//     incoming: Receiver<DrawAction>, // should these just be spawned? Probably
//     outgoing: Sender<DrawAction>
// }

// impl<'a> ClipMiddleware<'a>
// {
//     pub fn new(bounds: &'a Box<dyn Bounds>, outgoing: Sender<DrawAction>) -> (ClipMiddleware<'a>, Sender<DrawAction>)
//     {
//         let (tx, rx) = channel::<DrawAction>();

//         (
//             ClipMiddleware{
//                 bounds,
//                 incoming: rx,
//                 outgoing
//             },

//             tx
//         )

//     }
// }