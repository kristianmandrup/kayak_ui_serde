use std::any::Any;

use bevy::prelude::Color;
use kayak_ui::prelude::{Edge, KStyle, Corner, KCursorIcon, RenderCommand};
use morphorm::{Units, LayoutType};

use crate::{json_deserializer::{Style}, ui_parser::{Conv, UiParser}, ui_color::parse_color, ui_edge::{EdgeBuilder, to_edge_units}, ui_corner::CornerBuilder, ui_unit::UiUnit, ui_cursor_icon::to_cursor_icon, ui_layout_type::to_layout_type};

pub struct StyleBuilder {
    node: Style
}
impl StyleBuilder {
    pub fn new(node: Style) -> Self {
        Self {
            node
        }
    }

    fn prop_color(prop: &Option<String>) -> Option<Color> {
        let str = Conv::get_prop(prop);
        if let Some(val) = str {
            parse_color(val.as_str())    
        } else {
            None
        }        
    }


    fn background_color(&self) -> Option<Color> {
        let prop = &self.node.background_color.clone();
        StyleBuilder::prop_color(prop)
    }

    fn border(&self) -> Option<Edge<f32>> {
        if let Some(val) = self.node.border.clone() {
            let edge = val.clone();
            EdgeBuilder::create_from_str(edge).parse().ok()    
        } else {
            None
        }
    }

    fn border_color(&self) -> Option<Color> {
        let prop = &self.node.border_color.clone();
        StyleBuilder::prop_color(prop)
    }

    fn border_radius(&self) -> Option<Corner<f32>> {
        if let Some(val) = self.node.border_radius.clone() {
            let corner = val.clone();
            CornerBuilder::create_from_str(corner).parse().ok()    
        } else {
            None
        }
    }

    fn bottom(&self) -> Option<Units> {
        let prop = &self.node.border_color.clone();
        UiUnit::new(prop.clone()).parse().ok()
    }
    
    fn color(&self) -> Option<Color> {
        let prop = &self.node.color.clone();
        StyleBuilder::prop_color(prop)
    }

    fn col_between(&self) -> Option<Units> {
        let prop = &self.node.col_between.clone();
        UiUnit::new(prop.clone()).parse().ok()
    }
    
    fn cursor(&self) -> Option<KCursorIcon> {
        let prop = &self.node.cursor.clone();
        if let Some(val) = prop {
            let icon = to_cursor_icon(val.clone());
            Some(KCursorIcon(icon))
        } else {
            None
        }
    }
    
    fn font(&self) -> Option<String> {
        let prop = &self.node.font.clone();
        Conv::get_prop(prop)
    }

    fn font_size(&self) -> Option<f32> {
        let prop = &self.node.font_size.clone();
        if let Some(val) = prop {
            Conv(val.clone()).to_f32()
        } else {
            None
        }        
    }

    fn height(&self) -> Option<Units> {
        let prop = &self.node.height.clone();
        UiUnit::new(prop.clone()).parse().ok()
    }

    fn layout_type(&self) -> Option<LayoutType> {
        let prop = &self.node.layout_type.clone();
        if let Some(val) = prop {
            Some(to_layout_type(val.clone()))
        } else {
            None
        }        

        
    }    

    fn left(&self) -> Option<Units> {
        let prop = &self.node.left.clone();
        UiUnit::new(prop.clone()).parse().ok()
    }

    fn line_height(&self) -> Option<f32> {
        let prop = &self.node.line_height.clone();
        if let Some(val) = prop {
            Conv(val.clone()).to_f32()
        } else {
            None
        }        
    }

    fn max_height(&self) -> Option<Units> {
        let prop = &self.node.max_height.clone();
        UiUnit::new(prop.clone()).parse().ok()
    }

    fn max_width(&self) -> Option<Units> {
        let prop = &self.node.max_width.clone();
        UiUnit::new(prop.clone()).parse().ok()
    }

    fn min_height(&self) -> Option<Units> {
        let prop = &self.node.min_height.clone();
        UiUnit::new(prop.clone()).parse().ok()
    }

    fn min_width(&self) -> Option<Units> {
        let prop = &self.node.min_width.clone();
        UiUnit::new(prop.clone()).parse().ok()
    }

    fn offset(&self) -> Option<Edge<Units>> {
        let prop = &self.node.offset.clone();
        if let Some(_) = prop {
            Some(to_edge_units(prop.clone()))
        } else {
            None
        }        
    }

    fn padding(&self) -> Option<Edge<Units>> {
        let prop = &self.node.padding.clone();
        if let Some(_) = prop {
            Some(to_edge_units(prop.clone()))
        } else {
            None
        }
    }

    fn padding_top(&self) -> Option<Units> {
        let prop = &self.node.padding_top.clone();
        UiUnit::new(prop.clone()).parse().ok()
    }

    fn padding_bottom(&self) -> Option<Units> {
        let prop = &self.node.padding_bottom.clone();
        UiUnit::new(prop.clone()).parse().ok()
    }

    fn padding_left(&self) -> Option<Units> {
        let prop = &self.node.padding_left.clone();
        UiUnit::new(prop.clone()).parse().ok()
    }

    fn padding_right(&self) -> Option<Units> {
        let prop = &self.node.padding_right.clone();
        UiUnit::new(prop.clone()).parse().ok()
    }

    // fn pointer_events(&self) -> PointerEvents {
    //     let prop = &self.node.padding_right.clone();
    //     UiPointerEvents::new(prop.clone()).parse().unwrap()
    // }
    
    
    // fn position_type(&self) -> KPositionType {
    //     let prop = &self.node.position_type.clone();
    //     UiPositionType::new(prop.clone()).parse().unwrap()
    // }

    // fn render_command(&self) -> RenderCommand {
    //     let prop = &self.node.position_type.clone();
    //     UiRenderCommand::new(prop.clone()).parse().unwrap()
    // }

    fn right(&self) -> Option<Units> {
        let prop = &self.node.right.clone();
        UiUnit::new(prop.clone()).parse().ok()
    }

    fn row_between(&self) -> Option<Units> {
        let prop = &self.node.row_between.clone();
        UiUnit::new(prop.clone()).parse().ok()
    }

    fn top(&self) -> Option<Units> {
        let prop = &self.node.top.clone();
        UiUnit::new(prop.clone()).parse().ok()
    }

    fn width(&self) -> Option<Units> {
        let prop = &self.node.width.clone();
        UiUnit::new(prop.clone()).parse().ok()
    }

    fn z_index(&self) -> Option<i32> {
        let prop = &self.node.z_index.clone();
        if let Some(val) = prop {
            Conv(val.clone()).to_type::<i32>()
        } else {
            None
        }        
    }
}

impl UiParser for StyleBuilder {
    fn parse(&self) -> Result<Box<dyn Any>, &'static str> {
        let background_color = self.background_color();
        let border = self.border();
        let border_color = self.border_color();
        let border_radius = self.border_radius();
        let bottom = self.bottom();
        let color = self.color();
        let col_between = self.col_between();
        let cursor = self.cursor();
        let font = self.font();
        let font_size = self.font_size();
        let height = self.height();
        let layout_type = self.layout_type();
        let left = self.left();
        let line_height = self.line_height();
        let max_height = self.max_height();
        let max_width = self.max_width();
        let min_height = self.min_height();
        let min_width = self.min_width();
        let offset = self.offset();
        let padding = self.padding();
        let padding_top = self.padding_top();        
        let padding_bottom = self.padding_bottom();        
        let padding_left = self.padding_left();
        let padding_right = self.padding_right();
        let right = self.right();
        let row_between = self.row_between();
        let top = self.top();
        let width = self.width();        
        let z_index = self.z_index();

        let widget = KStyle {
            // background_color: background_color.into(),
            // border: border.into(),
            // border_color: border_color.into(),
            // border_radius: border_radius.into(),
            // bottom: bottom.into(),
            // color: color.into(),
            // col_between: col_between.into(),
            // cursor: cursor.into(),
            // font: font.into(),
            // font_size: font_size.into(),
            // height: height.into(),
            // layout_type: layout_type.into(),
            // left: left.into(),
            // line_height: line_height.into(),
            // max_height: max_height.into(),
            // max_width: max_width.into(),
            // min_height: min_height.into(),
            // min_width: min_width.into(),
            // offset: offset.into(),
            // padding: padding.into(),
            // padding_top: padding_top.into(),
            // padding_bottom: padding_bottom.into(),
            // padding_left: padding_left.into(),
            // padding_right: padding_right.into(),
            // right: right.into(),
            // row_between: row_between.into(),
            // top: top.into(),
            // width: width.into(),
            // z_index: z_index.into(),
            ..Default::default()
        };
        Ok(Box::new(widget))
    }
}

// impl KStyle {
//     /// Returns a `Style` object where all fields are set to their own initial values
//     ///
//     /// This is the actual "default" to apply over any field marked as [`StyleProp::Unset`] before
//     /// resolving the style.
//     pub fn initial() -> Self {
//         Self {
//             background_color: StyleProp::Default,
//             border: StyleProp::Default,
//             border_color: StyleProp::Default,
//             border_radius: StyleProp::Default,
//             bottom: StyleProp::Default,
//             color: StyleProp::Inherit,
//             cursor: StyleProp::Inherit,
//             col_between: StyleProp::Default,
//             font: StyleProp::Inherit,
//             font_size: StyleProp::Inherit,
//             height: StyleProp::Default,
//             layout_type: StyleProp::Default,
//             line_height: StyleProp::Inherit,
//             left: StyleProp::Default,
//             max_height: StyleProp::Default,
//             max_width: StyleProp::Default,
//             min_height: StyleProp::Default,
//             min_width: StyleProp::Default,
//             offset: StyleProp::Default,
//             padding: StyleProp::Default,
//             padding_bottom: StyleProp::Default,
//             padding_left: StyleProp::Default,
//             padding_right: StyleProp::Default,
//             padding_top: StyleProp::Default,
//             pointer_events: StyleProp::Default,
//             position_type: StyleProp::Default,
//             render_command: StyleProp::Value(RenderCommand::Layout),
//             right: StyleProp::Default,
//             row_between: StyleProp::Default,
//             top: StyleProp::Default,
//             width: StyleProp::Default,
//             z_index: StyleProp::Default,
//         }
//     }
// }

// pub struct KStyle {
//     /// The background color of this widget
//     ///
//     /// Only applies to widgets marked [`RenderCommand::Quad`]
//     pub background_color : StyleProp<Color>,
//     /// The color of the border around this widget
//     ///
//     /// Currently, this controls all border sides.
//     ///
//     /// Only applies to widgets marked [`RenderCommand::Quad`]
//     pub border_color: StyleProp<Color>,
//     /// The radius of the corners (in pixels)
//     ///
//     /// The order is (Top, Right, Bottom, Left).
//     ///
//     /// Only applies to widgets marked [`RenderCommand::Quad`] and [`RenderCommand::Image`]
//     pub border_radius: StyleProp<Corner<f32>>,
//     /// The widths of the borders (in pixels)
//     ///
//     /// The order is (Top, Right, Bottom, Left).
//     ///
//     /// Only applies to widgets marked [`RenderCommand::Quad`]
//     pub border: StyleProp<Edge<f32>>,
//     /// The distance between the bottom edge of this widget and the bottom edge of its containing widget
//     pub bottom: StyleProp<Units>,
//     /// The text color for this widget
//     ///
//     /// This property defaults to [`StyleProp::Inherit`] meaning that setting this field to some value will
//     /// cause all descendents to receive that value, up to the next set value.
//     ///
//     /// Only applies to widgets marked [`RenderCommand::Text`]
//     pub color: StyleProp<Color>,
//     /// The spacing between child widgets along the horizontal axis
//     pub col_between: StyleProp<Units>,
//     /// The cursor icon to display when hovering this widget
//     pub cursor: StyleProp<KCursorIcon>,
//     /// The font name for this widget
//     ///
//     /// Only applies to [`RenderCommand::Text`]
//     pub font: StyleProp<String>,
//     /// The font size for this widget, in pixels
//     ///
//     /// Only applies to [`RenderCommand::Text`]
//     pub font_size: StyleProp<f32>,
//     /// The height of this widget
//     pub height: StyleProp<Units>,
//     /// The layout method for children of this widget
//     pub layout_type: StyleProp<LayoutType>,
//     /// The distance between the left edge of this widget and the left edge of its containing widget
//     pub left: StyleProp<Units>,
//     /// The line height for this widget, in pixels
//     ///
//     /// Only applies to [`RenderCommand::Text`]
//     pub line_height: StyleProp<f32>,
//     /// The maximum height of this widget
//     pub max_height: StyleProp<Units>,
//     /// The maximum width of this widget
//     pub max_width: StyleProp<Units>,
//     /// The minimum height of this widget
//     pub min_height: StyleProp<Units>,
//     /// The minimum width of this widget
//     pub min_width: StyleProp<Units>,
//     /// The positional offset from this widget's default position
//     ///
//     /// This property has lower precedence than its more specific counterparts
//     /// ([`top`](Self::top), [`right`](Self::right), [`bottom`](Self::bottom), and [`left`](Self::left)),
//     /// allowing it to be overridden.
//     ///
//     /// For widgets with a [`position_type`](Self::position_type) of [`PositionType`](PositionType::ParentDirected)
//     /// this acts like margin around the widget. For [`PositionType`](PositionType::SelfDirected) this
//     /// acts as the actual position from the parent.
//     pub offset: StyleProp<Edge<Units>>,
//     /// The inner padding between the edges of this widget and its children
//     ///
//     /// This property has lower precedence than its more specific counterparts
//     /// ([`padding_top`](Self::padding_top), [`padding_right`](Self::padding_right),
//     /// [`padding_bottom`](Self::padding_bottom), and [`padding_left`](Self::padding_left)), allowing it
//     /// to be overridden.
//     ///
//     /// A child with their own padding properties set to anything other than [`Units::Auto`] will
//     /// override the padding set by this widget.
//     pub padding: StyleProp<Edge<Units>>,
//     /// The inner padding between the bottom edge of this widget and its children
//     ///
//     /// A child with their own `bottom` property set to anything other than `Units::Auto` will
//     /// override the padding set by this widget
//     pub padding_bottom: StyleProp<Units>,
//     /// The inner padding between the left edge of this widget and its children
//     ///
//     /// A child with their own `left` property set to anything other than `Units::Auto` will
//     /// override the padding set by this widget
//     pub padding_left: StyleProp<Units>,
//     /// The inner padding between the right edge of this widget and its children
//     ///
//     /// A child with their own `right` property set to anything other than `Units::Auto` will
//     /// override the padding set by this widget
//     pub padding_right: StyleProp<Units>,
//     /// The inner padding between the top edge of this widget and its children
//     ///
//     /// A child with their own `top` property set to anything other than `Units::Auto` will
//     /// override the padding set by this widget
//     pub padding_top: StyleProp<Units>,
//     /// Controls how the pointer interacts with the widget
//     ///
//     /// This can be used to block pointer events on itself and/or its children if needed, allowing
//     /// the event to "pass through" to widgets below.
//     pub pointer_events: StyleProp<PointerEvents>,
//     /// The position type of the widget relative to its parent
//     pub position_type: StyleProp<KPositionType>,
//     /// The render method for this widget
//     ///
//     /// This controls what actually gets rendered and how it's rendered.
//     pub render_command: StyleProp<RenderCommand>,
//     /// The distance between the right edge of this widget and the right edge of its containing widget
//     pub right: StyleProp<Units>,
//     /// The spacing between child widgets along the vertical axis
//     pub row_between: StyleProp<Units>,
//     /// The distance between the top edge of this widget and the top edge of its containing widget
//     pub top: StyleProp<Units>,
//     /// The width of this widget
//     pub width: StyleProp<Units>,
//     /// The z-index relative to it's parent.
//     pub z_index: StyleProp<i32>,
// }
// }