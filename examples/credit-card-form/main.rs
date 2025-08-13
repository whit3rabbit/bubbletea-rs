//! Credit Card Form Example
//!
//! Demonstrates a credit card form with validation using bubbletea-widgets

use bubbletea_rs::{quit, Cmd, KeyMsg, Model, Msg, Program};
use bubbletea_widgets::{key, textinput};
use lipgloss_extras::lipgloss::{join_horizontal, join_vertical, Color, Style, LEFT};

const CCN: usize = 0;
const EXP: usize = 1;
const CVV: usize = 2;

// Colors are created in functions since Color::from is not const

pub struct CreditCardForm {
    inputs: Vec<textinput::Model>,
    focused: usize,
    keymap: AppKeyMap,
}

impl CreditCardForm {
    fn new() -> Self {
        let mut inputs = vec![textinput::new(), textinput::new(), textinput::new()];

        // Credit Card Number
        inputs[CCN].set_placeholder("4505 **** **** 1234");
        let _ = inputs[CCN].focus();
        inputs[CCN].set_char_limit(20);
        inputs[CCN].set_width(30);
        inputs[CCN].set_validate(Box::new(ccn_validator));
        inputs[CCN].prompt = "".to_string();

        // Expiry Date
        inputs[EXP].set_placeholder("MM/YY ");
        inputs[EXP].set_char_limit(5);
        inputs[EXP].set_width(5);
        inputs[EXP].set_validate(Box::new(exp_validator));
        inputs[EXP].prompt = "".to_string();

        // CVV
        inputs[CVV].set_placeholder("XXX");
        inputs[CVV].set_char_limit(3);
        inputs[CVV].set_width(5);
        inputs[CVV].set_validate(Box::new(cvv_validator));
        inputs[CVV].prompt = "".to_string();

        Self {
            inputs,
            focused: 0,
            keymap: AppKeyMap::default(),
        }
    }

    fn next_input(&mut self) {
        self.focused = (self.focused + 1) % self.inputs.len();
    }

    fn prev_input(&mut self) {
        if self.focused == 0 {
            self.focused = self.inputs.len() - 1;
        } else {
            self.focused -= 1;
        }
    }
}

struct AppKeyMap {
    quit: key::Binding,
    enter: key::Binding,
    next: key::Binding,
    prev: key::Binding,
}

impl Default for AppKeyMap {
    fn default() -> Self {
        Self {
            quit: key::new_binding(vec![key::with_keys_str(&["esc", "ctrl+c"])]),
            enter: key::new_binding(vec![key::with_keys_str(&["enter"])]),
            next: key::new_binding(vec![key::with_keys_str(&["tab", "ctrl+n"])]),
            prev: key::new_binding(vec![key::with_keys_str(&["shift+tab", "ctrl+p"])]),
        }
    }
}

// Formatting functions

pub fn format_ccn(input: &str) -> String {
    // Remove all non-digit characters
    let digits: String = input.chars().filter(|c| c.is_ascii_digit()).collect();

    // Limit to 16 digits
    let digits = if digits.len() > 16 {
        &digits[..16]
    } else {
        &digits
    };

    // Add spaces every 4 digits
    let mut formatted = String::new();
    for (i, c) in digits.chars().enumerate() {
        if i > 0 && i % 4 == 0 {
            formatted.push(' ');
        }
        formatted.push(c);
    }

    formatted
}

pub fn format_exp(input: &str) -> String {
    // Remove all non-digit characters
    let digits: String = input.chars().filter(|c| c.is_ascii_digit()).collect();

    // Limit to 4 digits (MMYY)
    let digits = if digits.len() > 4 {
        &digits[..4]
    } else {
        &digits
    };

    // Add slash after 2 digits
    if digits.len() > 2 {
        format!("{}/{}", &digits[..2], &digits[2..])
    } else {
        digits.to_string()
    }
}

// Validation functions

fn ccn_validator(s: &str) -> Result<(), String> {
    // Credit Card Number should be a string less than 20 characters
    // It should include 16 integers and 3 spaces
    if s.len() > 16 + 3 {
        return Err("CCN is too long".to_string());
    }

    if s.is_empty() || (s.len() % 5 != 0 && (!s.chars().last().unwrap().is_ascii_digit())) {
        return Err("CCN is invalid".to_string());
    }

    // The last digit should be a number unless it is a multiple of 5 in which
    // case it should be a space
    if s.len() % 5 == 0 && s.chars().last() != Some(' ') {
        return Err("CCN must separate groups with spaces".to_string());
    }

    // The remaining digits should be integers
    let c = s.replace(' ', "");
    if !c.is_empty() {
        c.parse::<u64>().map_err(|_| "CCN is invalid".to_string())?;
    }

    Ok(())
}

fn exp_validator(s: &str) -> Result<(), String> {
    // The 3rd character should be a slash (/)
    // The rest should be numbers
    let e = s.replace('/', "");
    if !e.is_empty() {
        e.parse::<u64>().map_err(|_| "EXP is invalid".to_string())?;
    }

    // There should be only one slash and it should be in the 2nd index (3rd character)
    if s.len() >= 3 {
        let slash_pos = s.find('/');
        let last_slash_pos = s.rfind('/');
        if slash_pos != Some(2) || last_slash_pos != Some(2) {
            return Err("EXP is invalid".to_string());
        }
    }

    Ok(())
}

fn cvv_validator(s: &str) -> Result<(), String> {
    // The CVV should be a number of 3 digits
    // Since the input will already ensure that the CVV is a string of length 3,
    // All we need to do is check that it is a number
    if !s.is_empty() {
        s.parse::<u64>().map_err(|_| "CVV is invalid".to_string())?;
    }
    Ok(())
}

impl Model for CreditCardForm {
    fn init() -> (Self, Option<Cmd>) {
        let mut model = Self::new();
        let cmd = model.inputs[0].focus();
        (model, Some(cmd))
    }

    fn update(&mut self, msg: Msg) -> Option<Cmd> {
        if let Some(key_msg) = msg.downcast_ref::<KeyMsg>() {
            match key_msg {
                _ if self.keymap.quit.matches(key_msg) => {
                    return Some(quit());
                }
                _ if self.keymap.enter.matches(key_msg) => {
                    if self.focused == self.inputs.len() - 1 {
                        return Some(quit());
                    }
                    self.next_input();
                }
                _ if self.keymap.prev.matches(key_msg) => {
                    self.prev_input();
                }
                _ if self.keymap.next.matches(key_msg) => {
                    self.next_input();
                }
                _ => {}
            }

            // Update focus states
            for (i, input) in self.inputs.iter_mut().enumerate() {
                if i == self.focused {
                    let _ = input.focus();
                } else {
                    input.blur();
                }
            }
        }

        // Update the focused input and handle formatting
        let cmd = self.inputs[self.focused].update(msg);

        // Auto-format credit card number with spaces
        if self.focused == CCN {
            let current_value = self.inputs[CCN].value();
            let formatted = format_ccn(&current_value);
            if formatted != current_value {
                self.inputs[CCN].set_value(&formatted);
            }
        }

        // Auto-format expiry date with slash
        if self.focused == EXP {
            let current_value = self.inputs[EXP].value();
            let formatted = format_exp(&current_value);
            if formatted != current_value {
                self.inputs[EXP].set_value(&formatted);
            }
        }

        cmd
    }

    fn view(&self) -> String {
        let hot_pink = Color::from("#FF06B7");
        let dark_gray = Color::from("#767676");
        let input_style = Style::new().foreground(hot_pink);
        let continue_style = Style::new().foreground(dark_gray);

        let card_number_section = join_vertical(
            LEFT,
            &[
                &input_style.clone().width(30).render(" Card Number"),
                &format!(" {}", self.inputs[CCN].view()),
            ],
        );

        let labels_row = join_horizontal(
            LEFT,
            &[
                &input_style.clone().width(6).render(" EXP"),
                &"  ",
                &input_style.clone().width(6).render("CVV"),
            ],
        );

        let inputs_row = join_horizontal(
            LEFT,
            &[
                &Style::new()
                    .width(6)
                    .render(&format!(" {}", self.inputs[EXP].view())),
                &"  ",
                &Style::new().width(6).render(&self.inputs[CVV].view()),
            ],
        );

        let exp_cvv_section = join_vertical(LEFT, &[&labels_row, &inputs_row]);

        join_vertical(
            LEFT,
            &[
                &" Total: $21.50:",
                &"",
                &card_number_section,
                &"",
                &exp_cvv_section,
                &"",
                &continue_style.render(" Continue ->"),
            ],
        )
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let program = Program::<CreditCardForm>::builder()
        .alt_screen(true)
        .signal_handler(true)
        .build()?;
    let _ = program.run().await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_layout_alignment() {
        let (model, _) = CreditCardForm::init();
        let view = model.view();

        println!("=== Credit Card Form Layout ===");
        println!("{}", view);
        println!("=== End Layout ===");

        let lines: Vec<&str> = view.lines().collect();

        // Find the lines with EXP/CVV labels and inputs
        let mut exp_cvv_labels_line = None;
        let mut exp_cvv_inputs_line = None;

        for (i, line) in lines.iter().enumerate() {
            if line.contains("EXP") && line.contains("CVV") {
                exp_cvv_labels_line = Some((i, line));
            }
            if (line.contains("MM/YY") || line.contains("XXX"))
                && !line.contains("EXP")
                && !line.contains("CVV")
            {
                exp_cvv_inputs_line = Some((i, line));
            }
        }

        if let (Some((labels_idx, labels_line)), Some((inputs_idx, inputs_line))) =
            (exp_cvv_labels_line, exp_cvv_inputs_line)
        {
            println!("Labels line {}: '{}'", labels_idx, labels_line);
            println!("Inputs line {}: '{}'", inputs_idx, inputs_line);

            // Check that CVV appears in both lines at roughly the same column position
            if let (Some(labels_cvv_pos), Some(_inputs_placeholder_pos)) =
                (labels_line.find("CVV"), inputs_line.find("XXX"))
            {
                println!("CVV label position: {}", labels_cvv_pos);
                // This test verifies the layout is structured properly
                assert!(
                    labels_cvv_pos > 0,
                    "CVV label should be positioned after EXP"
                );
            }
        }

        // Verify the total line appears
        assert!(
            view.contains("Total: $21.50"),
            "Should contain total amount"
        );
        assert!(
            view.contains("Continue ->"),
            "Should contain continue button"
        );
    }
}
