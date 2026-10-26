use crate::text::tr;
use crate::{chrome::PageFrame, pages::Args};
use gpui::{prelude::*, *};
use std::{
    cell::RefCell,
    rc::Rc,
    time::{Duration, Instant},
};
use wegpui::{Theme, components::modal::ease};

const W: f32 = 180.;
const H: f32 = 320.;
const TRANSITION: Duration = Duration::from_millis(500);
const CELL: f32 = 3.;
const COLS: usize = (W / CELL) as usize;
const ROWS: usize = (H / CELL) as usize + 1;

struct LayerSpec {
    name: &'static str,
    title: &'static str,
    image: Option<&'static str>,
    depth: usize,
}

const LAYERS: [LayerSpec; 4] = [
    LayerSpec {
        name: "content",
        title: "Content",
        image: Some("layers_content.png"),
        depth: 3,
    },
    LayerSpec {
        name: "navigation",
        title: "Navigation",
        image: Some("layers_navigation.png"),
        depth: 2,
    },
    LayerSpec {
        name: "mask",
        title: "Mask",
        image: None,
        depth: 1,
    },
    LayerSpec {
        name: "popout",
        title: "Popout",
        image: Some("layers_popout.png"),
        depth: 0,
    },
];

const INFO: [(&str, &str, &str); 5] = [
    (
        "normal",
        "WeUI页面层级",
        "用于规范WeUI页面元素所属层级、层级顺序及组合规范。",
    ),
    (
        "popout",
        "Popout",
        "弹出层，作为内容层和导航层的补充，用于承载弹窗通知、操作菜单、菜单、成功或加载中等状态的Toast，表单报错提示等弹出内容。",
    ),
    (
        "mask",
        "Mask",
        "蒙层，配合Popout层使用，用于锁定内容层和导航层操作，不单独使用。",
    ),
    (
        "navigation",
        "Navigation",
        "导航层，位于内容层之上，在用户滑动内容层时可保持位置不动，通常用于承载导航栏等需要固定在页面的元素。",
    ),
    ("content", "Content", "内容层，承载页面主要内容。"),
];

#[derive(Clone, Copy)]
struct Tween {
    from: f32,
    to: f32,
    start: Option<Instant>,
}

impl Tween {
    fn new(v: f32) -> Self {
        Self {
            from: v,
            to: v,
            start: None,
        }
    }
    fn value(&self, reduce: bool) -> (f32, bool) {
        match self.start {
            Some(at) if !reduce => {
                let p = (at.elapsed().as_secs_f32() / TRANSITION.as_secs_f32()).min(1.);
                (self.from + (self.to - self.from) * ease(p), p < 1.)
            }
            _ => (self.to, false),
        }
    }
    fn go(&mut self, to: f32, reduce: bool) {
        self.from = self.value(reduce).0;
        self.to = to;
        self.start = Some(Instant::now());
    }
}

struct Layer {
    transform: Tween,
    opacity: Tween,
    label: Tween,
    pic: bool,
    generation: usize,
}

type Quad = [Point<Pixels>; 4];
type LayerCells = (Rc<Vec<Hsla>>, std::sync::Arc<RenderImage>);

pub struct LayersPage {
    layers: Vec<Layer>,
    info: usize,
    hide_generation: usize,
    cells: Vec<Option<LayerCells>>,
    quads: Rc<RefCell<Vec<Option<Quad>>>>,
    focus: Vec<FocusHandle>,
}

pub fn build(_: &'static str, args: &Args, window: &mut Window, cx: &mut App) -> AnyView {
    let state = args.state.clone();
    cx.new(|cx| LayersPage::new(state.as_deref(), window, cx))
        .into()
}

impl LayersPage {
    fn new(state: Option<&str>, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let mut this = Self {
            layers: (0..4)
                .map(|_| Layer {
                    transform: Tween::new(0.),
                    opacity: Tween::new(1.),
                    label: Tween::new(0.),
                    pic: true,
                    generation: 0,
                })
                .collect(),
            info: 0,
            hide_generation: 0,
            cells: vec![None, None, None, None],
            quads: Rc::new(RefCell::new(vec![None; 4])),
            focus: (0..4).map(|_| cx.focus_handle().tab_stop(true)).collect(),
        };
        match state {
            Some("3d") => {
                for layer in &mut this.layers {
                    layer.transform = Tween::new(1.);
                    layer.label = Tween::new(1.);
                    layer.pic = false;
                }
            }
            Some(name) if LAYERS.iter().any(|l| l.name == name) => {
                for (i, layer) in this.layers.iter_mut().enumerate() {
                    let target = LAYERS[i].name == name;
                    layer.transform = Tween::new(if target { 0. } else { 1. });
                    layer.opacity = Tween::new(if target { 1. } else { 0. });
                    layer.pic = target;
                }
                this.info = INFO.iter().position(|i| i.0 == name).unwrap_or(0);
            }
            _ => {
                cx.spawn_in(window, async move |this, cx| {
                    cx.background_executor()
                        .timer(Duration::from_millis(500))
                        .await;
                    this.update_in(cx, |this, window, cx| {
                        for i in 0..4 {
                            this.transform(i, 1., window, cx);
                        }
                    })
                    .ok();
                })
                .detach();
            }
        }
        this
    }

    fn transform(&mut self, i: usize, to: f32, window: &mut Window, cx: &mut Context<Self>) {
        let reduce = cx.reduce_motion();
        let layer = &mut self.layers[i];
        layer.transform.go(to, reduce);
        layer.generation += 1;
        let generation = layer.generation;
        let delay = if reduce { Duration::ZERO } else { TRANSITION };
        cx.spawn_in(window, async move |this, cx| {
            cx.background_executor().timer(delay).await;
            this.update_in(cx, |this, window, cx| {
                if this.layers[i].generation != generation {
                    return;
                }
                if to >= 1. {
                    cx.spawn_in(window, async move |this, cx| {
                        cx.background_executor()
                            .timer(Duration::from_millis(500))
                            .await;
                        this.update(cx, |this, cx| {
                            let layer = &mut this.layers[i];
                            if layer.generation == generation {
                                layer.pic = false;
                                layer.label.go(1., cx.reduce_motion());
                                cx.notify();
                            }
                        })
                        .ok();
                    })
                    .detach();
                } else {
                    let layer = &mut this.layers[i];
                    layer.pic = true;
                    layer.label = Tween::new(0.);
                    cx.notify();
                }
            })
            .ok();
        })
        .detach();
        cx.notify();
    }

    fn activate(&mut self, i: usize, window: &mut Window, cx: &mut Context<Self>) {
        if self.layers[i].opacity.to <= 0. {
            return;
        }
        let reduce = cx.reduce_motion();
        if self.layers.iter().any(|l| l.transform.to < 1.) {
            self.info = 0;
            for j in 0..4 {
                if self.layers[j].transform.to < 1. {
                    self.transform(j, 1., window, cx);
                }
            }
            self.hide_generation += 1;
            let generation = self.hide_generation;
            cx.spawn_in(window, async move |this, cx| {
                cx.background_executor()
                    .timer(Duration::from_millis(300))
                    .await;
                this.update(cx, |this, cx| {
                    if this.hide_generation == generation {
                        let reduce = cx.reduce_motion();
                        for layer in &mut this.layers {
                            if layer.opacity.to < 1. {
                                layer.opacity.go(1., reduce);
                            }
                        }
                        cx.notify();
                    }
                })
                .ok();
            })
            .detach();
            return;
        }
        self.hide_generation += 1;
        self.info = INFO.iter().position(|x| x.0 == LAYERS[i].name).unwrap_or(0);
        for j in 0..4 {
            if j != i {
                self.layers[j].opacity.go(0., reduce);
            }
        }
        self.transform(i, 0., window, cx);
        window.focus(&self.focus[i], cx);
    }

    fn hit(&self, position: Point<Pixels>) -> Option<usize> {
        let quads = self.quads.borrow();
        (0..4).rev().find(|&i| {
            self.layers[i].opacity.to > 0. && quads[i].is_some_and(|q| inside(&q, position))
        })
    }

    fn load_cells(&mut self, window: &mut Window, cx: &mut App) {
        for (i, spec) in LAYERS.iter().enumerate() {
            let Some(name) = spec.image else { continue };
            if self.cells[i].is_some() {
                continue;
            }
            let resource = Resource::Embedded(SharedString::from(name));
            if let Some(Ok(image)) = window.use_asset::<ImgResourceLoader>(&resource, cx) {
                self.cells[i] = Some((Rc::new(average_cells(&image)), image));
            }
        }
    }
}

fn average_cells(image: &RenderImage) -> Vec<Hsla> {
    let size = image.size(0);
    let (iw, ih) = (size.width.0 as usize, size.height.0 as usize);
    let Some(bytes) = image.as_bytes(0) else {
        return vec![];
    };
    let scale = (W / iw as f32).min(H / ih as f32);
    let (dw, dh) = (iw as f32 * scale, ih as f32 * scale);
    let (ox, oy) = ((W - dw) / 2., (H - dh) / 2.);
    let mut out = Vec::with_capacity(COLS * ROWS);
    for r in 0..ROWS {
        for c in 0..COLS {
            let x0 = ((c as f32 * CELL - ox) / scale).max(0.) as usize;
            let x1 = ((((c + 1) as f32 * CELL - ox) / scale) as usize).min(iw);
            let y0 = ((r as f32 * CELL - oy) / scale).max(0.) as usize;
            let y1 = ((((r + 1) as f32 * CELL - oy) / scale) as usize).min(ih);
            let (mut sr, mut sg, mut sb, mut sa, mut n) = (0f32, 0f32, 0f32, 0f32, 0f32);
            let mut y = y0;
            while y < y1 {
                let mut x = x0;
                while x < x1 {
                    let p = (y * iw + x) * 4;
                    let a = bytes[p + 3] as f32 / 255.;
                    sb += bytes[p] as f32 / 255. * a;
                    sg += bytes[p + 1] as f32 / 255. * a;
                    sr += bytes[p + 2] as f32 / 255. * a;
                    sa += a;
                    n += 1.;
                    x += 2;
                }
                y += 2;
            }
            out.push(if n == 0. || sa / n < 0.01 {
                transparent_black()
            } else {
                Rgba {
                    r: sr / sa,
                    g: sg / sa,
                    b: sb / sa,
                    a: sa / n,
                }
                .into()
            });
        }
    }
    out
}

fn inside(q: &Quad, p: Point<Pixels>) -> bool {
    let mut sign = 0.;
    for k in 0..4 {
        let a = q[k];
        let b = q[(k + 1) % 4];
        let cross = (b.x - a.x).as_f32() * (p.y - a.y).as_f32()
            - (b.y - a.y).as_f32() * (p.x - a.x).as_f32();
        if cross != 0. {
            if sign == 0. {
                sign = cross.signum();
            } else if cross.signum() != sign {
                return false;
            }
        }
    }
    true
}

#[derive(Clone, Copy)]
struct Projector {
    a: f32,
    z: f32,
    center: (f32, f32),
    width: f32,
    height: f32,
}

impl Projector {
    fn project(&self, x: f32, y: f32) -> Point<Pixels> {
        let a = self.a;
        let (mut px_, mut py_, pz) = (x - self.width / 2., y - self.height / 2., self.z * a);
        px_ += (-15f32 * a).to_radians().tan() * py_;
        let rz = (10. * a).to_radians();
        let (s, c) = rz.sin_cos();
        let (nx, ny) = (px_ * c - py_ * s, px_ * s + py_ * c);
        px_ = nx;
        py_ = ny;
        let rx = (45. * a).to_radians();
        let (s, c) = rx.sin_cos();
        let ny = py_ * c - pz * s;
        py_ = ny;
        px_ += 15. * a;
        let wx = self.center.0 + px_;
        let wy = self.center.1 + py_;
        point(px(wx), px(wy))
    }

    fn transformation(&self, scale: f32) -> TransformationMatrix {
        let origin = self.project(0., 0.);
        let x = self.project(1., 0.) - origin;
        let y = self.project(0., 1.) - origin;
        TransformationMatrix {
            rotation_scale: [[x.x.as_f32(), y.x.as_f32()], [x.y.as_f32(), y.y.as_f32()]],
            translation: [origin.x.as_f32() * scale, origin.y.as_f32() * scale],
        }
    }
}

fn fill(window: &mut Window, q: &Quad, color: Hsla) {
    if color.a <= 0. {
        return;
    }
    let mut path = Path::new(q[0]);
    path.line_to(q[1]);
    path.line_to(q[2]);
    path.line_to(q[3]);
    window.paint_path(path, color);
}

struct PaintLayer {
    a: f32,
    z: f32,
    opacity: f32,
    bg: Hsla,
    border: Option<Hsla>,
    cells: Option<(Rc<Vec<Hsla>>, std::sync::Arc<RenderImage>)>,
    label: (&'static str, Hsla),
    focused: bool,
}

fn layer_size(viewport: Size<Pixels>) -> (f32, f32) {
    if viewport.width >= px(414.) && viewport.height >= px(640.) {
        (200., 355.)
    } else if viewport.width >= px(375.) && viewport.height >= px(603.) {
        (180., 320.)
    } else {
        (150., 266.)
    }
}

impl Render for LayersPage {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let t = Theme::get(cx).clone();
        let reduce = cx.reduce_motion();
        self.load_cells(window, cx);
        let viewport = window.viewport_size();
        let (layer_width, layer_height) = layer_size(viewport);
        let compact = viewport.width <= px(320.);
        let mut animating = false;
        let mut paint = vec![];
        for (i, spec) in LAYERS.iter().enumerate() {
            let layer = &self.layers[i];
            let (a, m1) = layer.transform.value(reduce);
            let (opacity, m2) = layer.opacity.value(reduce);
            let (label, m3) = layer.label.value(reduce);
            animating |= m1 || m2 || m3;
            let bg: Hsla = match spec.name {
                "popout" => transparent_black(),
                "mask" => hsla(0., 0., 0., 0.5),
                "navigation" if !layer.pic => rgba(0x28bb6680).into(),
                "navigation" => transparent_black(),
                _ => t.bg_2,
            };
            let label_color: Hsla = if spec.name == "content" {
                rgb(0xaaaaaa).into()
            } else {
                t.white
            };
            paint.push(PaintLayer {
                a,
                z: if compact {
                    140. - spec.depth as f32 * 60.
                } else {
                    120. - spec.depth as f32 * 80.
                },
                opacity,
                bg,
                border: (spec.name == "popout" && !layer.pic).then(|| Hsla::from(rgba(0xcbcbcb80))),
                cells: if layer.pic {
                    self.cells[i].clone()
                } else {
                    None
                },
                label: (spec.title, label_color.opacity(label)),
                focused: self.focus[i].is_focused(window) && window.last_input_was_keyboard(),
            });
        }
        if animating {
            window.request_animation_frame();
        }
        let quads = self.quads.clone();
        let canvas_el = canvas(
            move |_, _, _| {},
            move |bounds, _, window, cx| {
                let center = (
                    bounds.origin.x.as_f32() + bounds.size.width.as_f32() / 2.,
                    bounds.origin.y.as_f32() + layer_height / 2.,
                );
                let mut quads = quads.borrow_mut();
                let mut focus = None;
                for (i, layer) in paint.iter().enumerate() {
                    let p = Projector {
                        a: layer.a,
                        z: layer.z,
                        center,
                        width: layer_width,
                        height: layer_height,
                    };
                    let outline = [
                        p.project(0., 0.),
                        p.project(layer_width, 0.),
                        p.project(layer_width, layer_height),
                        p.project(0., layer_height),
                    ];
                    quads[i] = Some(outline);
                    if layer.opacity <= 0.01 {
                        continue;
                    }
                    if layer.focused {
                        focus = Some(p);
                    }
                    let alpha = |c: Hsla| {
                        let mut c = c;
                        c.a *= layer.opacity;
                        c
                    };
                    fill(window, &outline, alpha(layer.bg));
                    if let Some((_, image)) = layer
                        .cells
                        .as_ref()
                        .filter(|_| layer.a < 0.001 && layer.opacity > 0.99)
                    {
                        let rect = Bounds::new(outline[0], size(px(layer_width), px(layer_height)));
                        let isz = image.size(0);
                        let scale = (layer_width / isz.width.0 as f32)
                            .min(layer_height / isz.height.0 as f32);
                        let (dw, dh) = (isz.width.0 as f32 * scale, isz.height.0 as f32 * scale);
                        let dest = Bounds::new(
                            point(
                                rect.origin.x + px((layer_width - dw) / 2.),
                                rect.origin.y + px((layer_height - dh) / 2.),
                            ),
                            size(px(dw), px(dh)),
                        );
                        window
                            .paint_image(dest, dest, Corners::default(), image.clone(), 0, false)
                            .ok();
                    } else if let Some((cells, _)) = layer.cells.as_ref() {
                        let mut grid = Vec::with_capacity((COLS + 1) * (ROWS + 1));
                        for r in 0..=ROWS {
                            for c in 0..=COLS {
                                grid.push(p.project(
                                    (c as f32 * CELL).min(W) * layer_width / W,
                                    (r as f32 * CELL).min(H) * layer_height / H,
                                ));
                            }
                        }
                        let at = |c: usize, r: usize| grid[r * (COLS + 1) + c];
                        for r in 0..ROWS {
                            for c in 0..COLS {
                                let color = cells[r * COLS + c];
                                if color.a <= 0. {
                                    continue;
                                }
                                let q = [at(c, r), at(c + 1, r), at(c + 1, r + 1), at(c, r + 1)];
                                fill(window, &q, alpha(color));
                            }
                        }
                    }
                    if let Some(border) = layer.border {
                        let b = alpha(border);
                        let edges = [
                            [(0., 0.), (layer_width, 0.), (layer_width, 1.), (0., 1.)],
                            [
                                (0., layer_height - 1.),
                                (layer_width, layer_height - 1.),
                                (layer_width, layer_height),
                                (0., layer_height),
                            ],
                            [(0., 0.), (1., 0.), (1., layer_height), (0., layer_height)],
                            [
                                (layer_width - 1., 0.),
                                (layer_width, 0.),
                                (layer_width, layer_height),
                                (layer_width - 1., layer_height),
                            ],
                        ];
                        for e in edges {
                            let q = e.map(|(x, y)| p.project(x, y));
                            fill(window, &q, b);
                        }
                    }
                    let (text, color) = layer.label;
                    let color = alpha(color);
                    if color.a > 0.01 {
                        let style = window.text_style();
                        let run = TextRun {
                            color,
                            ..style.to_run(text.len())
                        };
                        let line =
                            window
                                .text_system()
                                .shape_line(text.into(), px(14.), &[run], None);
                        let baseline = layer_height - 5.
                            - (14. * 1.6 - line.ascent.as_f32() - line.descent.as_f32()) / 2.
                            - line.descent.as_f32();
                        let family = if style.font_family.starts_with('.') {
                            "system-ui"
                        } else {
                            style.font_family.as_ref()
                        };
                        let family = family.replace('&', "&amp;").replace('"', "&quot;")
                            .replace('<', "&lt;").replace('>', "&gt;");
                        let data = format!(
                            "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{layer_width}\" height=\"{layer_height}\"><text x=\"{}\" y=\"{baseline}\" font-family=\"{family}\" font-size=\"14\" text-anchor=\"middle\">{text}</text></svg>",
                            layer_width / 2.
                        );
                        let path: SharedString = format!("layers-label:{layer_width}:{layer_height}:{family}:{text}").into();
                        window.paint_svg(
                            Bounds::new(point(px(0.), px(0.)), size(px(layer_width), px(layer_height))),
                            path, Some(data.as_bytes()), p.transformation(window.scale_factor()), color, cx,
                        ).ok();
                    }
                }
                if let Some(projector) = focus {
                    for (inset, color) in [(4., black()), (2., white())] {
                        for edge in [
                            [(0., 0.), (layer_width, 0.), (layer_width, inset), (0., inset)],
                            [(0., layer_height - inset), (layer_width, layer_height - inset), (layer_width, layer_height), (0., layer_height)],
                            [(0., 0.), (inset, 0.), (inset, layer_height), (0., layer_height)],
                            [(layer_width - inset, 0.), (layer_width, 0.), (layer_width, layer_height), (layer_width - inset, layer_height)],
                        ] {
                            fill(window, &edge.map(|(x, y)| projector.project(x, y)), color);
                        }
                    }
                }
            },
        )
        .absolute()
        .top_0()
        .left_0()
        .w_full()
        .h(px(layer_height));
        let (_, title, desc) = INFO[self.info];
        let (title, desc) = (tr(title), tr(desc));
        let header = div()
            .py(px(40.))
            .px(px(if compact { 20. } else { 40. }))
            .child(
                div()
                    .id("layers-title")
                    .role(Role::Heading)
                    .aria_level(2)
                    .aria_label(title)
                    .text_size(px(20.))
                    .line_height(relative(1.6))
                    .child(title),
            )
            .child(
                div()
                    .id("layers-description")
                    .role(Role::Paragraph)
                    .aria_label(desc)
                    .mt(px(4.))
                    .min_h(px(14. * 1.6 * 3.))
                    .text_size(px(14.))
                    .line_height(relative(1.6))
                    .text_color(t.fg_1)
                    .child(desc),
            );
        let visible_layers: Vec<_> = (0..4)
            .rev()
            .filter(|&i| self.layers[i].opacity.to > 0.)
            .collect();
        let buttons = visible_layers.into_iter().map(|i| {
            let weak = cx.entity().downgrade();
            div()
                .id(("layer", i))
                .role(Role::Button)
                .aria_label(LAYERS[i].title)
                .track_focus(&self.focus[i])
                .tab_stop(true)
                .absolute()
                .top_0()
                .left(px(-layer_width / 2.))
                .w(px(layer_width))
                .h(px(layer_height))
                .border_2()
                .border_color(transparent_black())
                .on_a11y_action(AccessibleAction::Click, move |_, window, cx| {
                    weak.update(cx, |this, cx| this.activate(i, window, cx))
                        .ok();
                })
                .on_click(cx.listener(move |this, e: &ClickEvent, window, cx| {
                    if e.is_keyboard() {
                        this.activate(i, window, cx);
                    }
                }))
        });
        let stage = div()
            .relative()
            .w_full()
            .h(px(0.))
            .child(canvas_el)
            .child(
                div()
                    .id("layers-hit")
                    .absolute()
                    .top(px(-120.))
                    .left_0()
                    .w_full()
                    .h(px(layer_height + 240.))
                    .on_click(cx.listener(|this, e: &ClickEvent, window, cx| {
                        if e.is_keyboard() {
                            return;
                        }
                        if let Some(i) = this.hit(e.position()) {
                            this.activate(i, window, cx);
                        }
                    })),
            )
            .child(
                div()
                    .absolute()
                    .top_0()
                    .left(relative(0.5))
                    .w(px(0.))
                    .h(px(layer_height))
                    .children(buttons),
            );
        PageFrame::new("layers").header(header).child(stage)
    }
}

#[cfg(test)]
mod tests {
    use super::{Projector, layer_size, px, size};

    #[test]
    fn projection_matches_upstream_browser_quads() {
        let cases = [
            (
                375.,
                812.,
                -120.,
                [
                    183.87157, 310.83527, 361.13696, 332.93707, 221.12843, 545.24536, 43.863045,
                    523.14355,
                ],
            ),
            (
                320.,
                812.,
                -40.,
                [
                    159.33046, 247.02206, 307.05164, 265.44025, 190.66954, 441.92148, 42.94838,
                    423.5033,
                ],
            ),
            (
                480.,
                812.,
                -120.,
                [
                    234.1802, 315.4968, 431.14175, 340.05435, 275.8198, 575.58386, 78.85825,
                    551.02625,
                ],
            ),
            (
                375.,
                480.,
                -120.,
                [
                    186.83046, 303.5906, 334.55164, 322.0088, 218.16954, 498.49002, 70.44838,
                    480.07187,
                ],
            ),
        ];
        for (viewport_width, viewport_height, z, expected) in cases {
            let (width, height) = layer_size(size(px(viewport_width), px(viewport_height)));
            let projector = Projector {
                a: 1.,
                z,
                width,
                height,
                center: (viewport_width / 2., 183.1875 + height / 2.),
            };
            for ((x, y), pair) in [(0., 0.), (width, 0.), (width, height), (0., height)]
                .into_iter()
                .zip(expected.chunks_exact(2))
            {
                let point = projector.project(x, y);
                assert!((point.x.as_f32() - pair[0]).abs() < 0.001);
                assert!((point.y.as_f32() - pair[1]).abs() < 0.001);
            }
        }
    }

    #[test]
    fn layer_size_follows_upstream_media_queries() {
        for (width, height, expected) in [
            (374., 812., (150., 266.)),
            (375., 602., (150., 266.)),
            (375., 603., (180., 320.)),
            (413., 812., (180., 320.)),
            (414., 639., (180., 320.)),
            (414., 640., (200., 355.)),
        ] {
            assert_eq!(layer_size(size(px(width), px(height))), expected);
        }
    }

    #[test]
    fn label_transform_tracks_plane_points_at_both_scales() {
        for progress in [0., 0.37, 1.] {
            for scale in [1., 2.] {
                let projector = Projector {
                    a: progress,
                    z: 120.,
                    width: 200.,
                    height: 355.,
                    center: (240., 360.6875),
                };
                let matrix = projector.transformation(scale);
                for (x, y) in [(0., 0.), (200., 355.), (100., 338.8)] {
                    let transformed = matrix.apply(super::point(px(x * scale), px(y * scale)));
                    let expected = projector.project(x, y);
                    assert!((transformed.x.as_f32() - expected.x.as_f32() * scale).abs() < 0.02);
                    assert!((transformed.y.as_f32() - expected.y.as_f32() * scale).abs() < 0.02);
                }
            }
        }
    }
}
