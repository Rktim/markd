use termimad::crossterm::style::Attribute;
use termimad::{rgb, MadSkin};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Theme {
    VsCode,
    GitHubDark,
    GitHubLight,
    Dracula,
    Catppuccin,
    Nord,
    Gruvbox,
    TokyoNight,
    OneDark,
    Omarchy,
}

impl Theme {
    pub const ALL: [Theme; 10] = [
        Theme::VsCode,
        Theme::GitHubDark,
        Theme::GitHubLight,
        Theme::Dracula,
        Theme::Catppuccin,
        Theme::Nord,
        Theme::Gruvbox,
        Theme::TokyoNight,
        Theme::OneDark,
        Theme::Omarchy,
    ];

    pub fn name(self) -> &'static str {
        match self {
            Theme::VsCode => "VS Code Dark+",
            Theme::GitHubDark => "GitHub Dark",
            Theme::GitHubLight => "GitHub Light",
            Theme::Dracula => "Dracula",
            Theme::Catppuccin => "Catppuccin Mocha",
            Theme::Nord => "Nord",
            Theme::Gruvbox => "Gruvbox",
            Theme::TokyoNight => "Tokyo Night",
            Theme::OneDark => "One Dark",
            Theme::Omarchy => "Omarchy",
        }
    }

    pub fn skin(self) -> MadSkin {
        let mut skin = MadSkin::default();

        match self {
            Theme::VsCode => {
                skin.bold.set_fg(rgb(86, 156, 214));
                skin.italic.set_fg(rgb(206, 145, 120));

                skin.inline_code.set_fg(rgb(206, 145, 120));
                skin.inline_code.set_bg(rgb(40, 40, 40));

                skin.headers[0].set_fg(rgb(78, 201, 176));
                skin.headers[0].add_attr(Attribute::Bold);

                skin.headers[1].set_fg(rgb(86, 156, 214));
                skin.headers[1].add_attr(Attribute::Bold);

                skin.headers[2].set_fg(rgb(220, 220, 170));

                skin.code_block.set_fg(rgb(220, 220, 220));
                skin.code_block.set_bg(rgb(30, 30, 30));

                skin.bullet.set_fg(rgb(86, 156, 214));
            }

            Theme::GitHubDark => {
                skin.bold.set_fg(rgb(88, 166, 255));
                skin.italic.set_fg(rgb(210, 168, 255));

                skin.inline_code.set_fg(rgb(255, 123, 114));
                skin.inline_code.set_bg(rgb(40, 44, 52));

                skin.headers[0].set_fg(rgb(88, 166, 255));
                skin.headers[0].add_attr(Attribute::Bold);

                skin.headers[1].set_fg(rgb(126, 231, 135));
                skin.headers[1].add_attr(Attribute::Bold);

                skin.headers[2].set_fg(rgb(255, 166, 87));

                skin.code_block.set_fg(rgb(201, 209, 217));
                skin.code_block.set_bg(rgb(22, 27, 34));

                skin.bullet.set_fg(rgb(88, 166, 255));
            }

            Theme::GitHubLight => {
                skin.bold.set_fg(rgb(9, 105, 218));
                skin.italic.set_fg(rgb(111, 66, 193));

                skin.inline_code.set_fg(rgb(36, 41, 47));
                skin.inline_code.set_bg(rgb(246, 248, 250));

                skin.headers[0].set_fg(rgb(9, 105, 218));
                skin.headers[0].add_attr(Attribute::Bold);

                skin.headers[1].set_fg(rgb(26, 127, 55));
                skin.headers[1].add_attr(Attribute::Bold);

                skin.code_block.set_fg(rgb(36, 41, 47));
                skin.code_block.set_bg(rgb(246, 248, 250));

                skin.bullet.set_fg(rgb(9, 105, 218));
            }

            Theme::Dracula => {
                skin.bold.set_fg(rgb(139, 233, 253));
                skin.italic.set_fg(rgb(255, 121, 198));

                skin.inline_code.set_fg(rgb(80, 250, 123));
                skin.inline_code.set_bg(rgb(68, 71, 90));

                skin.headers[0].set_fg(rgb(189, 147, 249));
                skin.headers[0].add_attr(Attribute::Bold);

                skin.headers[1].set_fg(rgb(139, 233, 253));
                skin.headers[1].add_attr(Attribute::Bold);

                skin.headers[2].set_fg(rgb(255, 184, 108));

                skin.code_block.set_fg(rgb(248, 248, 242));
                skin.code_block.set_bg(rgb(40, 42, 54));

                skin.bullet.set_fg(rgb(255, 121, 198));
            }

            Theme::Catppuccin => {
                skin.bold.set_fg(rgb(137, 180, 250));
                skin.italic.set_fg(rgb(245, 194, 231));

                skin.inline_code.set_fg(rgb(166, 227, 161));
                skin.inline_code.set_bg(rgb(49, 50, 68));

                skin.headers[0].set_fg(rgb(137, 180, 250));
                skin.headers[0].add_attr(Attribute::Bold);

                skin.headers[1].set_fg(rgb(203, 166, 247));
                skin.headers[1].add_attr(Attribute::Bold);

                skin.headers[2].set_fg(rgb(148, 226, 213));

                skin.code_block.set_fg(rgb(205, 214, 244));
                skin.code_block.set_bg(rgb(30, 30, 46));

                skin.bullet.set_fg(rgb(245, 194, 231));
            }

            Theme::Nord => {
                skin.bold.set_fg(rgb(136, 192, 208));
                skin.italic.set_fg(rgb(180, 142, 173));

                skin.inline_code.set_fg(rgb(163, 190, 140));
                skin.inline_code.set_bg(rgb(46, 52, 64));

                skin.headers[0].set_fg(rgb(136, 192, 208));
                skin.headers[0].add_attr(Attribute::Bold);

                skin.headers[1].set_fg(rgb(129, 161, 193));
                skin.headers[1].add_attr(Attribute::Bold);

                skin.headers[2].set_fg(rgb(235, 203, 139));

                skin.code_block.set_fg(rgb(216, 222, 233));
                skin.code_block.set_bg(rgb(46, 52, 64));

                skin.bullet.set_fg(rgb(143, 188, 187));
            }

            Theme::Gruvbox => {
                skin.bold.set_fg(rgb(131, 165, 152));
                skin.italic.set_fg(rgb(211, 134, 155));

                skin.inline_code.set_fg(rgb(184, 187, 38));
                skin.inline_code.set_bg(rgb(60, 56, 54));

                skin.headers[0].set_fg(rgb(250, 189, 47));
                skin.headers[0].add_attr(Attribute::Bold);

                skin.headers[1].set_fg(rgb(184, 187, 38));
                skin.headers[1].add_attr(Attribute::Bold);

                skin.headers[2].set_fg(rgb(254, 128, 125));

                skin.code_block.set_fg(rgb(235, 219, 178));
                skin.code_block.set_bg(rgb(40, 40, 40));

                skin.bullet.set_fg(rgb(131, 165, 152));
            }

            Theme::TokyoNight => {
                skin.bold.set_fg(rgb(125, 207, 255));
                skin.italic.set_fg(rgb(187, 154, 247));

                skin.inline_code.set_fg(rgb(158, 206, 106));
                skin.inline_code.set_bg(rgb(31, 35, 53));

                skin.headers[0].set_fg(rgb(125, 207, 255));
                skin.headers[0].add_attr(Attribute::Bold);

                skin.headers[1].set_fg(rgb(187, 154, 247));
                skin.headers[1].add_attr(Attribute::Bold);

                skin.headers[2].set_fg(rgb(255, 158, 100));

                skin.code_block.set_fg(rgb(192, 202, 245));
                skin.code_block.set_bg(rgb(26, 27, 38));

                skin.bullet.set_fg(rgb(115, 218, 202));
            }

            Theme::OneDark => {
                skin.bold.set_fg(rgb(97, 175, 239));
                skin.italic.set_fg(rgb(198, 120, 221));

                skin.inline_code.set_fg(rgb(152, 195, 121));
                skin.inline_code.set_bg(rgb(40, 44, 52));

                skin.headers[0].set_fg(rgb(97, 175, 239));
                skin.headers[0].add_attr(Attribute::Bold);

                skin.headers[1].set_fg(rgb(198, 120, 221));
                skin.headers[1].add_attr(Attribute::Bold);

                skin.headers[2].set_fg(rgb(229, 192, 123));

                skin.code_block.set_fg(rgb(171, 178, 191));
                skin.code_block.set_bg(rgb(40, 44, 52));

                skin.bullet.set_fg(rgb(86, 182, 194));
            }

            Theme::Omarchy => {
                skin.bold.set_fg(rgb(245, 224, 220));
                skin.italic.set_fg(rgb(203, 166, 247));

                skin.inline_code.set_fg(rgb(137, 180, 250));
                skin.inline_code.set_bg(rgb(30, 30, 46));

                skin.headers[0].set_fg(rgb(137, 220, 235));
                skin.headers[0].add_attr(Attribute::Bold);

                skin.headers[1].set_fg(rgb(203, 166, 247));
                skin.headers[1].add_attr(Attribute::Bold);

                skin.headers[2].set_fg(rgb(245, 224, 220));

                skin.code_block.set_fg(rgb(205, 214, 244));
                skin.code_block.set_bg(rgb(24, 24, 37));

                skin.bullet.set_fg(rgb(243, 139, 168));
            }
        }

        skin
    }
}
