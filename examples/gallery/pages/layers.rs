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
const PERSPECTIVE: f32 = 1000.;
const TRANSITION: Duration = Duration::from_millis(500);
const CELL: f32 = 3.;
const COLS: usize = (W / CELL) as usize;
const ROWS: usize = (H / CELL) as usize + 1;

struct LayerSpec {
    name: &'static str,
    title: &'static str,
    image: Option<&'static str>,
    z: f32,
}

const LAYERS: [LayerSpec; 4] = [
    LayerSpec {
        name: "content",
        title: "Content",
        image: Some("layers_content.png"),
        z: 120. - 3. * 80.,
    },
    LayerSpec {
        name: "navigation",
        title: "Navigation",
        image: Some("layers_navigation.png"),
        z: 120. - 2. * 80.,
    },
    LayerSpec {
        name: "mask",
        title: "Mask",
        image: None,
        z: 120. - 80.,
    },
    LayerSpec {
        name: "popout",
        title: "Popout",
        image: Some("layers_popout.png"),
        z: 120.,
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
    origin: (f32, f32),
}

impl Projector {
    fn project(&self, x: f32, y: f32) -> Point<Pixels> {
        let a = self.a;
        let (mut px_, mut py_, mut pz) = (x - W / 2., y - H / 2., self.z * a);
        px_ += (-15f32 * a).to_radians().tan() * py_;
        let rz = (10. * a).to_radians();
        let (s, c) = rz.sin_cos();
        let (nx, ny) = (px_ * c - py_ * s, px_ * s + py_ * c);
        px_ = nx;
        py_ = ny;
        let rx = (45. * a).to_radians();
        let (s, c) = rx.sin_cos();
        let (ny, nz) = (py_ * c - pz * s, py_ * s + pz * c);
        py_ = ny;
        pz = nz;
        px_ += 15. * a;
        let wx = self.center.0 + px_;
        let wy = self.center.1 + py_;
        let f = PERSPECTIVE / (PERSPECTIVE - pz);
        point(
            px(self.origin.0 + (wx - self.origin.0) * f),
            px(self.origin.1 + (wy - self.origin.1) * f),
        )
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
}

impl Render for LayersPage {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let t = Theme::get(cx).clone();
        let reduce = cx.reduce_motion();
        self.load_cells(window, cx);
        let viewport = window.viewport_size();
        let origin = (viewport.width.as_f32() / 2., viewport.height.as_f32() / 2.);
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
                z: spec.z,
                opacity,
                bg,
                border: (spec.name == "popout" && !layer.pic).then(|| Hsla::from(rgba(0xcbcbcb80))),
                cells: if layer.pic {
                    self.cells[i].clone()
                } else {
                    None
                },
                label: (spec.title, label_color.opacity(label)),
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
                    bounds.origin.y.as_f32() + H / 2.,
                );
                let mut quads = quads.borrow_mut();
                for (i, layer) in paint.iter().enumerate() {
                    let p = Projector {
                        a: layer.a,
                        z: layer.z,
                        center,
                        origin,
                    };
                    let outline = [
                        p.project(0., 0.),
                        p.project(W, 0.),
                        p.project(W, H),
                        p.project(0., H),
                    ];
                    quads[i] = Some(outline);
                    if layer.opacity <= 0.01 {
                        continue;
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
                        let rect = Bounds::new(outline[0], size(px(W), px(H)));
                        let isz = image.size(0);
                        let scale = (W / isz.width.0 as f32).min(H / isz.height.0 as f32);
                        let (dw, dh) = (isz.width.0 as f32 * scale, isz.height.0 as f32 * scale);
                        let dest = Bounds::new(
                            point(
                                rect.origin.x + px((W - dw) / 2.),
                                rect.origin.y + px((H - dh) / 2.),
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
                                grid.push(
                                    p.project((c as f32 * CELL).min(W), (r as f32 * CELL).min(H)),
                                );
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
                            [(0., 0.), (W, 0.), (W, 1.), (0., 1.)],
                            [(0., H - 1.), (W, H - 1.), (W, H), (0., H)],
                            [(0., 0.), (1., 0.), (1., H), (0., H)],
                            [(W - 1., 0.), (W, 0.), (W, H), (W - 1., H)],
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
                        let anchor = p.project(W / 2., H - 5. - 14. * 1.6 / 2.);
                        let at = point(anchor.x - line.width / 2., anchor.y - px(14. * 1.6 / 2.));
                        line.paint(at, px(14. * 1.6), TextAlign::Left, None, window, cx)
                            .ok();
                    }
                }
            },
        )
        .absolute()
        .top_0()
        .left_0()
        .w_full()
        .h(px(H));
        let (_, title, desc) = INFO[self.info];
        let header = div()
            .p(px(40.))
            .child(
                div()
                    .text_size(px(20.))
                    .line_height(relative(1.6))
                    .child(title),
            )
            .child(
                div()
                    .mt(px(4.))
                    .min_h(px(14. * 1.6 * 3.))
                    .text_size(px(14.))
                    .line_height(relative(1.6))
                    .text_color(t.fg_1)
                    .child(desc),
            );
        let buttons = (0..4).rev().map(|i| {
            div()
                .id(("layer", i))
                .role(Role::Button)
                .aria_label(LAYERS[i].title)
                .track_focus(&self.focus[i])
                .tab_stop(true)
                .absolute()
                .top_0()
                .left(px(-W / 2.))
                .w(px(W))
                .h(px(H))
                .border_2()
                .border_color(transparent_black())
                .focus_visible(|s| s.border_color(t.brand))
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
                    .h(px(H + 240.))
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
                    .h(px(H))
                    .children(buttons),
            );
        PageFrame::new("layers").header(header).child(stage)
    }
}
