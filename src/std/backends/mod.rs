pub mod pdf;
pub mod sfml;
pub mod svg;
pub mod web;
use crate::action::*;
use crate::information::*;

  
trait Backend: draw::Draw {}
impl<T> Backend for T where T: draw::Draw {}