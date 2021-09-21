#[cfg(feature = "no-std")]
use crate::float::FloatExt;
use crate::layout::*;
use crate::css::*;
use crate::dom::*;
use crate::*;
use alloc::string::*;
use alloc::vec::*;
use rusttype::{point, Font, Scale};
use crate::layout::DEFAULT_FONT_SIZE;
use tiny_skia::{PixmapMut, Paint, PathBuilder, Transform, FillRule};

pub struct Renderer<'a> {
    pub pixmap: PixmapMut<'a>,
    pub width: u32,
    pub height: u32,
}

impl<'a> Renderer<'a> {
    pub fn new(buffer: &'a mut [u8], width: u32, height: u32) -> Self {
        Self {
            pixmap: PixmapMut::from_bytes(buffer, width, height).unwrap(),
            width,
            height,
        }
    }

    fn draw_at(&mut self, x: u32, y: u32, color: Color) {
        let pos = ((y * self.width + x) * 4) as usize;
        self.pixmap.data_mut()[pos + 0] = color.r;
        self.pixmap.data_mut()[pos + 1] = color.g;
        self.pixmap.data_mut()[pos + 2] = color.b;
        self.pixmap.data_mut()[pos + 3] = color.a;
    }

    fn draw_text(&mut self, s: String, size: f32, base_x: u32, base_y: u32) {
        let font = Font::try_from_bytes(MPLUS_FONT).expect("font");

        let height: f32 = size; // 12.4;
        let _pixel_height = height.ceil() as usize;
        let scale = Scale {
            x: height,
            y: height,
        };

        let v_metrics = font.v_metrics(scale);
        let offset = point(0.0, v_metrics.ascent);
        let glyphs: Vec<_> = font.layout(s.as_str(), scale, offset).collect();
        let _width = glyphs
            .iter()
            .rev()
            .map(|g| g.position().x as f32 + g.unpositioned().h_metrics().advance_width)
            .next()
            .unwrap_or(0.0)
            .ceil() as usize;

        for g in glyphs {
            if let Some(bb) = g.pixel_bounding_box() {
                g.draw(|x, y, v| {
                    let x = x as i32 + bb.min.x;
                    let y = y as i32 + bb.min.y;
                    let alpha = ((1.0 - v) * 255.0) as u8;
                    if x >= 0 && (x as u32) < self.width && y >= 0 && (y as u32) < self.height {
                        self.draw_at(
                            x as u32 + base_x,
                            y as u32 + base_y,
                            Color {r: alpha, g: alpha, b: alpha, a: alpha},
                        );
                    }
                });
            }
        }
    }

    pub fn draw_rect(&mut self, rect: layout::Rect, color: Color) {
        let mut paint = Paint::default();
        
        #[cfg(target_arch = "riscv64")]
        paint.set_color(tiny_skia::Color::from_rgba8(color.b, color.g, color.r, color.a));
        #[cfg(target_arch = "x86_64")]
        paint.set_color(tiny_skia::Color::from_rgba8(color.r, color.g, color.b, color.a));
        paint.anti_alias = true;
        let path = PathBuilder::from_rect(
            tiny_skia::Rect::from_xywh(rect.x, rect.y, rect.width, rect.height).unwrap(),
        );
        self.pixmap.fill_path(
            &path,
            &paint,
            FillRule::Winding,
            Transform::identity(),
            None,
        );
    }

    fn draw_background(&mut self, layout_box: &LayoutBox<'_>) {
        let color = layout_box.get_style_node().value("background").unwrap_or(Value::ColorValue(Color {
            r: 0xff,
            g: 0xff,
            b: 0xff,
            a: 0xff,
        }));
        self.draw_rect(layout_box.dimensions.margin_box(), value_to_color(color));
    }

    fn draw_layout_box(&mut self, layout_box: &LayoutBox<'_>) {
        self.draw_background(layout_box);

        match layout_box.box_type {
            BoxType::BlockNode(style_node) => {
                match style_node.node.node_type {
                    NodeType::Text(ref s) => {
                        let x = layout_box.dimensions.content.x.ceil() as u32;
                        let y = layout_box.dimensions.content.y.ceil() as u32;
                        let px = style_node.value("font-size").unwrap_or(Value::Length(DEFAULT_FONT_SIZE, Unit::Px));
                        self.draw_text(s.to_string(), value_to_px(px), x, y);
                    }
                    _ => {}
                }
            }
            _ => unimplemented!()
        }

        for child in layout_box.children.iter() {
            self.draw_layout_box(child);
        }
    }

    pub fn render(&mut self, layout_root: &LayoutBox<'_>) {
        self.draw_layout_box(layout_root);
    }
}

fn value_to_color(val: Value) -> Color {
    match val {
        Value::ColorValue(color) => color,
        _ => panic!("not a color")
    }
}

fn value_to_px(val: Value) -> f32 {
    match val {
        Value::Length(v, unit) => {
            match unit {
                Unit::Px => {
                    v
                }
                // _ => unimplemented!()
            }
        }
        _ => panic!("not a length")
    }
}

