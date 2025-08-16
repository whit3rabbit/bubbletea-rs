use std::collections::HashMap;

use bubbletea_rs::{quit, Cmd, KeyMsg, Model, Msg, Program, WindowSizeMsg};
use crossterm::event::{KeyCode, KeyModifiers};
use lipgloss_extras::lipgloss::{thick_border, Color, Style};
use lipgloss_extras::table::{Table, HEADER_ROW};

#[derive(Debug)]
struct AppModel {
    headers: Vec<String>,
    rows: Vec<Vec<String>>,
    type_colors: HashMap<String, Color>,
    dim_type_colors: HashMap<String, Color>,
    width: i32,
    height: i32,
    scroll_offset: usize,
}

impl Model for AppModel {
    fn init() -> (Self, Option<Cmd>) {
        let mut type_colors = HashMap::new();
        type_colors.insert("Bug".to_string(), Color::from("#D7FF87"));
        type_colors.insert("Electric".to_string(), Color::from("#FDFF90"));
        type_colors.insert("Fire".to_string(), Color::from("#FF7698"));
        type_colors.insert("Flying".to_string(), Color::from("#FF87D7"));
        type_colors.insert("Grass".to_string(), Color::from("#75FBAB"));
        type_colors.insert("Ground".to_string(), Color::from("#FF875F"));
        type_colors.insert("Normal".to_string(), Color::from("#929292"));
        type_colors.insert("Poison".to_string(), Color::from("#7D5AFC"));
        type_colors.insert("Water".to_string(), Color::from("#00E2C7"));

        let mut dim_type_colors = HashMap::new();
        dim_type_colors.insert("Bug".to_string(), Color::from("#97AD64"));
        dim_type_colors.insert("Electric".to_string(), Color::from("#FCFF5F"));
        dim_type_colors.insert("Fire".to_string(), Color::from("#BA5F75"));
        dim_type_colors.insert("Flying".to_string(), Color::from("#C97AB2"));
        dim_type_colors.insert("Grass".to_string(), Color::from("#59B980"));
        dim_type_colors.insert("Ground".to_string(), Color::from("#C77252"));
        dim_type_colors.insert("Normal".to_string(), Color::from("#727272"));
        dim_type_colors.insert("Poison".to_string(), Color::from("#634BD0"));
        dim_type_colors.insert("Water".to_string(), Color::from("#439F8E"));

        let headers = vec![
            "#".to_string(),
            "NAME".to_string(),
            "TYPE 1".to_string(),
            "TYPE 2".to_string(),
            "JAPANESE".to_string(),
            "OFFICIAL ROM.".to_string(),
        ];

        let rows = vec![
            vec![
                "1".to_string(),
                "Bulbasaur".to_string(),
                "Grass".to_string(),
                "Poison".to_string(),
                "フシギダネ".to_string(),
                "Bulbasaur".to_string(),
            ],
            vec![
                "2".to_string(),
                "Ivysaur".to_string(),
                "Grass".to_string(),
                "Poison".to_string(),
                "フシギソウ".to_string(),
                "Ivysaur".to_string(),
            ],
            vec![
                "3".to_string(),
                "Venusaur".to_string(),
                "Grass".to_string(),
                "Poison".to_string(),
                "フシギバナ".to_string(),
                "Venusaur".to_string(),
            ],
            vec![
                "4".to_string(),
                "Charmander".to_string(),
                "Fire".to_string(),
                "".to_string(),
                "ヒトカゲ".to_string(),
                "Hitokage".to_string(),
            ],
            vec![
                "5".to_string(),
                "Charmeleon".to_string(),
                "Fire".to_string(),
                "".to_string(),
                "リザード".to_string(),
                "Lizardo".to_string(),
            ],
            vec![
                "6".to_string(),
                "Charizard".to_string(),
                "Fire".to_string(),
                "Flying".to_string(),
                "リザードン".to_string(),
                "Lizardon".to_string(),
            ],
            vec![
                "7".to_string(),
                "Squirtle".to_string(),
                "Water".to_string(),
                "".to_string(),
                "ゼニガメ".to_string(),
                "Zenigame".to_string(),
            ],
            vec![
                "8".to_string(),
                "Wartortle".to_string(),
                "Water".to_string(),
                "".to_string(),
                "カメール".to_string(),
                "Kameil".to_string(),
            ],
            vec![
                "9".to_string(),
                "Blastoise".to_string(),
                "Water".to_string(),
                "".to_string(),
                "カメックス".to_string(),
                "Kamex".to_string(),
            ],
            vec![
                "10".to_string(),
                "Caterpie".to_string(),
                "Bug".to_string(),
                "".to_string(),
                "キャタピー".to_string(),
                "Caterpie".to_string(),
            ],
            vec![
                "11".to_string(),
                "Metapod".to_string(),
                "Bug".to_string(),
                "".to_string(),
                "トランセル".to_string(),
                "Trancell".to_string(),
            ],
            vec![
                "12".to_string(),
                "Butterfree".to_string(),
                "Bug".to_string(),
                "Flying".to_string(),
                "バタフリー".to_string(),
                "Butterfree".to_string(),
            ],
            vec![
                "13".to_string(),
                "Weedle".to_string(),
                "Bug".to_string(),
                "Poison".to_string(),
                "ビードル".to_string(),
                "Beedle".to_string(),
            ],
            vec![
                "14".to_string(),
                "Kakuna".to_string(),
                "Bug".to_string(),
                "Poison".to_string(),
                "コクーン".to_string(),
                "Cocoon".to_string(),
            ],
            vec![
                "15".to_string(),
                "Beedrill".to_string(),
                "Bug".to_string(),
                "Poison".to_string(),
                "スピアー".to_string(),
                "Spear".to_string(),
            ],
            vec![
                "16".to_string(),
                "Pidgey".to_string(),
                "Normal".to_string(),
                "Flying".to_string(),
                "ポッポ".to_string(),
                "Poppo".to_string(),
            ],
            vec![
                "17".to_string(),
                "Pidgeotto".to_string(),
                "Normal".to_string(),
                "Flying".to_string(),
                "ピジョン".to_string(),
                "Pigeon".to_string(),
            ],
            vec![
                "18".to_string(),
                "Pidgeot".to_string(),
                "Normal".to_string(),
                "Flying".to_string(),
                "ピジョット".to_string(),
                "Pigeot".to_string(),
            ],
            vec![
                "19".to_string(),
                "Rattata".to_string(),
                "Normal".to_string(),
                "".to_string(),
                "コラッタ".to_string(),
                "Koratta".to_string(),
            ],
            vec![
                "20".to_string(),
                "Raticate".to_string(),
                "Normal".to_string(),
                "".to_string(),
                "ラッタ".to_string(),
                "Ratta".to_string(),
            ],
            vec![
                "21".to_string(),
                "Spearow".to_string(),
                "Normal".to_string(),
                "Flying".to_string(),
                "オニスズメ".to_string(),
                "Onisuzume".to_string(),
            ],
            vec![
                "22".to_string(),
                "Fearow".to_string(),
                "Normal".to_string(),
                "Flying".to_string(),
                "オニドリル".to_string(),
                "Onidrill".to_string(),
            ],
            vec![
                "23".to_string(),
                "Ekans".to_string(),
                "Poison".to_string(),
                "".to_string(),
                "アーボ".to_string(),
                "Arbo".to_string(),
            ],
            vec![
                "24".to_string(),
                "Arbok".to_string(),
                "Poison".to_string(),
                "".to_string(),
                "アーボック".to_string(),
                "Arbok".to_string(),
            ],
            vec![
                "25".to_string(),
                "Pikachu".to_string(),
                "Electric".to_string(),
                "".to_string(),
                "ピカチュウ".to_string(),
                "Pikachu".to_string(),
            ],
            vec![
                "26".to_string(),
                "Raichu".to_string(),
                "Electric".to_string(),
                "".to_string(),
                "ライチュウ".to_string(),
                "Raichu".to_string(),
            ],
            vec![
                "27".to_string(),
                "Sandshrew".to_string(),
                "Ground".to_string(),
                "".to_string(),
                "サンド".to_string(),
                "Sand".to_string(),
            ],
            vec![
                "28".to_string(),
                "Sandslash".to_string(),
                "Ground".to_string(),
                "".to_string(),
                "サンドパン".to_string(),
                "Sandpan".to_string(),
            ],
        ];

        let model = AppModel {
            headers,
            rows,
            type_colors,
            dim_type_colors,
            width: 80,  // Default width
            height: 24, // Default height
            scroll_offset: 0,
        };

        (model, None)
    }

    fn update(&mut self, msg: Msg) -> Option<Cmd> {
        if let Some(key_msg) = msg.downcast_ref::<KeyMsg>() {
            match key_msg.key {
                KeyCode::Char('q') | KeyCode::Esc => return Some(quit()),
                KeyCode::Char('c') if key_msg.modifiers.contains(KeyModifiers::CONTROL) => {
                    return Some(quit())
                }
                KeyCode::Up | KeyCode::Char('k') => {
                    if self.scroll_offset > 0 {
                        self.scroll_offset -= 1;
                    }
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    let available_height = (self.height - 4).max(3) as usize;
                    let max_visible_rows = available_height.saturating_sub(1);
                    if self.scroll_offset + max_visible_rows < self.rows.len() {
                        self.scroll_offset += 1;
                    }
                }
                KeyCode::Home => {
                    self.scroll_offset = 0;
                }
                KeyCode::End => {
                    let available_height = (self.height - 4).max(3) as usize;
                    let max_visible_rows = available_height.saturating_sub(1);
                    self.scroll_offset = self.rows.len().saturating_sub(max_visible_rows);
                }
                _ => {}
            }
        }

        if let Some(size_msg) = msg.downcast_ref::<WindowSizeMsg>() {
            self.width = size_msg.width as i32;
            self.height = size_msg.height as i32;
        }

        None
    }

    fn view(&self) -> String {
        let base_style = Style::new().padding(0, 1, 0, 1);
        let header_style = base_style.clone().foreground(Color::from("252")).bold(true);
        let selected_style = base_style
            .clone()
            .foreground(Color::from("#01BE85"))
            .background(Color::from("#00432F"));

        // Calculate how many rows can fit in the terminal
        // Account for header, borders, and some padding
        let available_height = (self.height - 4).max(3) as usize; // Reserve space for borders
        let max_visible_rows = available_height.saturating_sub(1); // Reserve space for header

        // Determine which rows to show based on scroll offset
        let visible_rows: Vec<Vec<String>> = if self.rows.len() <= max_visible_rows {
            // All rows fit, show everything
            self.rows.clone()
        } else {
            // Need to scroll, show rows based on scroll offset
            let start_idx = self.scroll_offset;
            let end_idx = (start_idx + max_visible_rows).min(self.rows.len());
            self.rows[start_idx..end_idx].to_vec()
        };

        // Build table with styling
        let mut table = Table::new()
            .headers(self.headers.iter().map(|s| s.as_str()).collect::<Vec<_>>())
            .rows(
                visible_rows
                    .iter()
                    .map(|row| row.iter().map(|s| s.as_str()).collect::<Vec<_>>())
                    .collect::<Vec<_>>(),
            )
            .border(thick_border())
            .width(self.width)
            .style_func_boxed({
                let rows = visible_rows.clone();
                let type_colors = self.type_colors.clone();
                let dim_type_colors = self.dim_type_colors.clone();

                Box::new(move |row, col| {
                    if row == HEADER_ROW {
                        return header_style.clone();
                    }

                    let row_index = row as usize;
                    if row_index >= rows.len() {
                        return base_style.clone();
                    }

                    // Check if this is Pikachu
                    if rows[row_index][1] == "Pikachu" {
                        return selected_style.clone();
                    }

                    let even = (row + 1) % 2 == 0;

                    // Handle type columns (TYPE 1 and TYPE 2)
                    if col == 2 || col == 3 {
                        let col_index = col as usize;
                        if col_index < rows[row_index].len() {
                            let type_name = &rows[row_index][col_index];
                            if !type_name.is_empty() {
                                let colors = if even { &dim_type_colors } else { &type_colors };
                                if let Some(color) = colors.get(type_name) {
                                    return base_style.clone().foreground(color.clone());
                                }
                            }
                        }
                    }

                    // Regular row coloring
                    if even {
                        base_style.clone().foreground(Color::from("245"))
                    } else {
                        base_style.clone().foreground(Color::from("252"))
                    }
                })
            });

        let table_output = table.render();

        // Add navigation help and scroll indicator
        let total_rows = self.rows.len();
        let showing_start = self.scroll_offset + 1;
        let showing_end = (self.scroll_offset + visible_rows.len()).min(total_rows);

        let help_text = if total_rows > max_visible_rows {
            format!(
                "Showing rows {}-{} of {} | ↑/k: up, ↓/j: down, Home: top, End: bottom, q: quit",
                showing_start, showing_end, total_rows
            )
        } else {
            "All rows visible | q: quit".to_string()
        };

        format!("{}\n{}", table_output, help_text)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create and run the program
    let program = Program::<AppModel>::builder()
        .alt_screen(true) // Use alternate screen for cleaner display
        .signal_handler(true) // Enable Ctrl+C handling
        .build()?;

    // Run the program and handle errors
    if let Err(err) = program.run().await {
        match err {
            bubbletea_rs::Error::Interrupted => {
                std::process::exit(130);
            }
            bubbletea_rs::Error::ProgramKilled => {
                std::process::exit(1);
            }
            _ => {
                eprintln!("Error: {}", err);
                std::process::exit(1);
            }
        }
    }

    Ok(())
}
