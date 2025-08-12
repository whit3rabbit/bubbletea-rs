use bubbletea_rs::{window_size, Cmd, KeyMsg, Model as BubbleTeaModel, Msg, Program, WindowSizeMsg};
use bubbletea_widgets::list::{Item, ItemDelegate, Model as List};
use bubbletea_widgets::key::{new_binding, with_keys_str, with_help, matches_binding, Binding, KeyMap};
use bubbletea_widgets::help::{Model as HelpModel, KeyMap as HelpKeyMap};
use bubbletea_widgets::paginator::Type as PaginatorType;
use lipgloss_extras::lipgloss::{Color, Style};
use rand::{seq::SliceRandom, thread_rng};
use std::fmt::Display;
use std::sync::{Arc, Mutex};

// Synthetic message used to trigger the initial render immediately after startup.
struct InitRenderMsg;

fn init_render_cmd() -> Cmd {
    Box::pin(async { Some(Box::new(InitRenderMsg) as Msg) })
}

// Status message for the list
struct StatusMessage(String);

fn status_message_cmd(msg: String) -> Cmd {
    Box::pin(async move {
        Some(Box::new(StatusMessage(msg)) as Msg)
    })
}

// Spinner animation not yet implemented in current bubbletea-widgets version

// App styles matching Go version
fn app_style() -> Style {
    Style::new().padding(1, 2, 1, 2)
}

fn title_style() -> Style {
    Style::new()
        .foreground(Color::from("#FFFDF5"))
        .background(Color::from("#25A065"))
        .padding(0, 1, 0, 1)
}

fn status_message_style() -> Style {
    Style::new()
        .foreground(Color::from("#04B575"))
}

// Item struct matching Go version
#[derive(Debug, Clone)]
pub struct GroceryItem {
    title: String,
    description: String,
}

impl GroceryItem {
    fn new(title: String, description: String) -> Self {
        Self { title, description }
    }
}

impl Display for GroceryItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.title)
    }
}

impl Item for GroceryItem {
    fn filter_value(&self) -> String {
        self.title.clone()
    }
}

// Random item generator matching Go version
#[derive(Debug, Clone)]
pub struct RandomItemGenerator {
    titles: Vec<String>,
    descs: Vec<String>,
    title_index: usize,
    desc_index: usize,
}

impl RandomItemGenerator {
    fn new() -> Self {
        let mut titles: Vec<String> = vec![
            "Artichoke", "Baking Flour", "Bananas", "Barley", "Bean Sprouts",
            "Bitter Melon", "Black Cod", "Blood Orange", "Brown Sugar", "Cashew Apple",
            "Cashews", "Cat Food", "Coconut Milk", "Cucumber", "Curry Paste",
            "Currywurst", "Dill", "Dragonfruit", "Dried Shrimp", "Eggs",
            "Fish Cake", "Furikake", "Garlic", "Gherkin", "Ginger",
            "Granulated Sugar", "Grapefruit", "Green Onion", "Hazelnuts", "Heavy whipping cream",
            "Honey Dew", "Horseradish", "Jicama", "Kohlrabi", "Leeks",
            "Lentils", "Licorice Root", "Meyer Lemons", "Milk", "Molasses",
            "Muesli", "Nectarine", "Niagamo Root", "Nopal", "Nutella",
            "Oat Milk", "Oatmeal", "Olives", "Papaya", "Party Gherkin",
            "Peppers", "Persian Lemons", "Pickle", "Pineapple", "Plantains",
            "Pocky", "Powdered Sugar", "Quince", "Radish", "Ramps",
            "Star Anise", "Sweet Potato", "Tamarind", "Unsalted Butter", "Watermelon",
            "Weißwurst", "Yams", "Yeast", "Yuzu", "Snow Peas",
        ].into_iter().map(String::from).collect();

        let mut descs: Vec<String> = vec![
            "A little weird", "Bold flavor", "Can't get enough", "Delectable", "Expensive",
            "Expired", "Exquisite", "Fresh", "Gimme", "In season",
            "Kind of spicy", "Looks fresh", "Looks good to me", "Maybe not", "My favorite",
            "Oh my", "On sale", "Organic", "Questionable", "Really fresh",
            "Refreshing", "Salty", "Scrumptious", "Delectable", "Slightly sweet",
            "Smells great", "Tasty", "Too ripe", "At last", "What?",
            "Wow", "Yum", "Maybe", "Sure, why not?",
        ].into_iter().map(String::from).collect();

        // Shuffle both arrays once
        let mut rng = thread_rng();
        titles.shuffle(&mut rng);
        descs.shuffle(&mut rng);

        Self {
            titles,
            descs,
            title_index: 0,
            desc_index: 0,
        }
    }

    fn next(&mut self) -> GroceryItem {
        let item = GroceryItem::new(
            self.titles[self.title_index].clone(),
            self.descs[self.desc_index].clone(),
        );

        self.title_index = (self.title_index + 1) % self.titles.len();
        self.desc_index = (self.desc_index + 1) % self.descs.len();

        item
    }
}

// Delegate key bindings
#[derive(Debug, Clone)]
pub struct DelegateKeyMap {
    pub choose: Binding,
    pub remove: Binding,
}

impl Default for DelegateKeyMap {
    fn default() -> Self {
        Self {
            choose: new_binding(vec![
                with_keys_str(&["enter"]),
                with_help("enter", "choose"),
            ]),
            remove: new_binding(vec![
                with_keys_str(&["x", "backspace"]),
                with_help("x", "delete"),
            ]),
        }
    }
}

impl KeyMap for DelegateKeyMap {
    fn short_help(&self) -> Vec<&Binding> {
        vec![&self.choose, &self.remove]
    }

    fn full_help(&self) -> Vec<Vec<&Binding>> {
        vec![vec![&self.choose, &self.remove]]
    }
}

// Custom delegate with enhanced features
#[derive(Debug, Clone)]
pub struct FancyDelegate {
    keys: Arc<Mutex<DelegateKeyMap>>,
}

impl FancyDelegate {
    fn new() -> Self {
        Self {
            keys: Arc::new(Mutex::new(DelegateKeyMap::default())),
        }
    }
}

impl ItemDelegate<GroceryItem> for FancyDelegate {
    fn render(&self, m: &List<GroceryItem>, index: usize, item: &GroceryItem) -> String {
        let cursor = if index == m.cursor() { "•" } else { " " };
        format!("{} {}\n  {}", cursor, item.title, item.description)
    }

    fn height(&self) -> usize {
        2 // Title + description
    }

    fn spacing(&self) -> usize {
        1 // Space between items
    }

    fn update(&self, msg: &Msg, m: &mut List<GroceryItem>) -> Option<Cmd> {
        if let Some(key_msg) = msg.downcast_ref::<KeyMsg>() {
            let keys = self.keys.lock().unwrap();
            
            // Handle choose action
            if matches_binding(key_msg, &keys.choose) {
                if let Some(item) = m.selected_item() {
                    let status_msg = status_message_style().render(&format!("You chose {}", item.title));
                    return Some(status_message_cmd(status_msg));
                }
            }
            
            // Handle remove action
            if matches_binding(key_msg, &keys.remove) {
                if let Some(item) = m.selected_item() {
                    let title = item.title.clone();
                    let index = m.cursor();
                    m.remove_item(index);
                    
                    // Disable remove key if list is now empty
                    if m.items().is_empty() {
                        drop(keys);
                        self.keys.lock().unwrap().remove.set_enabled(false);
                    }
                    
                    let status_msg = status_message_style().render(&format!("Deleted {}", title));
                    return Some(status_message_cmd(status_msg));
                }
            }
        }
        None
    }

    fn short_help(&self) -> Vec<Binding> {
        let keys = self.keys.lock().unwrap();
        keys.short_help().into_iter().cloned().collect()
    }

    fn full_help(&self) -> Vec<Vec<Binding>> {
        let keys = self.keys.lock().unwrap();
        keys.full_help().into_iter().map(|row| row.into_iter().cloned().collect()).collect()
    }

    fn on_select(&self, _index: usize, _item: &GroceryItem) -> Option<Cmd> {
        // Handled in update() method now
        None
    }

    fn on_remove(&self, _index: usize, _item: &GroceryItem) -> Option<Cmd> {
        // Handled in update() method now
        None
    }

    fn can_remove(&self, _index: usize, _item: &GroceryItem) -> bool {
        true
    }
}

// Main app key bindings
#[derive(Debug, Clone)]
pub struct ListKeyMap {
    pub toggle_spinner: Binding,
    pub toggle_title_bar: Binding,
    pub toggle_status_bar: Binding,
    pub toggle_pagination: Binding,
    pub toggle_help_menu: Binding,
    pub insert_item: Binding,
    pub quit: Binding,
    pub force_quit: Binding,
}

impl Default for ListKeyMap {
    fn default() -> Self {
        Self {
            insert_item: new_binding(vec![
                with_keys_str(&["a"]),
                with_help("a", "add item"),
            ]),
            toggle_spinner: new_binding(vec![
                with_keys_str(&["s"]),
                with_help("s", "toggle spinner"),
            ]),
            toggle_title_bar: new_binding(vec![
                with_keys_str(&["T"]),
                with_help("T", "toggle title"),
            ]),
            toggle_status_bar: new_binding(vec![
                with_keys_str(&["S"]),
                with_help("S", "toggle status"),
            ]),
            toggle_pagination: new_binding(vec![
                with_keys_str(&["P"]),
                with_help("P", "toggle pagination"),
            ]),
            toggle_help_menu: new_binding(vec![
                with_keys_str(&["H", "h", "?"]),
                with_help("H/?", "toggle help"),
            ]),
            quit: new_binding(vec![
                with_keys_str(&["q", "esc"]),
                with_help("q", "quit"),
            ]),
            force_quit: new_binding(vec![
                with_keys_str(&["ctrl+c"]),
                with_help("ctrl+c", "force quit"),
            ]),
        }
    }
}

impl KeyMap for ListKeyMap {
    fn short_help(&self) -> Vec<&Binding> {
        vec![
            &self.toggle_spinner,
            &self.insert_item,
            &self.toggle_title_bar,
            &self.toggle_status_bar,
            &self.toggle_pagination,
            &self.toggle_help_menu,
            &self.quit,
            &self.force_quit,
        ]
    }

    fn full_help(&self) -> Vec<Vec<&Binding>> {
        vec![vec![
            &self.toggle_spinner,
            &self.insert_item,
            &self.toggle_title_bar,
            &self.toggle_status_bar,
            &self.toggle_pagination,
            &self.toggle_help_menu,
            &self.quit,
            &self.force_quit,
        ]]
    }
}

// Main application model
pub struct Model {
    list: List<GroceryItem>,
    help: HelpModel,
    item_generator: Arc<Mutex<RandomItemGenerator>>,
    keys: ListKeyMap,
    delegate: Arc<FancyDelegate>,
    delegate_keys: DelegateKeyMap,
    status_message: String,
}

impl Model {
    fn new() -> Self {
        let mut generator = RandomItemGenerator::new();

        // Create initial items (24 like Go version)
        let mut items = Vec::new();
        for _ in 0..24 {
            items.push(generator.next());
        }

        let delegate = Arc::new(FancyDelegate::new());
        let keys = ListKeyMap::default();
        let delegate_keys = DelegateKeyMap::default();

        // Create list with title and custom styling
        let mut list = List::new(items, delegate.as_ref().clone(), 80, 24)
            .with_title("Groceries")
            .with_pagination_type(PaginatorType::Dots)  // Use dots pagination to match Go version
            .with_show_pagination(true)
            .with_show_status_bar(true)  // Enable status bar so toggle works
            .with_show_help(false); // Disable built-in help - we'll use our custom help

        // Apply title styling
        let mut styles = list.styles().clone();
        styles.title = title_style();
        list = list.with_styles(styles);

        // Create help component - start with short help visible
        let mut help = HelpModel::new().with_width(0); // No width limit
        help.show_all = false; // Start with short help visible

        Self {
            list,
            help,
            item_generator: Arc::new(Mutex::new(generator)),
            keys,
            delegate,
            delegate_keys,
            status_message: String::new(),
        }
    }

}

// Implement KeyMap trait to provide comprehensive help
impl HelpKeyMap for Model {
    fn short_help(&self) -> Vec<&Binding> {
        // Show most essential keys in compact view
        vec![
            &self.keys.insert_item,
            &self.keys.toggle_help_menu,
            &self.keys.quit,
        ]
    }

    fn full_help(&self) -> Vec<Vec<&Binding>> {
        vec![
            // Actions column
            vec![&self.keys.insert_item, &self.keys.toggle_spinner],
            // UI toggles column
            vec![
                &self.keys.toggle_title_bar,
                &self.keys.toggle_status_bar,
                &self.keys.toggle_pagination,
                &self.keys.toggle_help_menu,
            ],
            // Navigation/Item actions column (from delegate)
            vec![&self.delegate_keys.choose, &self.delegate_keys.remove],
            // App control column
            vec![&self.keys.quit, &self.keys.force_quit],
        ]
    }
}

impl BubbleTeaModel for Model {
    fn init() -> (Self, Option<Cmd>) {
        let model = Self::new();
        (model, Some(init_render_cmd()))
    }

    fn update(&mut self, msg: Msg) -> Option<Cmd> {
        // Handle initial render message
        if msg.downcast_ref::<InitRenderMsg>().is_some() {
            return Some(window_size());
        }

        // Handle status messages
        if let Some(status_msg) = msg.downcast_ref::<StatusMessage>() {
            self.status_message = status_msg.0.clone();
            return None;
        }

        // Spinner animation is not fully working in current implementation
        // Removed custom tick handling for now

        // Handle window size changes like Go version
        if let Some(size_msg) = msg.downcast_ref::<WindowSizeMsg>() {
            // Calculate frame size from app_style: padding(1, 2, 1, 2) = 4 horizontal, 2 vertical
            let h = 4; // 2 left + 2 right padding
            let v = 2; // 1 top + 1 bottom padding
            
            // Reserve space for help at the bottom (about 3-4 lines)
            let help_space = 4;
            let available_height = size_msg.height.saturating_sub(v).saturating_sub(help_space);
            
            self.list.set_size(size_msg.width.saturating_sub(h) as usize, available_height as usize);
            
            // Update help width to match available width
            self.help.width = size_msg.width.saturating_sub(h) as usize;
            return None;
        }

        // Handle key messages
        if let Some(key_msg) = msg.downcast_ref::<KeyMsg>() {
            // Handle help toggle FIRST, even during filtering
            if matches_binding(key_msg, &self.keys.toggle_help_menu) {
                self.help.show_all = !self.help.show_all;
                return None;
            }
            
            // Don't match any keys if we're actively filtering
            if self.list.is_filtering() {
                // Delegate to list for processing
                return self.list.update(msg);
            }

            // Handle app-level key bindings
            if matches_binding(key_msg, &self.keys.quit) {
                return Some(bubbletea_rs::quit());
            }
            else if matches_binding(key_msg, &self.keys.toggle_spinner) {
                // Note: Spinner animation may not be fully implemented in current bubbletea-widgets version
                // For now, just toggle visibility - the spinning animation isn't working yet
                let show_spinner = !self.list.show_spinner();
                self.list.set_show_spinner(show_spinner);
                return None;
            }
            else if matches_binding(key_msg, &self.keys.toggle_title_bar) {
                let show_title = !self.list.show_title();
                self.list.set_show_title(show_title);
                return None;
            }
            else if matches_binding(key_msg, &self.keys.toggle_status_bar) {
                let show_status = !self.list.show_status_bar();
                self.list.set_show_status_bar(show_status);
                return None;
            }
            else if matches_binding(key_msg, &self.keys.toggle_pagination) {
                let show_pagination = !self.list.show_pagination();
                self.list.set_show_pagination(show_pagination);
                return None;
            }
            // Help toggle is handled above before filtering check
            // Remove this duplicate handler
            else if matches_binding(key_msg, &self.keys.insert_item) {
                // Enable remove key binding since we're adding an item
                self.delegate.keys.lock().unwrap().remove.set_enabled(true);
                
                // Add new item
                if let Ok(mut generator) = self.item_generator.lock() {
                    let new_item = generator.next();
                    let title = new_item.title.clone();
                    self.list.insert_item(0, new_item);
                    
                    // Set status message
                    let status_msg = status_message_style().render(&format!("Added {}", title));
                    return Some(status_message_cmd(status_msg));
                }
            }
            else if matches_binding(key_msg, &self.keys.force_quit) {
                return Some(bubbletea_rs::quit());
            }
        }

        // Delegate to list widget for other messages
        self.list.update(msg)
    }

    fn view(&self) -> String {
        let mut view = self.list.view();
        
        // Add status message if present
        if !self.status_message.is_empty() {
            view = format!("{}\n\n{}", view, self.status_message);
        }
        
        // Add comprehensive help at the bottom
        let help_view = self.help.view(self);
        if !help_view.is_empty() {
            view = format!("{}\n\n{}", view, help_view);
        }
        
        app_style().render(&view)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let program = Program::<Model>::builder()
        .alt_screen(true)
        .build()?;

    program.run().await?;
    Ok(())
}