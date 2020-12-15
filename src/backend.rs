use crate::action::*;
use crate::information::*;

  
pub trait Backend: draw::Draw {}
impl<T> Backend for T where T: draw::Draw {}