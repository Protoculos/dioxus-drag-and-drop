use super::svg::Svg;

#[derive(PartialEq, Debug, Clone)]
pub struct ItemCard {
    pub title: &'static str,
    pub draggable: bool,
    pub svgLeft: Svg,
    // pub svgRight: &'static Svg,
}
