use crate::action::draw::{Color, TextStyle, StrokeStyle, ShapeStyle};

pub trait Theme
{
    fn text_primary() -> TextStyle;
    fn text_emphasis() -> TextStyle;

    fn fg_primary() -> TextStyle;
    fn fg_emphasis() -> TextStyle;

    fn bg_primary() -> Color;
    fn bg_secondary() -> Color;

    // Semantic highlighting preferences
    fn function() -> TextStyle;
    fn literal() -> TextStyle;
    fn class() -> TextStyle;
    fn variable() -> TextStyle;
    fn constant() -> TextStyle;

}