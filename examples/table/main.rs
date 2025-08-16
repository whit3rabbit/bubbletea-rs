use bubbletea_rs::{quit, Cmd, KeyMsg, Model, Msg, Program, WindowSizeMsg};
use crossterm::event::{KeyCode, KeyModifiers};
use lipgloss_extras::lipgloss::{normal_border, Color, Style};
use lipgloss_extras::table::{Table, HEADER_ROW};

// Synthetic message used to trigger the initial render immediately after startup.
#[derive(Debug)]
struct InitRenderMsg;

fn init_render_cmd() -> Cmd {
    Box::pin(async { Some(Box::new(InitRenderMsg) as Msg) })
}

#[derive(Debug)]
struct AppModel {
    headers: Vec<String>,
    rows: Vec<Vec<String>>,
    width: i32,
    height: i32,
    cursor: usize,
    focused: bool,
    message: Option<String>,
}

impl Model for AppModel {
    fn init() -> (Self, Option<Cmd>) {
        let headers = vec![
            "Rank".to_string(),
            "City".to_string(),
            "Country".to_string(),
            "Population".to_string(),
        ];

        let rows = vec![
            vec![
                "1".to_string(),
                "Tokyo".to_string(),
                "Japan".to_string(),
                "37,274,000".to_string(),
            ],
            vec![
                "2".to_string(),
                "Delhi".to_string(),
                "India".to_string(),
                "32,065,760".to_string(),
            ],
            vec![
                "3".to_string(),
                "Shanghai".to_string(),
                "China".to_string(),
                "28,516,904".to_string(),
            ],
            vec![
                "4".to_string(),
                "Dhaka".to_string(),
                "Bangladesh".to_string(),
                "22,478,116".to_string(),
            ],
            vec![
                "5".to_string(),
                "São Paulo".to_string(),
                "Brazil".to_string(),
                "22,429,800".to_string(),
            ],
            vec![
                "6".to_string(),
                "Mexico City".to_string(),
                "Mexico".to_string(),
                "22,085,140".to_string(),
            ],
            vec![
                "7".to_string(),
                "Cairo".to_string(),
                "Egypt".to_string(),
                "21,750,020".to_string(),
            ],
            vec![
                "8".to_string(),
                "Beijing".to_string(),
                "China".to_string(),
                "21,333,332".to_string(),
            ],
            vec![
                "9".to_string(),
                "Mumbai".to_string(),
                "India".to_string(),
                "20,961,472".to_string(),
            ],
            vec![
                "10".to_string(),
                "Osaka".to_string(),
                "Japan".to_string(),
                "19,059,856".to_string(),
            ],
            vec![
                "11".to_string(),
                "Chongqing".to_string(),
                "China".to_string(),
                "16,874,740".to_string(),
            ],
            vec![
                "12".to_string(),
                "Karachi".to_string(),
                "Pakistan".to_string(),
                "16,839,950".to_string(),
            ],
            vec![
                "13".to_string(),
                "Istanbul".to_string(),
                "Turkey".to_string(),
                "15,636,243".to_string(),
            ],
            vec![
                "14".to_string(),
                "Kinshasa".to_string(),
                "DR Congo".to_string(),
                "15,628,085".to_string(),
            ],
            vec![
                "15".to_string(),
                "Lagos".to_string(),
                "Nigeria".to_string(),
                "15,387,639".to_string(),
            ],
            vec![
                "16".to_string(),
                "Buenos Aires".to_string(),
                "Argentina".to_string(),
                "15,369,919".to_string(),
            ],
            vec![
                "17".to_string(),
                "Kolkata".to_string(),
                "India".to_string(),
                "15,133,888".to_string(),
            ],
            vec![
                "18".to_string(),
                "Manila".to_string(),
                "Philippines".to_string(),
                "14,406,059".to_string(),
            ],
            vec![
                "19".to_string(),
                "Tianjin".to_string(),
                "China".to_string(),
                "14,011,828".to_string(),
            ],
            vec![
                "20".to_string(),
                "Guangzhou".to_string(),
                "China".to_string(),
                "13,964,637".to_string(),
            ],
            vec![
                "21".to_string(),
                "Rio De Janeiro".to_string(),
                "Brazil".to_string(),
                "13,634,274".to_string(),
            ],
            vec![
                "22".to_string(),
                "Lahore".to_string(),
                "Pakistan".to_string(),
                "13,541,764".to_string(),
            ],
            vec![
                "23".to_string(),
                "Bangalore".to_string(),
                "India".to_string(),
                "13,193,035".to_string(),
            ],
            vec![
                "24".to_string(),
                "Shenzhen".to_string(),
                "China".to_string(),
                "12,831,330".to_string(),
            ],
            vec![
                "25".to_string(),
                "Moscow".to_string(),
                "Russia".to_string(),
                "12,640,818".to_string(),
            ],
            vec![
                "26".to_string(),
                "Chennai".to_string(),
                "India".to_string(),
                "11,503,293".to_string(),
            ],
            vec![
                "27".to_string(),
                "Bogota".to_string(),
                "Colombia".to_string(),
                "11,344,312".to_string(),
            ],
            vec![
                "28".to_string(),
                "Paris".to_string(),
                "France".to_string(),
                "11,142,303".to_string(),
            ],
            vec![
                "29".to_string(),
                "Jakarta".to_string(),
                "Indonesia".to_string(),
                "11,074,811".to_string(),
            ],
            vec![
                "30".to_string(),
                "Lima".to_string(),
                "Peru".to_string(),
                "11,044,607".to_string(),
            ],
            vec![
                "31".to_string(),
                "Bangkok".to_string(),
                "Thailand".to_string(),
                "10,899,698".to_string(),
            ],
            vec![
                "32".to_string(),
                "Hyderabad".to_string(),
                "India".to_string(),
                "10,534,418".to_string(),
            ],
            vec![
                "33".to_string(),
                "Seoul".to_string(),
                "South Korea".to_string(),
                "9,975,709".to_string(),
            ],
            vec![
                "34".to_string(),
                "Nagoya".to_string(),
                "Japan".to_string(),
                "9,571,596".to_string(),
            ],
            vec![
                "35".to_string(),
                "London".to_string(),
                "United Kingdom".to_string(),
                "9,540,576".to_string(),
            ],
            vec![
                "36".to_string(),
                "Chengdu".to_string(),
                "China".to_string(),
                "9,478,521".to_string(),
            ],
            vec![
                "37".to_string(),
                "Nanjing".to_string(),
                "China".to_string(),
                "9,429,381".to_string(),
            ],
            vec![
                "38".to_string(),
                "Tehran".to_string(),
                "Iran".to_string(),
                "9,381,546".to_string(),
            ],
            vec![
                "39".to_string(),
                "Ho Chi Minh City".to_string(),
                "Vietnam".to_string(),
                "9,077,158".to_string(),
            ],
            vec![
                "40".to_string(),
                "Luanda".to_string(),
                "Angola".to_string(),
                "8,952,496".to_string(),
            ],
            vec![
                "41".to_string(),
                "Wuhan".to_string(),
                "China".to_string(),
                "8,591,611".to_string(),
            ],
            vec![
                "42".to_string(),
                "Xi An Shaanxi".to_string(),
                "China".to_string(),
                "8,537,646".to_string(),
            ],
            vec![
                "43".to_string(),
                "Ahmedabad".to_string(),
                "India".to_string(),
                "8,450,228".to_string(),
            ],
            vec![
                "44".to_string(),
                "Kuala Lumpur".to_string(),
                "Malaysia".to_string(),
                "8,419,566".to_string(),
            ],
            vec![
                "45".to_string(),
                "New York City".to_string(),
                "United States".to_string(),
                "8,177,020".to_string(),
            ],
            vec![
                "46".to_string(),
                "Hangzhou".to_string(),
                "China".to_string(),
                "8,044,878".to_string(),
            ],
            vec![
                "47".to_string(),
                "Surat".to_string(),
                "India".to_string(),
                "7,784,276".to_string(),
            ],
            vec![
                "48".to_string(),
                "Suzhou".to_string(),
                "China".to_string(),
                "7,764,499".to_string(),
            ],
            vec![
                "49".to_string(),
                "Hong Kong".to_string(),
                "Hong Kong".to_string(),
                "7,643,256".to_string(),
            ],
            vec![
                "50".to_string(),
                "Riyadh".to_string(),
                "Saudi Arabia".to_string(),
                "7,538,200".to_string(),
            ],
            vec![
                "51".to_string(),
                "Shenyang".to_string(),
                "China".to_string(),
                "7,527,975".to_string(),
            ],
            vec![
                "52".to_string(),
                "Baghdad".to_string(),
                "Iraq".to_string(),
                "7,511,920".to_string(),
            ],
            vec![
                "53".to_string(),
                "Dongguan".to_string(),
                "China".to_string(),
                "7,511,851".to_string(),
            ],
            vec![
                "54".to_string(),
                "Foshan".to_string(),
                "China".to_string(),
                "7,497,263".to_string(),
            ],
            vec![
                "55".to_string(),
                "Dar Es Salaam".to_string(),
                "Tanzania".to_string(),
                "7,404,689".to_string(),
            ],
            vec![
                "56".to_string(),
                "Pune".to_string(),
                "India".to_string(),
                "6,987,077".to_string(),
            ],
            vec![
                "57".to_string(),
                "Santiago".to_string(),
                "Chile".to_string(),
                "6,856,939".to_string(),
            ],
            vec![
                "58".to_string(),
                "Madrid".to_string(),
                "Spain".to_string(),
                "6,713,557".to_string(),
            ],
            vec![
                "59".to_string(),
                "Haerbin".to_string(),
                "China".to_string(),
                "6,665,951".to_string(),
            ],
            vec![
                "60".to_string(),
                "Toronto".to_string(),
                "Canada".to_string(),
                "6,312,974".to_string(),
            ],
            vec![
                "61".to_string(),
                "Belo Horizonte".to_string(),
                "Brazil".to_string(),
                "6,194,292".to_string(),
            ],
            vec![
                "62".to_string(),
                "Khartoum".to_string(),
                "Sudan".to_string(),
                "6,160,327".to_string(),
            ],
            vec![
                "63".to_string(),
                "Johannesburg".to_string(),
                "South Africa".to_string(),
                "6,065,354".to_string(),
            ],
            vec![
                "64".to_string(),
                "Singapore".to_string(),
                "Singapore".to_string(),
                "6,039,577".to_string(),
            ],
            vec![
                "65".to_string(),
                "Dalian".to_string(),
                "China".to_string(),
                "5,930,140".to_string(),
            ],
            vec![
                "66".to_string(),
                "Qingdao".to_string(),
                "China".to_string(),
                "5,865,232".to_string(),
            ],
            vec![
                "67".to_string(),
                "Zhengzhou".to_string(),
                "China".to_string(),
                "5,690,312".to_string(),
            ],
            vec![
                "68".to_string(),
                "Ji Nan Shandong".to_string(),
                "China".to_string(),
                "5,663,015".to_string(),
            ],
            vec![
                "69".to_string(),
                "Barcelona".to_string(),
                "Spain".to_string(),
                "5,658,472".to_string(),
            ],
            vec![
                "70".to_string(),
                "Saint Petersburg".to_string(),
                "Russia".to_string(),
                "5,535,556".to_string(),
            ],
            vec![
                "71".to_string(),
                "Abidjan".to_string(),
                "Ivory Coast".to_string(),
                "5,515,790".to_string(),
            ],
            vec![
                "72".to_string(),
                "Yangon".to_string(),
                "Myanmar".to_string(),
                "5,514,454".to_string(),
            ],
            vec![
                "73".to_string(),
                "Fukuoka".to_string(),
                "Japan".to_string(),
                "5,502,591".to_string(),
            ],
            vec![
                "74".to_string(),
                "Alexandria".to_string(),
                "Egypt".to_string(),
                "5,483,605".to_string(),
            ],
            vec![
                "75".to_string(),
                "Guadalajara".to_string(),
                "Mexico".to_string(),
                "5,339,583".to_string(),
            ],
            vec![
                "76".to_string(),
                "Ankara".to_string(),
                "Turkey".to_string(),
                "5,309,690".to_string(),
            ],
            vec![
                "77".to_string(),
                "Chittagong".to_string(),
                "Bangladesh".to_string(),
                "5,252,842".to_string(),
            ],
            vec![
                "78".to_string(),
                "Addis Ababa".to_string(),
                "Ethiopia".to_string(),
                "5,227,794".to_string(),
            ],
            vec![
                "79".to_string(),
                "Melbourne".to_string(),
                "Australia".to_string(),
                "5,150,766".to_string(),
            ],
            vec![
                "80".to_string(),
                "Nairobi".to_string(),
                "Kenya".to_string(),
                "5,118,844".to_string(),
            ],
            vec![
                "81".to_string(),
                "Hanoi".to_string(),
                "Vietnam".to_string(),
                "5,067,352".to_string(),
            ],
            vec![
                "82".to_string(),
                "Sydney".to_string(),
                "Australia".to_string(),
                "5,056,571".to_string(),
            ],
            vec![
                "83".to_string(),
                "Monterrey".to_string(),
                "Mexico".to_string(),
                "5,036,535".to_string(),
            ],
            vec![
                "84".to_string(),
                "Changsha".to_string(),
                "China".to_string(),
                "4,809,887".to_string(),
            ],
            vec![
                "85".to_string(),
                "Brasilia".to_string(),
                "Brazil".to_string(),
                "4,803,877".to_string(),
            ],
            vec![
                "86".to_string(),
                "Cape Town".to_string(),
                "South Africa".to_string(),
                "4,800,954".to_string(),
            ],
            vec![
                "87".to_string(),
                "Jiddah".to_string(),
                "Saudi Arabia".to_string(),
                "4,780,740".to_string(),
            ],
            vec![
                "88".to_string(),
                "Urumqi".to_string(),
                "China".to_string(),
                "4,710,203".to_string(),
            ],
            vec![
                "89".to_string(),
                "Kunming".to_string(),
                "China".to_string(),
                "4,657,381".to_string(),
            ],
            vec![
                "90".to_string(),
                "Changchun".to_string(),
                "China".to_string(),
                "4,616,002".to_string(),
            ],
            vec![
                "91".to_string(),
                "Hefei".to_string(),
                "China".to_string(),
                "4,496,456".to_string(),
            ],
            vec![
                "92".to_string(),
                "Shantou".to_string(),
                "China".to_string(),
                "4,490,411".to_string(),
            ],
            vec![
                "93".to_string(),
                "Xinbei".to_string(),
                "Taiwan".to_string(),
                "4,470,672".to_string(),
            ],
            vec![
                "94".to_string(),
                "Kabul".to_string(),
                "Afghanistan".to_string(),
                "4,457,882".to_string(),
            ],
            vec![
                "95".to_string(),
                "Ningbo".to_string(),
                "China".to_string(),
                "4,405,292".to_string(),
            ],
            vec![
                "96".to_string(),
                "Tel Aviv".to_string(),
                "Israel".to_string(),
                "4,343,584".to_string(),
            ],
            vec![
                "97".to_string(),
                "Yaounde".to_string(),
                "Cameroon".to_string(),
                "4,336,670".to_string(),
            ],
            vec![
                "98".to_string(),
                "Rome".to_string(),
                "Italy".to_string(),
                "4,297,877".to_string(),
            ],
            vec![
                "99".to_string(),
                "Shijiazhuang".to_string(),
                "China".to_string(),
                "4,285,135".to_string(),
            ],
            vec![
                "100".to_string(),
                "Montreal".to_string(),
                "Canada".to_string(),
                "4,276,526".to_string(),
            ],
        ];

        let model = AppModel {
            headers,
            rows,
            width: 80,
            height: 24,
            cursor: 0,
            focused: true,
            message: None,
        };
        (model, Some(init_render_cmd()))
    }

    fn update(&mut self, msg: Msg) -> Option<Cmd> {
        // Handle initial render message
        if msg.downcast_ref::<InitRenderMsg>().is_some() {
            return None; // Just trigger a render, no command needed
        }

        if let Some(key_msg) = msg.downcast_ref::<KeyMsg>() {
            // If a message is showing, any key should quit
            if self.message.is_some() {
                return Some(quit());
            }

            match key_msg.key {
                KeyCode::Char('q') => return Some(quit()),
                KeyCode::Char('c') if key_msg.modifiers.contains(KeyModifiers::CONTROL) => {
                    return Some(quit())
                }
                KeyCode::Esc => {
                    self.focused = !self.focused;
                }
                KeyCode::Enter => {
                    if self.cursor < self.rows.len() && self.rows[self.cursor].len() >= 2 {
                        // Show message instead of immediately quitting
                        self.message = Some(format!(
                            "Let's go to {}! Press any key to exit.",
                            self.rows[self.cursor][1]
                        ));
                    }
                }
                KeyCode::Up | KeyCode::Char('k') => {
                    if self.focused && self.cursor > 0 {
                        self.cursor -= 1;
                    }
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    if self.focused && self.cursor < self.rows.len().saturating_sub(1) {
                        self.cursor += 1;
                    }
                }
                KeyCode::Home => {
                    if self.focused {
                        self.cursor = 0;
                    }
                }
                KeyCode::End => {
                    if self.focused {
                        self.cursor = self.rows.len().saturating_sub(1);
                    }
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
        // If we have a message, display it instead of the table
        if let Some(ref message) = self.message {
            let message_style = Style::new()
                .foreground(Color::from("229"))
                .background(Color::from("57"))
                .padding(1, 2, 1, 2)
                .border(normal_border())
                .border_foreground(Color::from("240"));
            return message_style.render(message);
        }

        let base_style = Style::new().padding(0, 1, 0, 1);
        let header_style = base_style.clone().bold(false);
        let selected_style = base_style
            .clone()
            .foreground(Color::from("229"))
            .background(Color::from("57"));
        let normal_style = base_style.clone();

        // Calculate visible rows (7 rows as in Go example)
        let visible_height = 7;
        let start_idx = if self.cursor >= visible_height {
            self.cursor - visible_height + 1
        } else {
            0
        };
        let end_idx = (start_idx + visible_height).min(self.rows.len());
        let visible_rows = &self.rows[start_idx..end_idx];

        // Build table with styling
        let mut table = Table::new()
            .headers(self.headers.iter().map(|s| s.as_str()).collect::<Vec<_>>())
            .rows(
                visible_rows
                    .iter()
                    .map(|row| row.iter().map(|s| s.as_str()).collect::<Vec<_>>())
                    .collect::<Vec<_>>(),
            )
            .border(normal_border())
            .width(self.width)
            .style_func_boxed({
                let cursor_in_visible_range = self.cursor >= start_idx && self.cursor < end_idx;
                let cursor_relative = if cursor_in_visible_range {
                    Some(self.cursor - start_idx)
                } else {
                    None
                };
                let focused = self.focused;

                Box::new(move |row, _col| {
                    if row == HEADER_ROW {
                        return header_style.clone();
                    }

                    let row_index = row as usize;
                    if focused && cursor_relative == Some(row_index) {
                        return selected_style.clone();
                    }

                    normal_style.clone()
                })
            });

        let table_output = table.render();

        // Base style with border to match Go example
        let base_border_style = Style::new()
            .border(normal_border())
            .border_foreground(Color::from("240"));

        base_border_style.render(&table_output) + "\n"
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let program = Program::<AppModel>::builder().alt_screen(true).build()?;

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
