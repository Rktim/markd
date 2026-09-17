use ::crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{
        self,
        DisableMouseCapture,
        EnableMouseCapture,
        Event,
        KeyCode,
        KeyEventKind,
        MouseButton,
        MouseEventKind,
    },
    style::{
        Color,
        Print,
        ResetColor,
        SetBackgroundColor,
        SetForegroundColor,
    },
    terminal::{
        self,
        Clear,
        ClearType,
        EnterAlternateScreen,
        LeaveAlternateScreen,
    },
    QueueableCommand,
};

use std::error::Error;
use std::io::{stdout, Stdout, Write};
use std::thread;
use std::time::Duration;

use termimad::{Area, MadView};

use crate::document::{Document, DocumentMode};
use crate::theme::Theme;

pub fn run(document: &Document) -> Result<(), Box<dyn Error>> {
    let mut stdout = stdout();

    terminal::enable_raw_mode()?;

    stdout.queue(EnterAlternateScreen)?;
    stdout.queue(EnableMouseCapture)?;
    stdout.queue(Hide)?;
    stdout.flush()?;

    // Opening animation.
    intro_animation(&mut stdout)?;

    let result = run_loop(document, &mut stdout);

    // Always restore terminal state.
    let _ = stdout.queue(Show);
    let _ = stdout.queue(DisableMouseCapture);
    let _ = stdout.queue(LeaveAlternateScreen);
    let _ = stdout.flush();
    let _ = terminal::disable_raw_mode();

    result
}

fn intro_animation(
    stdout: &mut Stdout,
) -> Result<(), Box<dyn Error>> {
    // Mistral-inspired startup palette:
    // warm orange/red primary + light neutral text.
    let primary = Color::Rgb {
        r: 255,
        g: 112,
        b: 64,
    };

    let secondary = Color::Rgb {
        r: 255,
        g: 170,
        b: 120,
    };

    let muted = Color::Rgb {
        r: 35,
        g: 33,
        b: 39,
    };

    let logo = [
        "  ███╗   ███╗ █████╗ ██████╗ ██╗  ██╗██████╗ ",
        "  ████╗ ████║██╔══██╗██╔══██╗██║ ██╔╝██╔══██╗",
        "  ██╔████╔██║███████║██████╔╝█████╔╝ ██║  ██║",
        "  ██║╚██╔╝██║██╔══██║██╔══██╗██╔═██╗ ██║  ██║",
        "  ██║ ╚═╝ ██║██║  ██║██║  ██║██║  ██╗██████╔╝",
        "  ╚═╝     ╚═╝╚═╝  ╚═╝╚═╝  ╚═╝╚═╝  ╚═╝╚═════╝  ██╗",
        "                                              ╚═╝",
    ];

    stdout.queue(Clear(ClearType::All))?;
    stdout.queue(ResetColor)?;

    // ------------------------------------------------------------
    // Center logo
    // ------------------------------------------------------------
    let (width, height) = terminal::size()?;

    let logo_width = logo
        .iter()
        .map(|line| line.chars().count())
        .max()
        .unwrap_or(0);

    let start_x =
        width.saturating_sub(logo_width as u16) / 2;

    let start_y =
        height.saturating_sub(10) / 2;

    // ------------------------------------------------------------
    // Pop the logo in column by column.
    //
    // Instead of revealing the entire line at once,
    // progressively reveal each column of the ASCII art.
    // ------------------------------------------------------------
    let max_columns = logo_width;

    for column in 0..max_columns {
        for (row, line) in logo.iter().enumerate() {
            let chars: Vec<char> = line.chars().collect();

            if column >= chars.len() {
                continue;
            }

            let ch = chars[column];

            // Don't render empty spaces.
            if ch == ' ' {
                continue;
            }

            stdout.queue(
                MoveTo(
                    start_x + column as u16,
                    start_y + row as u16,
                ),
            )?;

            // Slight alternating highlight for the pop effect.
            if column % 3 == 0 {
                stdout.queue(
                    SetForegroundColor(primary),
                )?;
            } else {
                stdout.queue(
                    SetForegroundColor(secondary),
                )?;
            }

            stdout.queue(Print(ch))?;
        }

        stdout.flush()?;

        thread::sleep(
            Duration::from_millis(28),
        );
    }

    // ------------------------------------------------------------
    // Small glow / settle effect.
    // ------------------------------------------------------------
    thread::sleep(
        Duration::from_millis(160),
    );

    // Repaint the logo cleanly in the primary color.
    for (row, line) in logo.iter().enumerate() {
        stdout.queue(
            MoveTo(
                start_x,
                start_y + row as u16,
            ),
        )?;

        stdout.queue(
            SetForegroundColor(primary),
        )?;

        stdout.queue(Print(*line))?;
    }

    // ------------------------------------------------------------
    // Subtitle
    // ------------------------------------------------------------
    let subtitle =
        "universal terminal document viewer";

    let subtitle_x =
        width.saturating_sub(
            subtitle.len() as u16
        ) / 2;

    stdout.queue(
        MoveTo(
            subtitle_x,
            start_y + 8,
        ),
    )?;

    stdout.queue(
        SetForegroundColor(muted),
    )?;

    stdout.queue(
        Print(subtitle),
    )?;

    // ------------------------------------------------------------
    // Ready indicator
    // ------------------------------------------------------------
    stdout.queue(
        MoveTo(
            width.saturating_sub(21) / 2,
            start_y + 9,
        ),
    )?;

    stdout.queue(
        SetForegroundColor(primary),
    )?;

    stdout.queue(
        Print("● loading"),
    )?;

    stdout.flush()?;

    thread::sleep(
        Duration::from_millis(220),
    );

    stdout.queue(
        MoveTo(
            width.saturating_sub(21) / 2,
            start_y + 9,
        ),
    )?;

    stdout.queue(
        SetForegroundColor(primary),
    )?;

    stdout.queue(
        Print("✓ ready  "),
    )?;

    stdout.flush()?;

    // Let the user actually see the completed splash.
    thread::sleep(
        Duration::from_millis(700),
    );

    stdout.queue(
        Clear(ClearType::All),
    )?;

    stdout.queue(ResetColor)?;
    stdout.flush()?;

    Ok(())
}

fn run_loop(
    document: &Document,
    stdout: &mut Stdout,
) -> Result<(), Box<dyn Error>> {
    let mut current_theme = Theme::VsCode;
    let mut selected_theme = 0usize;
    let mut menu_open = false;

    let (width, height) = terminal::size()?;

    let mut area = Area::new(
        0,
        1,
        width,
        height.saturating_sub(2),
    );

    let mut view = MadView::from(
        document.content.clone(),
        area.clone(),
        current_theme.skin(),
    );

    loop {
        render(
            document,
            &mut view,
            current_theme,
            menu_open,
            selected_theme,
            stdout,
        )?;

        match event::read()? {
            Event::Key(key)
                if key.kind == KeyEventKind::Press =>
            {
                if menu_open {
                    handle_menu_key(
                        key.code,
                        document,
                        &mut view,
                        &mut current_theme,
                        &mut selected_theme,
                        &area,
                        &mut menu_open,
                    )?;
                } else {
                    match key.code {
                        // Esc = quit
                        KeyCode::Esc => {
                            break;
                        }

                        // Open theme menu
                        KeyCode::Char('t')
                        | KeyCode::Char('m') => {
                            menu_open = true;
                        }

                        // Scroll
                        KeyCode::Down
                        | KeyCode::Char('j') => {
                            view.try_scroll_lines(1);
                        }

                        KeyCode::Up
                        | KeyCode::Char('k') => {
                            view.try_scroll_lines(-1);
                        }

                        // Page down
                        KeyCode::PageDown
                        | KeyCode::Char('f')
                        | KeyCode::Char(' ') => {
                            view.try_scroll_pages(1);
                        }

                        // Page up
                        KeyCode::PageUp
                        | KeyCode::Char('b') => {
                            view.try_scroll_pages(-1);
                        }

                        // Top
                        KeyCode::Home
                        | KeyCode::Char('g') => {
                            view.scroll = 0;
                        }

                        // Bottom
                        KeyCode::Char('G') => {
                            view.scroll = usize::MAX;
                        }

                        _ => {}
                    }
                }
            }

            Event::Mouse(mouse) => {
                let (term_w, _) = terminal::size()?;

                let gear_x = term_w.saturating_sub(7);

                // Hover gear -> open menu.
                let over_gear =
                    mouse.row == 0
                        && mouse.column >= gear_x
                        && mouse.column < term_w;

                if over_gear {
                    menu_open = true;
                    continue;
                }

                if menu_open {
                    let (
                        menu_x,
                        menu_y,
                        menu_w,
                        menu_h,
                    ) = menu_geometry(term_w);

                    let over_menu = inside(
                        mouse.column,
                        mouse.row,
                        menu_x,
                        menu_y,
                        menu_w,
                        menu_h,
                    );

                    if over_menu {
                        let theme_start =
                            menu_y + 3;

                        let theme_end =
                            theme_start
                                + Theme::ALL.len() as u16;

                        if mouse.row >= theme_start
                            && mouse.row < theme_end
                        {
                            let index =
                                (mouse.row - theme_start)
                                    as usize;

                            if index < Theme::ALL.len() {
                                selected_theme = index;

                                if mouse.kind
                                    == MouseEventKind::Down(
                                        MouseButton::Left,
                                    )
                                {
                                    current_theme =
                                        Theme::ALL[index];

                                    let scroll =
                                        view.scroll;

                                    view = rebuild_view(
                                        document,
                                        &area,
                                        current_theme,
                                        scroll,
                                    );

                                    menu_open = false;
                                }
                            }
                        }

                        continue;
                    }

                    // Close when leaving popup.
                    menu_open = false;
                }
            }

            Event::Resize(
                new_width,
                new_height,
            ) => {
                area = Area::new(
                    0,
                    1,
                    new_width,
                    new_height.saturating_sub(2),
                );

                view.resize(&area);
            }

            _ => {}
        }
    }

    Ok(())
}

fn handle_menu_key(
    key: KeyCode,
    document: &Document,
    view: &mut MadView,
    current_theme: &mut Theme,
    selected_theme: &mut usize,
    area: &Area,
    menu_open: &mut bool,
) -> Result<(), Box<dyn Error>> {
    match key {
        // Esc closes menu, not app.
        KeyCode::Esc => {
            *menu_open = false;
        }

        // Next theme.
        KeyCode::Down
        | KeyCode::Char('j') => {
            *selected_theme =
                (*selected_theme + 1)
                    % Theme::ALL.len();
        }

        // Previous theme.
        KeyCode::Up
        | KeyCode::Char('k') => {
            if *selected_theme == 0 {
                *selected_theme =
                    Theme::ALL.len() - 1;
            } else {
                *selected_theme -= 1;
            }
        }

        // Direct theme selection.
        KeyCode::Char('1')
        | KeyCode::Char('2')
        | KeyCode::Char('3')
        | KeyCode::Char('4')
        | KeyCode::Char('5')
        | KeyCode::Char('6')
        | KeyCode::Char('7')
        | KeyCode::Char('8')
        | KeyCode::Char('9') => {
            let index = match key {
                KeyCode::Char(c) => {
                    c.to_digit(10)
                        .unwrap_or(1) as usize
                        - 1
                }
                _ => 0,
            };

            if index < Theme::ALL.len() {
                *selected_theme = index;
                *current_theme =
                    Theme::ALL[index];

                let scroll = view.scroll;

                *view = rebuild_view(
                    document,
                    area,
                    *current_theme,
                    scroll,
                );

                *menu_open = false;
            }
        }

        // 0 = tenth theme.
        KeyCode::Char('0') => {
            let index = 9;

            *selected_theme = index;
            *current_theme =
                Theme::ALL[index];

            let scroll = view.scroll;

            *view = rebuild_view(
                document,
                area,
                *current_theme,
                scroll,
            );

            *menu_open = false;
        }

        // Apply currently selected theme.
        KeyCode::Enter => {
            *current_theme =
                Theme::ALL[*selected_theme];

            let scroll = view.scroll;

            *view = rebuild_view(
                document,
                area,
                *current_theme,
                scroll,
            );

            *menu_open = false;
        }

        _ => {}
    }

    Ok(())
}

fn rebuild_view(
    document: &Document,
    area: &Area,
    theme: Theme,
    scroll: usize,
) -> MadView {
    let mut view = MadView::from(
        document.content.clone(),
        area.clone(),
        theme.skin(),
    );

    view.scroll = scroll;

    view
}

fn render(
    document: &Document,
    view: &mut MadView,
    theme: Theme,
    menu_open: bool,
    selected_theme: usize,
    stdout: &mut Stdout,
) -> Result<(), Box<dyn Error>> {
    stdout.queue(
        Clear(ClearType::All)
    )?;

    view.write_on(stdout)?;

    let (width, height) =
        terminal::size()?;

    render_header(
        document,
        theme,
        menu_open,
        width,
        stdout,
    )?;

    render_status_bar(
        document,
        theme,
        height,
        width,
        stdout,
    )?;

    if menu_open {
        render_theme_menu(
            width,
            theme,
            selected_theme,
            stdout,
        )?;
    }

    stdout.flush()?;

    Ok(())
}

fn render_header(
    document: &Document,
    theme: Theme,
    menu_open: bool,
    width: u16,
    stdout: &mut Stdout,
) -> Result<(), Box<dyn Error>> {
    stdout.queue(
        MoveTo(0, 0)
    )?;

    stdout.queue(
        SetBackgroundColor(
            Color::Rgb {
                r: 35,
                g: 35,
                b: 45,
            },
        ),
    )?;

    stdout.queue(
        SetForegroundColor(
            Color::Rgb {
                r: 220,
                g: 220,
                b: 230,
            },
        ),
    )?;

    let mode = match &document.mode {
        DocumentMode::Markdown => {
            "Markdown".to_string()
        }

        DocumentMode::Converted(ext) => {
            format!(
                "{} → MD",
                ext.to_uppercase()
            )
        }
    };

    let left = format!(
        " markd │ {} │ {} │ {}",
        document.source_path,
        mode,
        theme.name(),
    );

    let gear = " [ ⚙ ] ";

    let left_len =
        left.chars().count();

    let gear_len = 7usize;

    let padding =
        (width as usize)
            .saturating_sub(
                left_len + gear_len
            );

    stdout.queue(
        Print(format!(
            "{}{}{}",
            left,
            " ".repeat(padding),
            gear
        )),
    )?;

    if menu_open && width >= 7 {
        let gear_x =
            width.saturating_sub(7);

        stdout.queue(
            MoveTo(gear_x, 0)
        )?;

        stdout.queue(
            SetBackgroundColor(
                Color::Rgb {
                    r: 86,
                    g: 156,
                    b: 214,
                },
            ),
        )?;

        stdout.queue(
            SetForegroundColor(
                Color::Black
            )
        )?;

        stdout.queue(Print(gear))?;
    }

    stdout.queue(ResetColor)?;

    Ok(())
}

fn render_status_bar(
    document: &Document,
    theme: Theme,
    height: u16,
    width: u16,
    stdout: &mut Stdout,
) -> Result<(), Box<dyn Error>> {
    let y =
        height.saturating_sub(1);

    stdout.queue(
        MoveTo(0, y)
    )?;

    stdout.queue(
        SetBackgroundColor(
            Color::Rgb {
                r: 30,
                g: 30,
                b: 38,
            },
        ),
    )?;

    stdout.queue(
        SetForegroundColor(
            Color::Rgb {
                r: 180,
                g: 180,
                b: 190,
            },
        ),
    )?;

    let filename =
        document
            .source_path
            .rsplit('/')
            .next()
            .unwrap_or(
                &document.source_path
            );

    let status = format!(
        " {} │ {} │ j/k scroll │ f/b page │ t themes │ Esc exit",
        filename,
        theme.name(),
    );

    let status_len =
        status.chars().count();

    let line = if status_len
        >= width as usize
    {
        status
            .chars()
            .take(width as usize)
            .collect::<String>()
    } else {
        format!(
            "{}{}",
            status,
            " ".repeat(
                width as usize - status_len
            )
        )
    };

    stdout.queue(Print(line))?;
    stdout.queue(ResetColor)?;

    Ok(())
}

fn render_theme_menu(
    width: u16,
    theme: Theme,
    selected: usize,
    stdout: &mut Stdout,
) -> Result<(), Box<dyn Error>> {
    let menu_w = 30u16;

    let x =
        width.saturating_sub(
            menu_w + 1
        );

    let y = 1u16;

    stdout.queue(
        SetBackgroundColor(
            Color::Rgb {
                r: 45,
                g: 45,
                b: 58,
            },
        ),
    )?;

    stdout.queue(
        SetForegroundColor(
            Color::White
        )
    )?;

    stdout.queue(
        MoveTo(x, y)
    )?;

    stdout.queue(Print(
        "┌────────────────────────────┐"
    ))?;

    stdout.queue(
        MoveTo(x, y + 1)
    )?;

    stdout.queue(Print(
        "│ Developer Themes           │"
    ))?;

    stdout.queue(
        MoveTo(x, y + 2)
    )?;

    stdout.queue(Print(
        "├────────────────────────────┤"
    ))?;

    for (index, available) in
        Theme::ALL.iter().enumerate()
    {
        let row =
            y + 3 + index as u16;

        stdout.queue(
            MoveTo(x, row)
        )?;

        let prefix =
            if index == selected {
                "› "
            } else {
                "  "
            };

        let check =
            if *available == theme {
                " ✓"
            } else {
                "  "
            };

        if index == selected {
            stdout.queue(
                SetForegroundColor(
                    Color::Cyan
                )
            )?;
        } else {
            stdout.queue(
                SetForegroundColor(
                    Color::White
                )
            )?;
        }

        let line = format!(
            "│{}{: <22}{}│",
            prefix,
            available.name(),
            check
        );

        stdout.queue(Print(line))?;
    }

    let bottom =
        y + 3 + Theme::ALL.len() as u16;

    stdout.queue(
        MoveTo(x, bottom)
    )?;

    stdout.queue(
        SetForegroundColor(
            Color::DarkGrey
        )
    )?;

    stdout.queue(Print(
        "├────────────────────────────┤"
    ))?;

    stdout.queue(
        MoveTo(x, bottom + 1)
    )?;

    stdout.queue(Print(
        "│ ↑↓ select   Enter apply    │"
    ))?;

    stdout.queue(
        MoveTo(x, bottom + 2)
    )?;

    stdout.queue(Print(
        "└────────────────────────────┘"
    ))?;

    stdout.queue(ResetColor)?;

    Ok(())
}

fn menu_geometry(
    width: u16,
) -> (u16, u16, u16, u16) {
    let menu_w = 30u16;
    let menu_h =
        8u16 + Theme::ALL.len() as u16;

    (
        width.saturating_sub(
            menu_w + 1
        ),
        1,
        menu_w,
        menu_h,
    )
}

fn inside(
    column: u16,
    row: u16,
    x: u16,
    y: u16,
    width: u16,
    height: u16,
) -> bool {
    column >= x
        && column < x.saturating_add(width)
        && row >= y
        && row < y.saturating_add(height)
}