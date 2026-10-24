use gpui::{App, Global, Hsla, Window, WindowAppearance, rgb};

#[derive(Clone, Debug)]
pub struct Theme {
    pub is_dark: bool,
    pub mode: ThemeMode,
    pub btn_height: f32,
    pub btn_height_medium: f32,
    pub btn_height_small: f32,
    pub btn_active_mask: Hsla,
    pub btn_default_active_bg: Hsla,
    pub dialog_line_color: Hsla,
    pub bg_0: Hsla,
    pub bg_1: Hsla,
    pub bg_2: Hsla,
    pub bg_3: Hsla,
    pub bg_4: Hsla,
    pub bg_5: Hsla,
    pub blue_100: Hsla,
    pub blue_120: Hsla,
    pub blue_170: Hsla,
    pub blue_80: Hsla,
    pub blue_90: Hsla,
    pub blue_bg_100: Hsla,
    pub blue_bg_110: Hsla,
    pub blue_bg_130: Hsla,
    pub blue_bg_90: Hsla,
    pub brand_100: Hsla,
    pub brand_120: Hsla,
    pub brand_170: Hsla,
    pub brand_80: Hsla,
    pub brand_90: Hsla,
    pub brand_bg_100: Hsla,
    pub brand_bg_110: Hsla,
    pub brand_bg_130: Hsla,
    pub brand_bg_90: Hsla,
    pub fg_0: Hsla,
    pub fg_0_5: Hsla,
    pub fg_1: Hsla,
    pub fg_2: Hsla,
    pub fg_3: Hsla,
    pub fg_4: Hsla,
    pub glyph_0: Hsla,
    pub glyph_1: Hsla,
    pub glyph_2: Hsla,
    pub glyph_white_0: Hsla,
    pub glyph_white_1: Hsla,
    pub glyph_white_2: Hsla,
    pub glyph_white_3: Hsla,
    pub green_100: Hsla,
    pub green_120: Hsla,
    pub green_170: Hsla,
    pub green_80: Hsla,
    pub green_90: Hsla,
    pub green_bg_100: Hsla,
    pub green_bg_110: Hsla,
    pub green_bg_130: Hsla,
    pub green_bg_90: Hsla,
    pub indigo_100: Hsla,
    pub indigo_120: Hsla,
    pub indigo_170: Hsla,
    pub indigo_80: Hsla,
    pub indigo_90: Hsla,
    pub indigo_bg_100: Hsla,
    pub indigo_bg_110: Hsla,
    pub indigo_bg_130: Hsla,
    pub indigo_bg_90: Hsla,
    pub lightgreen_100: Hsla,
    pub lightgreen_120: Hsla,
    pub lightgreen_170: Hsla,
    pub lightgreen_80: Hsla,
    pub lightgreen_90: Hsla,
    pub lightgreen_bg_100: Hsla,
    pub lightgreen_bg_110: Hsla,
    pub lightgreen_bg_130: Hsla,
    pub lightgreen_bg_90: Hsla,
    pub link_100: Hsla,
    pub link_120: Hsla,
    pub link_170: Hsla,
    pub link_80: Hsla,
    pub link_90: Hsla,
    pub linkfinder_100: Hsla,
    pub material_attachmentcolumn: Hsla,
    pub material_navigationbar: Hsla,
    pub material_regular: Hsla,
    pub material_thick: Hsla,
    pub material_thin: Hsla,
    pub material_toolbar: Hsla,
    pub orange_100: Hsla,
    pub orange_120: Hsla,
    pub orange_170: Hsla,
    pub orange_80: Hsla,
    pub orange_90: Hsla,
    pub orange_bg_100: Hsla,
    pub orange_bg_110: Hsla,
    pub orange_bg_130: Hsla,
    pub orange_bg_90: Hsla,
    pub orangered_100: Hsla,
    pub overlay: Hsla,
    pub overlay_white: Hsla,
    pub purple_100: Hsla,
    pub purple_120: Hsla,
    pub purple_170: Hsla,
    pub purple_80: Hsla,
    pub purple_90: Hsla,
    pub purple_bg_100: Hsla,
    pub purple_bg_110: Hsla,
    pub purple_bg_130: Hsla,
    pub purple_bg_90: Hsla,
    pub red_100: Hsla,
    pub red_120: Hsla,
    pub red_170: Hsla,
    pub red_80: Hsla,
    pub red_90: Hsla,
    pub red_bg_100: Hsla,
    pub red_bg_110: Hsla,
    pub red_bg_130: Hsla,
    pub red_bg_90: Hsla,
    pub secondary_bg: Hsla,
    pub separator_0: Hsla,
    pub separator_1: Hsla,
    pub statelayer_hovered: Hsla,
    pub statelayer_pressed: Hsla,
    pub statelayer_pressedstrengthened: Hsla,
    pub yellow_100: Hsla,
    pub yellow_120: Hsla,
    pub yellow_170: Hsla,
    pub yellow_80: Hsla,
    pub yellow_90: Hsla,
    pub yellow_bg_100: Hsla,
    pub yellow_bg_110: Hsla,
    pub yellow_bg_130: Hsla,
    pub yellow_bg_90: Hsla,
    pub fg_half: Hsla,
    pub red: Hsla,
    pub orangered: Hsla,
    pub orange: Hsla,
    pub yellow: Hsla,
    pub green: Hsla,
    pub lightgreen: Hsla,
    pub textgreen: Hsla,
    pub brand: Hsla,
    pub blue: Hsla,
    pub indigo: Hsla,
    pub purple: Hsla,
    pub link: Hsla,
    pub tag_text_orange: Hsla,
    pub tag_text_green: Hsla,
    pub tag_text_blue: Hsla,
    pub redorange: Hsla,
    pub tag_text_black: Hsla,
    pub tag_background_black: Hsla,
    pub white: Hsla,
    pub bg: Hsla,
    pub fg: Hsla,
    pub fg_5: Hsla,
    pub tag_background_orange: Hsla,
    pub tag_background_green: Hsla,
    pub tag_text_red: Hsla,
    pub tag_background_red: Hsla,
    pub tag_background_blue: Hsla,
    pub bg_color_active: Hsla,
    pub steps_default_color: Hsla,
    pub steps_highlight_color: Hsla,
    pub steps_font_size: f32,
    pub steps_lineheight: f32,
    pub steps_dot_size: &'static str,
    pub steps_icon_size: f32,
    pub steps_vertical_dot_gap: &'static str,
    pub steps_horizonal_dot_gap: f32,
    pub cell_margin_lr: f32,
    pub cell_padding_lr: f32,
    pub information_bar_fg: Hsla,
    pub information_bar_link: Hsla,
    pub information_bar_icon: Hsla,
    pub information_bar_bg: Hsla,
    pub information_bar_warn_weak_fg: Hsla,
    pub information_bar_warn_weak_link: Hsla,
    pub information_bar_warn_weak_icon: Hsla,
    pub information_bar_warn_weak_bg: Hsla,
    pub information_bar_no_color_bg: Hsla,
    pub information_bar_tips_strong_bg: Hsla,
    pub information_bar_tips_weak_fg: Hsla,
    pub information_bar_tips_weak_link: Hsla,
    pub information_bar_tips_weak_icon: Hsla,
    pub information_bar_tips_weak_bg: Hsla,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ThemeMode {
    #[default]
    System,
    Light,
    Dark,
}

impl Global for Theme {}

impl Default for Theme {
    fn default() -> Self {
        Self::light()
    }
}

impl Theme {
    pub fn light() -> Self {
        Self {
            is_dark: false,
            mode: ThemeMode::Light,
            btn_height: 48.0,
            btn_height_medium: 40.0,
            btn_height_small: 32.0,
            btn_active_mask: gpui::Rgba {
                r: 0.000000000,
                g: 0.000000000,
                b: 0.000000000,
                a: 0.2,
            }
            .into(),
            btn_default_active_bg: rgb(0xe6e6e6).into(),
            dialog_line_color: gpui::Rgba {
                r: 0.000000000,
                g: 0.000000000,
                b: 0.000000000,
                a: 0.1,
            }
            .into(),
            bg_0: rgb(0xededed).into(),
            bg_1: rgb(0xf7f7f7).into(),
            bg_2: rgb(0xffffff).into(),
            bg_3: rgb(0xf7f7f7).into(),
            bg_4: rgb(0x4c4c4c).into(),
            bg_5: rgb(0xffffff).into(),
            blue_100: rgb(0x10aeff).into(),
            blue_120: rgb(0x3fbeff).into(),
            blue_170: rgb(0xb7e6ff).into(),
            blue_80: rgb(0x0c8bcc).into(),
            blue_90: rgb(0x0e9ce6).into(),
            blue_bg_100: rgb(0x48a6e2).into(),
            blue_bg_110: rgb(0x5aafe4).into(),
            blue_bg_130: rgb(0x7fc0ea).into(),
            blue_bg_90: rgb(0x4095cb).into(),
            brand_100: rgb(0x07c160).into(),
            brand_120: rgb(0x38cd7f).into(),
            brand_170: rgb(0xb4ecce).into(),
            brand_80: rgb(0x059a4c).into(),
            brand_90: rgb(0x06ae56).into(),
            brand_bg_100: rgb(0x2aae67).into(),
            brand_bg_110: rgb(0x3eb575).into(),
            brand_bg_130: rgb(0x69c694).into(),
            brand_bg_90: rgb(0x259c5c).into(),
            fg_0: gpui::Rgba {
                r: 0.000000000,
                g: 0.000000000,
                b: 0.000000000,
                a: 0.9,
            }
            .into(),
            fg_0_5: gpui::Rgba {
                r: 0.000000000,
                g: 0.000000000,
                b: 0.000000000,
                a: 0.9,
            }
            .into(),
            fg_1: gpui::Rgba {
                r: 0.000000000,
                g: 0.000000000,
                b: 0.000000000,
                a: 0.55,
            }
            .into(),
            fg_2: gpui::Rgba {
                r: 0.000000000,
                g: 0.000000000,
                b: 0.000000000,
                a: 0.3,
            }
            .into(),
            fg_3: gpui::Rgba {
                r: 0.000000000,
                g: 0.000000000,
                b: 0.000000000,
                a: 0.1,
            }
            .into(),
            fg_4: gpui::Rgba {
                r: 0.000000000,
                g: 0.000000000,
                b: 0.000000000,
                a: 0.15,
            }
            .into(),
            glyph_0: gpui::Rgba {
                r: 0.000000000,
                g: 0.000000000,
                b: 0.000000000,
                a: 0.9,
            }
            .into(),
            glyph_1: gpui::Rgba {
                r: 0.000000000,
                g: 0.000000000,
                b: 0.000000000,
                a: 0.55,
            }
            .into(),
            glyph_2: gpui::Rgba {
                r: 0.000000000,
                g: 0.000000000,
                b: 0.000000000,
                a: 0.3,
            }
            .into(),
            glyph_white_0: gpui::Rgba {
                r: 1.000000000,
                g: 1.000000000,
                b: 1.000000000,
                a: 0.8,
            }
            .into(),
            glyph_white_1: gpui::Rgba {
                r: 1.000000000,
                g: 1.000000000,
                b: 1.000000000,
                a: 0.5,
            }
            .into(),
            glyph_white_2: gpui::Rgba {
                r: 1.000000000,
                g: 1.000000000,
                b: 1.000000000,
                a: 0.3,
            }
            .into(),
            glyph_white_3: rgb(0xffffff).into(),
            green_100: rgb(0x91d300).into(),
            green_120: rgb(0xa7db33).into(),
            green_170: rgb(0xdef1b3).into(),
            green_80: rgb(0x74a800).into(),
            green_90: rgb(0x82bd00).into(),
            green_bg_100: rgb(0x96be40).into(),
            green_bg_110: rgb(0xa0c452).into(),
            green_bg_130: rgb(0xb5d179).into(),
            green_bg_90: rgb(0x86aa39).into(),
            indigo_100: rgb(0x1485ee).into(),
            indigo_120: rgb(0x439df1).into(),
            indigo_170: rgb(0xb8daf9).into(),
            indigo_80: rgb(0x106abe).into(),
            indigo_90: rgb(0x1277d6).into(),
            indigo_bg_100: rgb(0x2b77bf).into(),
            indigo_bg_110: rgb(0x3f84c5).into(),
            indigo_bg_130: rgb(0x6ba0d2).into(),
            indigo_bg_90: rgb(0x266aab).into(),
            lightgreen_100: rgb(0x95ec69).into(),
            lightgreen_120: rgb(0xaaef87).into(),
            lightgreen_170: rgb(0xdef9d1).into(),
            lightgreen_80: rgb(0x77bc54).into(),
            lightgreen_90: rgb(0x85d35e).into(),
            lightgreen_bg_100: rgb(0x72cf60).into(),
            lightgreen_bg_110: rgb(0x80d370).into(),
            lightgreen_bg_130: rgb(0x9cdd90).into(),
            lightgreen_bg_90: rgb(0x66b956).into(),
            link_100: rgb(0x576b95).into(),
            link_120: rgb(0x7888aa).into(),
            link_170: rgb(0xccd2de).into(),
            link_80: rgb(0x455577).into(),
            link_90: rgb(0x4e6085).into(),
            linkfinder_100: rgb(0x002666).into(),
            material_attachmentcolumn: gpui::Rgba {
                r: 0.960784314,
                g: 0.960784314,
                b: 0.960784314,
                a: 0.95,
            }
            .into(),
            material_navigationbar: gpui::Rgba {
                r: 0.929411765,
                g: 0.929411765,
                b: 0.929411765,
                a: 0.94,
            }
            .into(),
            material_regular: gpui::Rgba {
                r: 0.968627451,
                g: 0.968627451,
                b: 0.968627451,
                a: 0.3,
            }
            .into(),
            material_thick: gpui::Rgba {
                r: 0.968627451,
                g: 0.968627451,
                b: 0.968627451,
                a: 0.8,
            }
            .into(),
            material_thin: gpui::Rgba {
                r: 1.000000000,
                g: 1.000000000,
                b: 1.000000000,
                a: 0.2,
            }
            .into(),
            material_toolbar: gpui::Rgba {
                r: 0.964705882,
                g: 0.964705882,
                b: 0.964705882,
                a: 0.82,
            }
            .into(),
            orange_100: rgb(0xfa9d3b).into(),
            orange_120: rgb(0xfbb062).into(),
            orange_170: rgb(0xfde1c3).into(),
            orange_80: rgb(0xc87d2f).into(),
            orange_90: rgb(0xe08c34).into(),
            orange_bg_100: rgb(0xea7800).into(),
            orange_bg_110: rgb(0xec8519).into(),
            orange_bg_130: rgb(0xf0a04d).into(),
            orange_bg_90: rgb(0xd26b00).into(),
            orangered_100: rgb(0xff6146).into(),
            overlay: gpui::Rgba {
                r: 0.000000000,
                g: 0.000000000,
                b: 0.000000000,
                a: 0.5,
            }
            .into(),
            overlay_white: gpui::Rgba {
                r: 0.949019608,
                g: 0.949019608,
                b: 0.949019608,
                a: 0.8,
            }
            .into(),
            purple_100: rgb(0x6467f0).into(),
            purple_120: rgb(0x8385f3).into(),
            purple_170: rgb(0xd0d1fa).into(),
            purple_80: rgb(0x5052c0).into(),
            purple_90: rgb(0x595cd7).into(),
            purple_bg_100: rgb(0x6769ba).into(),
            purple_bg_110: rgb(0x7678c1).into(),
            purple_bg_130: rgb(0x9496ce).into(),
            purple_bg_90: rgb(0x5c5ea7).into(),
            red_100: rgb(0xfa5151).into(),
            red_120: rgb(0xfb7373).into(),
            red_170: rgb(0xfdcaca).into(),
            red_80: rgb(0xc84040).into(),
            red_90: rgb(0xe14949).into(),
            red_bg_100: rgb(0xcf5148).into(),
            red_bg_110: rgb(0xd3625a).into(),
            red_bg_130: rgb(0xdd847e).into(),
            red_bg_90: rgb(0xb94840).into(),
            secondary_bg: gpui::Rgba {
                r: 0.000000000,
                g: 0.000000000,
                b: 0.000000000,
                a: 0.05,
            }
            .into(),
            separator_0: gpui::Rgba {
                r: 0.000000000,
                g: 0.000000000,
                b: 0.000000000,
                a: 0.1,
            }
            .into(),
            separator_1: gpui::Rgba {
                r: 0.000000000,
                g: 0.000000000,
                b: 0.000000000,
                a: 0.15,
            }
            .into(),
            statelayer_hovered: gpui::Rgba {
                r: 0.000000000,
                g: 0.000000000,
                b: 0.000000000,
                a: 0.02,
            }
            .into(),
            statelayer_pressed: gpui::Rgba {
                r: 0.000000000,
                g: 0.000000000,
                b: 0.000000000,
                a: 0.1,
            }
            .into(),
            statelayer_pressedstrengthened: gpui::Rgba {
                r: 0.000000000,
                g: 0.000000000,
                b: 0.000000000,
                a: 0.2,
            }
            .into(),
            yellow_100: rgb(0xffc300).into(),
            yellow_120: rgb(0xffcf33).into(),
            yellow_170: rgb(0xffecb2).into(),
            yellow_80: rgb(0xcc9c00).into(),
            yellow_90: rgb(0xe6af00).into(),
            yellow_bg_100: rgb(0xefb600).into(),
            yellow_bg_110: rgb(0xf0bd19).into(),
            yellow_bg_130: rgb(0xf3cc4d).into(),
            yellow_bg_90: rgb(0xd7a400).into(),
            fg_half: gpui::Rgba {
                r: 0.000000000,
                g: 0.000000000,
                b: 0.000000000,
                a: 0.9,
            }
            .into(),
            red: rgb(0xfa5151).into(),
            orangered: rgb(0xff6146).into(),
            orange: rgb(0xfa9d3b).into(),
            yellow: rgb(0xffc300).into(),
            green: rgb(0x91d300).into(),
            lightgreen: rgb(0x95ec69).into(),
            textgreen: rgb(0x06ae56).into(),
            brand: rgb(0x07c160).into(),
            blue: rgb(0x10aeff).into(),
            indigo: rgb(0x1485ee).into(),
            purple: rgb(0x6467f0).into(),
            link: rgb(0x576b95).into(),
            tag_text_orange: rgb(0xfa9d3b).into(),
            tag_text_green: rgb(0x06ae56).into(),
            tag_text_blue: rgb(0x10aeff).into(),
            redorange: rgb(0xff6146).into(),
            tag_text_black: gpui::Rgba {
                r: 0.000000000,
                g: 0.000000000,
                b: 0.000000000,
                a: 0.5,
            }
            .into(),
            tag_background_black: gpui::Rgba {
                r: 0.000000000,
                g: 0.000000000,
                b: 0.000000000,
                a: 0.05,
            }
            .into(),
            white: rgb(0xffffff).into(),
            bg: rgb(0xffffff).into(),
            fg: rgb(0x000000).into(),
            fg_5: gpui::Rgba {
                r: 0.000000000,
                g: 0.000000000,
                b: 0.000000000,
                a: 0.05,
            }
            .into(),
            tag_background_orange: gpui::Rgba {
                r: 0.980392157,
                g: 0.615686275,
                b: 0.231372549,
                a: 0.1,
            }
            .into(),
            tag_background_green: gpui::Rgba {
                r: 0.023529412,
                g: 0.682352941,
                b: 0.337254902,
                a: 0.1,
            }
            .into(),
            tag_text_red: gpui::Rgba {
                r: 0.980392157,
                g: 0.317647059,
                b: 0.317647059,
                a: 0.6,
            }
            .into(),
            tag_background_red: gpui::Rgba {
                r: 0.980392157,
                g: 0.317647059,
                b: 0.317647059,
                a: 0.1,
            }
            .into(),
            tag_background_blue: gpui::Rgba {
                r: 0.062745098,
                g: 0.682352941,
                b: 1.000000000,
                a: 0.1,
            }
            .into(),
            bg_color_active: rgb(0xececec).into(),
            steps_default_color: gpui::Rgba {
                r: 0.000000000,
                g: 0.000000000,
                b: 0.000000000,
                a: 0.1,
            }
            .into(),
            steps_highlight_color: rgb(0x07c160).into(),
            steps_font_size: 17.0,
            steps_lineheight: 1.4,
            steps_dot_size: "calc(8 / var(--weui-STEPS-FONT-SIZE) * 1em)",
            steps_icon_size: 40.0,
            steps_vertical_dot_gap: "calc((1em - var(--weui-STEPS-DOT-SIZE)) / 2)",
            steps_horizonal_dot_gap: 4.0,
            cell_margin_lr: 16.0,
            cell_padding_lr: 16.0,
            information_bar_fg: rgb(0xFFFFFF).into(),
            information_bar_link: rgb(0xFFFFFF).into(),
            information_bar_icon: rgb(0xFFFFFF).into(),
            information_bar_bg: rgb(0xFA5151).into(),
            information_bar_warn_weak_fg: gpui::Rgba {
                r: 0.000000000,
                g: 0.000000000,
                b: 0.000000000,
                a: 0.55,
            }
            .into(),
            information_bar_warn_weak_link: rgb(0x576B95).into(),
            information_bar_warn_weak_icon: rgb(0xFA5151).into(),
            information_bar_warn_weak_bg: gpui::Rgba {
                r: 0.980392157,
                g: 0.317647059,
                b: 0.317647059,
                a: 0.1,
            }
            .into(),
            information_bar_no_color_bg: gpui::Rgba {
                r: 0.000000000,
                g: 0.000000000,
                b: 0.000000000,
                a: 0.3,
            }
            .into(),
            information_bar_tips_strong_bg: rgb(0xFA9D3B).into(),
            information_bar_tips_weak_fg: gpui::Rgba {
                r: 0.000000000,
                g: 0.000000000,
                b: 0.000000000,
                a: 0.55,
            }
            .into(),
            information_bar_tips_weak_link: rgb(0x576B95).into(),
            information_bar_tips_weak_icon: gpui::Rgba {
                r: 0.000000000,
                g: 0.000000000,
                b: 0.000000000,
                a: 0.55,
            }
            .into(),
            information_bar_tips_weak_bg: rgb(0xF7F7F7).into(),
        }
    }
    pub fn dark() -> Self {
        Self {
            is_dark: true,
            mode: ThemeMode::Dark,
            btn_active_mask: gpui::Rgba {
                r: 1.000000000,
                g: 1.000000000,
                b: 1.000000000,
                a: 0.2,
            }
            .into(),
            btn_default_active_bg: gpui::Rgba {
                r: 1.000000000,
                g: 1.000000000,
                b: 1.000000000,
                a: 0.126,
            }
            .into(),
            dialog_line_color: gpui::Rgba {
                r: 1.000000000,
                g: 1.000000000,
                b: 1.000000000,
                a: 0.1,
            }
            .into(),
            bg_0: rgb(0x111111).into(),
            bg_1: rgb(0x1e1e1e).into(),
            bg_2: rgb(0x191919).into(),
            bg_3: rgb(0x202020).into(),
            bg_4: rgb(0x404040).into(),
            bg_5: rgb(0x2c2c2c).into(),
            blue_100: rgb(0x10aeff).into(),
            blue_120: rgb(0x0c8bcc).into(),
            blue_170: rgb(0x04344d).into(),
            blue_80: rgb(0x3fbeff).into(),
            blue_90: rgb(0x28b6ff).into(),
            blue_bg_100: rgb(0x48a6e2).into(),
            blue_bg_110: rgb(0x4095cb).into(),
            blue_bg_130: rgb(0x32749e).into(),
            blue_bg_90: rgb(0x5aafe4).into(),
            brand_100: rgb(0x07c160).into(),
            brand_120: rgb(0x059a4c).into(),
            brand_170: rgb(0x023a1c).into(),
            brand_80: rgb(0x38cd7f).into(),
            brand_90: rgb(0x20c770).into(),
            brand_bg_100: rgb(0x2aae67).into(),
            brand_bg_110: rgb(0x259c5c).into(),
            brand_bg_130: rgb(0x1d7a48).into(),
            brand_bg_90: rgb(0x3eb575).into(),
            fg_0: gpui::Rgba {
                r: 1.000000000,
                g: 1.000000000,
                b: 1.000000000,
                a: 0.8,
            }
            .into(),
            fg_0_5: gpui::Rgba {
                r: 1.000000000,
                g: 1.000000000,
                b: 1.000000000,
                a: 0.6,
            }
            .into(),
            fg_1: gpui::Rgba {
                r: 1.000000000,
                g: 1.000000000,
                b: 1.000000000,
                a: 0.5,
            }
            .into(),
            fg_2: gpui::Rgba {
                r: 1.000000000,
                g: 1.000000000,
                b: 1.000000000,
                a: 0.3,
            }
            .into(),
            fg_3: gpui::Rgba {
                r: 1.000000000,
                g: 1.000000000,
                b: 1.000000000,
                a: 0.1,
            }
            .into(),
            fg_4: gpui::Rgba {
                r: 1.000000000,
                g: 1.000000000,
                b: 1.000000000,
                a: 0.15,
            }
            .into(),
            glyph_0: gpui::Rgba {
                r: 1.000000000,
                g: 1.000000000,
                b: 1.000000000,
                a: 0.8,
            }
            .into(),
            glyph_1: gpui::Rgba {
                r: 1.000000000,
                g: 1.000000000,
                b: 1.000000000,
                a: 0.5,
            }
            .into(),
            glyph_2: gpui::Rgba {
                r: 1.000000000,
                g: 1.000000000,
                b: 1.000000000,
                a: 0.3,
            }
            .into(),
            glyph_white_0: gpui::Rgba {
                r: 1.000000000,
                g: 1.000000000,
                b: 1.000000000,
                a: 0.8,
            }
            .into(),
            glyph_white_1: gpui::Rgba {
                r: 1.000000000,
                g: 1.000000000,
                b: 1.000000000,
                a: 0.5,
            }
            .into(),
            glyph_white_2: gpui::Rgba {
                r: 1.000000000,
                g: 1.000000000,
                b: 1.000000000,
                a: 0.3,
            }
            .into(),
            glyph_white_3: rgb(0xffffff).into(),
            green_100: rgb(0x74a800).into(),
            green_120: rgb(0x5c8600).into(),
            green_170: rgb(0x233200).into(),
            green_80: rgb(0x8fb933).into(),
            green_90: rgb(0x82b01a).into(),
            green_bg_100: rgb(0x789833).into(),
            green_bg_110: rgb(0x6b882d).into(),
            green_bg_130: rgb(0x65802b).into(),
            green_bg_90: rgb(0x85a247).into(),
            indigo_100: rgb(0x1196ff).into(),
            indigo_120: rgb(0x0d78cc).into(),
            indigo_170: rgb(0x052d4d).into(),
            indigo_80: rgb(0x40abff).into(),
            indigo_90: rgb(0x28a0ff).into(),
            indigo_bg_100: rgb(0x0d78cc).into(),
            indigo_bg_110: rgb(0x0b6bb7).into(),
            indigo_bg_130: rgb(0x09548f).into(),
            indigo_bg_90: rgb(0x2585d1).into(),
            lightgreen_100: rgb(0x3eb575).into(),
            lightgreen_120: rgb(0x31905d).into(),
            lightgreen_170: rgb(0x123522).into(),
            lightgreen_80: rgb(0x64c390).into(),
            lightgreen_90: rgb(0x51bc83).into(),
            lightgreen_bg_100: rgb(0x31905d).into(),
            lightgreen_bg_110: rgb(0x2c8153).into(),
            lightgreen_bg_130: rgb(0x226541).into(),
            lightgreen_bg_90: rgb(0x31905d).into(),
            link_100: rgb(0x7d90a9).into(),
            link_120: rgb(0x647387).into(),
            link_170: rgb(0x252a32).into(),
            link_80: rgb(0x97a6ba).into(),
            link_90: rgb(0x899ab1).into(),
            linkfinder_100: rgb(0xdee9ff).into(),
            material_attachmentcolumn: gpui::Rgba {
                r: 0.125490196,
                g: 0.125490196,
                b: 0.125490196,
                a: 0.93,
            }
            .into(),
            material_navigationbar: gpui::Rgba {
                r: 0.070588235,
                g: 0.070588235,
                b: 0.070588235,
                a: 0.9,
            }
            .into(),
            material_regular: gpui::Rgba {
                r: 0.145098039,
                g: 0.145098039,
                b: 0.145098039,
                a: 0.6,
            }
            .into(),
            material_thick: gpui::Rgba {
                r: 0.133333333,
                g: 0.133333333,
                b: 0.133333333,
                a: 0.9,
            }
            .into(),
            material_thin: gpui::Rgba {
                r: 0.372549020,
                g: 0.372549020,
                b: 0.372549020,
                a: 0.4,
            }
            .into(),
            material_toolbar: gpui::Rgba {
                r: 0.137254902,
                g: 0.137254902,
                b: 0.137254902,
                a: 0.93,
            }
            .into(),
            orange_100: rgb(0xc87d2f).into(),
            orange_120: rgb(0xa06425).into(),
            orange_170: rgb(0x3b250e).into(),
            orange_80: rgb(0xd39758).into(),
            orange_90: rgb(0xcd8943).into(),
            orange_bg_100: rgb(0xbb6000).into(),
            orange_bg_110: rgb(0xa85600).into(),
            orange_bg_130: rgb(0x824300).into(),
            orange_bg_90: rgb(0xc1701a).into(),
            orangered_100: rgb(0xff6146).into(),
            overlay: gpui::Rgba {
                r: 0.000000000,
                g: 0.000000000,
                b: 0.000000000,
                a: 0.8,
            }
            .into(),
            overlay_white: gpui::Rgba {
                r: 0.949019608,
                g: 0.949019608,
                b: 0.949019608,
                a: 0.8,
            }
            .into(),
            purple_100: rgb(0x8183ff).into(),
            purple_120: rgb(0x6768cc).into(),
            purple_170: rgb(0x26274c).into(),
            purple_80: rgb(0x9a9bff).into(),
            purple_90: rgb(0x8d8fff).into(),
            purple_bg_100: rgb(0x6768cc).into(),
            purple_bg_110: rgb(0x5c5db7).into(),
            purple_bg_130: rgb(0x48498f).into(),
            purple_bg_90: rgb(0x7677d1).into(),
            red_100: rgb(0xfa5151).into(),
            red_120: rgb(0xc84040).into(),
            red_170: rgb(0x4b1818).into(),
            red_80: rgb(0xfb7373).into(),
            red_90: rgb(0xfa6262).into(),
            red_bg_100: rgb(0xcf5148).into(),
            red_bg_110: rgb(0xba4940).into(),
            red_bg_130: rgb(0x913832).into(),
            red_bg_90: rgb(0xd3625a).into(),
            secondary_bg: gpui::Rgba {
                r: 1.000000000,
                g: 1.000000000,
                b: 1.000000000,
                a: 0.1,
            }
            .into(),
            separator_0: gpui::Rgba {
                r: 1.000000000,
                g: 1.000000000,
                b: 1.000000000,
                a: 0.05,
            }
            .into(),
            separator_1: gpui::Rgba {
                r: 1.000000000,
                g: 1.000000000,
                b: 1.000000000,
                a: 0.15,
            }
            .into(),
            statelayer_hovered: gpui::Rgba {
                r: 0.000000000,
                g: 0.000000000,
                b: 0.000000000,
                a: 0.02,
            }
            .into(),
            statelayer_pressed: gpui::Rgba {
                r: 1.000000000,
                g: 1.000000000,
                b: 1.000000000,
                a: 0.1,
            }
            .into(),
            statelayer_pressedstrengthened: gpui::Rgba {
                r: 1.000000000,
                g: 1.000000000,
                b: 1.000000000,
                a: 0.2,
            }
            .into(),
            yellow_100: rgb(0xcc9c00).into(),
            yellow_120: rgb(0xa37c00).into(),
            yellow_170: rgb(0x3d2f00).into(),
            yellow_80: rgb(0xd6af33).into(),
            yellow_90: rgb(0xd1a519).into(),
            yellow_bg_100: rgb(0xbf9100).into(),
            yellow_bg_110: rgb(0xab8200).into(),
            yellow_bg_130: rgb(0x866500).into(),
            yellow_bg_90: rgb(0xc59c1a).into(),
            fg_half: gpui::Rgba {
                r: 1.000000000,
                g: 1.000000000,
                b: 1.000000000,
                a: 0.6,
            }
            .into(),
            red: rgb(0xfa5151).into(),
            orangered: rgb(0xff6146).into(),
            orange: rgb(0xc87d2f).into(),
            yellow: rgb(0xcc9c00).into(),
            green: rgb(0x74a800).into(),
            lightgreen: rgb(0x3eb575).into(),
            textgreen: rgb(0x259c5c).into(),
            brand: rgb(0x07c160).into(),
            blue: rgb(0x10aeff).into(),
            indigo: rgb(0x1196ff).into(),
            purple: rgb(0x8183ff).into(),
            link: rgb(0x7d90a9).into(),
            redorange: rgb(0xff6146).into(),
            tag_text_black: gpui::Rgba {
                r: 1.000000000,
                g: 1.000000000,
                b: 1.000000000,
                a: 0.5,
            }
            .into(),
            tag_background_black: gpui::Rgba {
                r: 1.000000000,
                g: 1.000000000,
                b: 1.000000000,
                a: 0.05,
            }
            .into(),
            white: gpui::Rgba {
                r: 1.000000000,
                g: 1.000000000,
                b: 1.000000000,
                a: 0.8,
            }
            .into(),
            fg: rgb(0xffffff).into(),
            bg: rgb(0x000000).into(),
            fg_5: gpui::Rgba {
                r: 1.000000000,
                g: 1.000000000,
                b: 1.000000000,
                a: 0.1,
            }
            .into(),
            tag_background_orange: gpui::Rgba {
                r: 0.980392157,
                g: 0.615686275,
                b: 0.231372549,
                a: 0.1,
            }
            .into(),
            tag_background_green: gpui::Rgba {
                r: 0.023529412,
                g: 0.682352941,
                b: 0.337254902,
                a: 0.1,
            }
            .into(),
            tag_text_red: gpui::Rgba {
                r: 0.980392157,
                g: 0.317647059,
                b: 0.317647059,
                a: 0.6,
            }
            .into(),
            tag_background_red: gpui::Rgba {
                r: 0.980392157,
                g: 0.317647059,
                b: 0.317647059,
                a: 0.1,
            }
            .into(),
            tag_background_blue: gpui::Rgba {
                r: 0.062745098,
                g: 0.682352941,
                b: 1.000000000,
                a: 0.1,
            }
            .into(),
            tag_text_orange: gpui::Rgba {
                r: 0.980392157,
                g: 0.615686275,
                b: 0.231372549,
                a: 0.6,
            }
            .into(),
            tag_text_green: gpui::Rgba {
                r: 0.023529412,
                g: 0.682352941,
                b: 0.337254902,
                a: 0.6,
            }
            .into(),
            tag_text_blue: gpui::Rgba {
                r: 0.062745098,
                g: 0.682352941,
                b: 1.000000000,
                a: 0.6,
            }
            .into(),
            bg_color_active: rgb(0x373737).into(),
            information_bar_fg: rgb(0xFFFFFF).into(),
            information_bar_link: rgb(0xFFFFFF).into(),
            information_bar_icon: rgb(0xFFFFFF).into(),
            information_bar_bg: rgb(0xFA5151).into(),
            information_bar_warn_weak_fg: gpui::Rgba {
                r: 1.000000000,
                g: 1.000000000,
                b: 1.000000000,
                a: 0.5,
            }
            .into(),
            information_bar_warn_weak_link: rgb(0x7D90A9).into(),
            information_bar_warn_weak_icon: rgb(0xFA5151).into(),
            information_bar_warn_weak_bg: gpui::Rgba {
                r: 0.980392157,
                g: 0.317647059,
                b: 0.317647059,
                a: 0.1,
            }
            .into(),
            information_bar_no_color_bg: gpui::Rgba {
                r: 1.000000000,
                g: 1.000000000,
                b: 1.000000000,
                a: 0.3,
            }
            .into(),
            information_bar_tips_strong_bg: rgb(0xC87D2F).into(),
            information_bar_tips_weak_fg: gpui::Rgba {
                r: 1.000000000,
                g: 1.000000000,
                b: 1.000000000,
                a: 0.5,
            }
            .into(),
            information_bar_tips_weak_link: rgb(0x7D90A9).into(),
            information_bar_tips_weak_icon: gpui::Rgba {
                r: 1.000000000,
                g: 1.000000000,
                b: 1.000000000,
                a: 0.5,
            }
            .into(),
            information_bar_tips_weak_bg: rgb(0x1E1E1E).into(),
            btn_height: 48.0,
            btn_height_medium: 40.0,
            btn_height_small: 32.0,
            steps_default_color: gpui::Rgba {
                r: 1.000000000,
                g: 1.000000000,
                b: 1.000000000,
                a: 0.1,
            }
            .into(),
            steps_highlight_color: rgb(0x07c160).into(),
            steps_font_size: 17.0,
            steps_lineheight: 1.4,
            steps_dot_size: "calc(8 / var(--weui-STEPS-FONT-SIZE) * 1em)",
            steps_icon_size: 40.0,
            steps_vertical_dot_gap: "calc((1em - var(--weui-STEPS-DOT-SIZE)) / 2)",
            steps_horizonal_dot_gap: 4.0,
            cell_margin_lr: 16.0,
            cell_padding_lr: 16.0,
        }
    }
    pub fn with_brand(mut self, brand: impl Into<Hsla>) -> Self {
        self.brand = brand.into();
        self
    }
    pub fn get(cx: &App) -> &Self {
        cx.global::<Self>()
    }
    pub fn set_mode(mode: ThemeMode, cx: &mut App) {
        let brand = Self::get(cx).brand;
        let dark = matches!(
            cx.window_appearance(),
            WindowAppearance::Dark | WindowAppearance::VibrantDark
        );
        let mut theme = match mode {
            ThemeMode::Dark => Self::dark(),
            ThemeMode::Light => Self::light(),
            ThemeMode::System if dark => Self::dark(),
            _ => Self::light(),
        };
        theme.mode = mode;
        theme.brand = brand;
        cx.set_global(theme);
        cx.refresh_windows();
    }
    pub fn init(cx: &mut App) {
        cx.set_global(Self::light());
        Self::set_mode(ThemeMode::System, cx);
    }
    pub fn observe(window: &mut Window) {
        window
            .observe_window_appearance(|_, cx| {
                if Self::get(cx).mode == ThemeMode::System {
                    Self::set_mode(ThemeMode::System, cx);
                }
            })
            .detach();
    }
}
