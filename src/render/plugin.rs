use crate::resources::{FillTessellator, StrokeTessellator};
use bevy::app::{App, Plugin};

use crate::render::svg2d;

/// Plugin that renders [`Svg`](crate::svg::Svg)s in 2D
pub struct SvgPlugin;

impl Plugin for SvgPlugin {
    fn build(&self, app: &mut App) {
        let fill_tess = FillTessellator::default();
        let stroke_tess = StrokeTessellator::default();
        app.insert_resource(fill_tess).insert_resource(stroke_tess);

        app.add_plugins(svg2d::RenderPlugin);
    }
}
