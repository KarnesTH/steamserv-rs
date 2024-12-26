use std::io::{self, Write};

#[derive(Clone, Debug)]
pub enum ProgressStyle {
    Bar,
    Spinner { states: Vec<char> },
}

impl Default for ProgressStyle {
    fn default() -> Self {
        Self::Bar
    }
}

#[derive(Clone, Debug)]
pub struct Progress {
    pub current: usize,
    pub total: usize,
    pub message: String,
    pub style: ProgressStyle,
}

impl Progress {
    /// Create a new progress bar
    ///
    /// # Arguments
    ///
    /// - `total` - The total number of items to process
    /// - `message` - The message to display with the progress bar
    /// - `style` - The style of the progress bar
    ///
    /// # Returns
    ///
    /// The created progress bar
    ///
    /// # Errors
    ///
    /// If the progress bar could not be created due to an IO error
    pub fn new(total: usize, message: &str, style: ProgressStyle) -> Result<Self, std::io::Error> {
        let progress = Self {
            current: 0,
            total,
            message: message.to_string(),
            style,
        };

        Ok(progress)
    }

    /// Update the progress bar with the current value
    ///
    /// # Arguments
    ///
    /// - `current` - The current value of the progress bar
    ///
    /// # Returns
    ///
    /// Ok if the progress bar was updated successfully
    ///
    /// # Errors
    ///
    /// If the progress bar could not be updated due to an IO error
    pub fn update(&mut self, current: usize) -> Result<(), std::io::Error> {
        self.current = current;
        self.render()?;
        Ok(())
    }

    /// Finish the progress bar
    ///
    /// # Returns
    ///
    /// Ok if the progress bar was finished successfully
    ///
    /// # Errors
    ///
    /// If the progress bar could not be finished due to an IO error
    pub fn finish(&self) -> Result<(), std::io::Error> {
        println!("\n{} - Complete!", self.message);
        Ok(())
    }

    /// Render the progress bar
    ///
    /// # Returns
    ///
    /// Ok if the progress bar was rendered successfully
    ///
    /// # Errors
    ///
    /// If the progress bar could not be rendered due to an IO error
    pub fn render(&self) -> Result<(), std::io::Error> {
        match &self.style {
            ProgressStyle::Bar => self.render_bar()?,
            ProgressStyle::Spinner { states } => self.render_spinner(states)?,
        }

        Ok(())
    }

    /// Render a progress bar
    ///
    /// # Returns
    ///
    /// Ok if the progress bar was rendered successfully
    ///
    /// # Errors
    ///
    /// If the progress bar could not be rendered due to an IO error
    fn render_bar(&self) -> Result<(), std::io::Error> {
        let progress = (self.current as f64 / self.total as f64) * 100.0;
        let width = 50;
        let filled = (width as f64 * (self.current as f64 / self.total as f64)) as usize;

        print!("\r{} [", self.message);
        for i in 0..width {
            if i <= filled {
                print!("=");
            } else {
                print!(" ");
            }
        }
        print!("] {:.1}%", progress);
        io::stdout().flush().unwrap();

        Ok(())
    }

    /// Render a spinner
    ///
    /// # Arguments
    ///
    /// - `states` - The states of the spinner
    ///
    /// # Returns
    ///
    /// Ok if the spinner was rendered successfully
    ///
    /// # Errors
    ///
    /// If the spinner could not be rendered due to an IO error
    fn render_spinner(&self, states: &[char]) -> Result<(), std::io::Error> {
        let state = states[self.current % states.len()];
        print!("\r{} {}", state, self.message);
        io::stdout().flush().unwrap();

        Ok(())
    }

    pub fn tick(&mut self) -> Result<(), std::io::Error> {
        if let ProgressStyle::Spinner { states } = &self.style {
            self.current += 1;
            self.render_spinner(states)?;
        }
        Ok(())
    }
}

/// Get the default spinner style
///
/// # Returns
///
/// The default spinner style
///
/// # Errors
///
/// If the spinner style could not be created due to an IO error
pub fn default_spinner() -> Result<ProgressStyle, std::io::Error> {
    Ok(ProgressStyle::Spinner {
        states: vec!['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏'],
    })
}
