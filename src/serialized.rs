use nanoserde::DeJson;

pub type OptStr = Option<String>;

#[derive(DeJson, Clone)]
pub struct STextBoxProps {
    pub disabled: OptStr,
    pub placeholder: OptStr,
    pub value: OptStr,
}

#[derive(DeJson, Clone)]
pub struct STextBoxBundle {
    pub text_box: Option<STextBoxProps>,
    pub styles: Option<SKStyle>,
    // pub on_event: OnEvent,
    // pub on_layout: OnLayout,
    // pub on_change: OnChange,
    pub focusable: OptStr,
    pub name: String,
}


#[derive(DeJson)]
pub struct KayakData {
    pub assets: Option<SAssets>,
    pub styles: Option<Vec<SKStyle>>,
    pub widgets: Option<SWidgets>,
    pub bundles: Option<SBundles>,
}

#[derive(DeJson, Clone)]
pub struct SClipBundle {
    pub clip: OptStr,
    pub styles: Option<SKStyle>,
    pub name: String,
}

#[derive(DeJson, Clone)]
pub struct SBackgroundBundle {
    pub background: OptStr,
    pub styles: Option<SKStyle>,
    pub name: String,
}

#[derive(DeJson, Clone)]
pub struct SSize {
    /// The width of the 2-dimensional area.
    pub width: OptStr,
    /// The height of the 2-dimensional area.
    pub height: OptStr,
}


#[derive(DeJson, Clone)]
pub struct SUiRect {
    pub left: OptStr,
    /// The value corresponding to the right side of the UI rect.
    pub right: OptStr,
    /// The value corresponding to the top side of the UI rect.
    pub top: OptStr,
    /// The value corresponding to the bottom side of the UI rect.
    pub bottom: OptStr,
}


#[derive(DeJson, Clone)]
pub struct SRect {
    pub posy: OptStr,
    pub posx: OptStr,
    pub width: OptStr,
    pub height: OptStr,
    pub z_index: OptStr,
}

#[derive(DeJson, Clone)]
pub struct SChildren {
    pub widgets: Option<SWidgets>
}

#[derive(DeJson, Clone)]
pub struct STextureAtlasProps {
    /// The handle to image
    pub handle: Option<SImage>,
    /// The position of the tile (in pixels)
    pub position: Option<Vec<OptStr>>,
    /// The size of the tile (in pixels)
    pub tile_size: Option<Vec<OptStr>>,
}


#[derive(DeJson, Clone)]
pub struct STextureAtlasBundle {
    pub atlas: Option<STextureAtlasProps>,
    pub styles: Option<SKStyle>,
    pub name: String,
}

#[derive(DeJson, Clone)]
pub struct SWindow {
    /// If true, allows the window to be draggable by its title bar
    pub draggable: OptStr,
    /// The initial position at which to display the window in pixels
    pub initial_position: Option<Vec<OptStr>>,
    /// The size of the window in pixels
    pub size: Option<Vec<OptStr>>,
    /// The text to display in the window's title bar
    pub title: Option<String>,
    /// Styles for the main window quad.
    pub window_styles: SKStyle,
    /// A set of styles to apply to the children element wrapper.
    pub children_styles: SKStyle,
}

#[derive(DeJson, Clone)]
pub struct SWindowBundle {
    pub window: Option<SWindow>,
    pub styles: Option<SKStyle>,
    // pub children: SChildren,
    pub name: String
}

#[derive(DeJson, Clone)]
pub struct SElementBundle {
    pub element: OptStr,
    pub styles: Option<SKStyle>,
    // pub children: SChildren,
    pub name: String
}


#[derive(DeJson, Clone)]
pub struct SImageAsset {
    pub name: String,
    pub path: String,
}
#[derive(DeJson, Clone)]
pub struct SFontAsset {
    pub name: String,
    pub path: String,
}
#[derive(DeJson, Clone)]
pub struct SAssets {
    pub images: Option<Vec<SImageAsset>>,
    pub fonts: Option<Vec<SFontAsset>>,
}

#[derive(DeJson, Clone)]

pub struct SBevyStyle {
    pub display: OptStr,
    /// Whether to arrange this node relative to other nodes, or positioned absolutely
    pub position_type: OptStr,
    pub direction: OptStr,
    pub flex_direction: OptStr,
    pub flex_wrap: OptStr,
    pub align_items: OptStr,
    pub align_self: OptStr,
    pub align_content: OptStr,
    pub justify_content: OptStr,
    pub position: Option<SUiRect>,
    pub margin: Option<SUiRect>,
    pub padding: Option<SUiRect>,
    pub border: Option<SUiRect>,
    pub flex_grow: OptStr,
    pub flex_shrink: OptStr,
    pub flex_basis: OptStr,    
    pub size: Option<SSize>,    
    pub min_size: Option<SSize>,        
    pub max_size: Option<SSize>,        
    pub aspect_ratio: OptStr,    
    pub overflow: OptStr,                
}

#[derive(DeJson, Clone)]
pub struct SKStyle {
    pub name: String,
    pub extends: OptStr,
    pub background_color: OptStr,
    pub border: OptStr,
    pub border_color: OptStr,
    pub border_radius: OptStr,
    pub bottom: OptStr,
    pub col_between: OptStr,
    pub color: OptStr,
    pub content: OptStr,
    pub cursor: OptStr,   
    pub font: OptStr,
    pub font_size: OptStr,
    pub height: OptStr,
    pub layout_type: OptStr,
    pub left: OptStr,
    pub line_height: OptStr,
    pub max_height: OptStr,
    pub max_width: OptStr,
    pub min_height: OptStr,
    pub min_width: OptStr,
    pub offset: OptStr,
    pub padding: OptStr,
    pub padding_top: OptStr,
    pub padding_bottom: OptStr,
    pub padding_left: OptStr,
    pub padding_right: OptStr,
    pub position_type: OptStr,
    pub right: OptStr,
    pub row_between: OptStr,
    pub top: OptStr,
    pub width: OptStr,
    pub z_index: OptStr,
}

#[derive(DeJson, Clone)]
pub struct STextProps {
    pub alignment: OptStr,
    pub content: OptStr,
    pub font: OptStr,
    pub line_height: OptStr,
    pub show_cursor: OptStr,
    pub size: OptStr,
    pub user_styles: SKStyle,
    pub word_wrap: OptStr
}    

#[derive(DeJson, Clone)]
pub struct SImage {
    pub path: OptStr,
    pub image_ref: OptStr,
}    

#[derive(DeJson, Clone)]
pub struct SButton {
    pub name: String,
    pub styles: Option<SKStyle>,
}

#[derive(DeJson, Clone)]
pub struct SButtonBundle {
    pub button: Option<SButton>,
    pub styles: Option<SKStyle>,
    // pub on_event: OnEvent,
    pub name: String,
}

#[derive(DeJson, Clone)]
pub struct SImageBundle {
    pub name: String,
    pub image: SImage,
    pub styles: SBevyStyle,        
    pub image_mode: OptStr,
    pub calculated_size: OptStr,
    // pub struct BackgroundColor(pub Color);
    pub background_color: OptStr,
    pub focus_policy: OptStr,
    pub transform: STransform,
    pub visibility: OptStr,
    pub computed_visibility: OptStr,
}  

#[derive(DeJson, Clone)]
pub struct SVec2 {
    pub x: OptStr,
    pub y: OptStr,
}

#[derive(DeJson, Clone)]
pub struct SVec3 {
    pub x: OptStr,
    pub y: OptStr,
    pub z: OptStr,
}

#[derive(DeJson, Clone)]
pub struct STransform {
    pub translation: SVec3, 
    pub rotation: SVec3, 
    pub scale: SVec3,
}


// Transform
// pub translation: Vec3,
// /// Rotation of the entity.
// ///
// /// See the [`3d_rotation`] example for usage.
// ///
// /// [`3d_rotation`]: https://github.com/bevyengine/bevy/blob/latest/examples/transforms/3d_rotation.rs
// pub rotation: Quat,
// /// Scale of the entity.
// ///
// /// See the [`scale`] example for usage.
// ///
// /// [`scale`]: https://github.com/bevyengine/bevy/blob/latest/examples/transforms/scale.rs
// pub scale: Vec3,


// pub node: Node,
// /// Describes the style including flexbox settings
// pub style: Style,
// /// Configures how the image should scale
// pub image_mode: ImageMode,
// /// The calculated size based on the given image
// pub calculated_size: CalculatedSize,
// /// The background color, which serves as a "fill" for this node
// ///
// /// When combined with `UiImage`, tints the provided image.
// pub background_color: BackgroundColor,
// /// The image of the node
// pub image: UiImage,
// /// Whether this node should block interaction with lower nodes
// pub focus_policy: FocusPolicy,
// /// The transform of the node
// ///
// /// This field is automatically managed by the UI layout system.
// /// To alter the position of the `NodeBundle`, use the properties of the [`Style`] component.
// pub transform: Transform,
// /// The global transform of the node
// ///
// /// This field is automatically managed by the UI layout system.
// /// To alter the position of the `NodeBundle`, use the properties of the [`Style`] component.
// pub global_transform: GlobalTransform,
// /// Describes the visibility properties of the node
// pub visibility: Visibility,
// /// Algorithmically-computed indication of whether an entity is visible and should be extracted for rendering
// pub computed_visibility: ComputedVisibility,
// /// Indicates the depth at which the node should appear in the UI
// pub z_index: ZIndex,

#[derive(DeJson, Clone)]
pub struct STextWidgetBundle {
    pub name: String,
    pub text: STextProps,
    pub styles: SKStyle,
}

#[derive(DeJson, Clone)]
pub struct SWidgets {
    pub buttons: Option<Vec<SButton>>,
}


#[derive(DeJson, Clone)]
pub struct SBundles {
    pub text_widget_bundles: Option<Vec<STextWidgetBundle>>,
    pub image_bundles: Option<Vec<SImageBundle>>,
    pub window_bundles: Option<Vec<SWindowBundle>>,
    pub texture_atlas_bundles: Option<Vec<STextureAtlasBundle>>,
    pub button_bundles: Option<Vec<SButtonBundle>>,
    pub background_bundles: Option<Vec<SBackgroundBundle>>,    
    pub clip_bundles: Option<Vec<SClipBundle>>,    
    pub text_box_bundles: Option<Vec<STextBoxBundle>>,        
    pub element_bundles: Option<Vec<SElementBundle>>,        
}