/// Calculator settings: mode, angle type, radix, word width, precision.

use calc_manager::prelude::*;

/// All configurable calculator settings.
#[derive(Debug, Clone)]
pub struct Settings {
    pub mode: CalculatorMode,
    pub angle_type: AngleType,
    pub radix_type: RadixType,
    pub num_width: NumWidth,
    pub number_format: NumberFormat,
    pub display_precision: Option<i32>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            mode: CalculatorMode::Standard,
            angle_type: AngleType::Degrees,
            radix_type: RadixType::Decimal,
            num_width: NumWidth::QWord,
            number_format: NumberFormat::Float,
            display_precision: None,
        }
    }
}

impl Settings {
    /// Get the internal precision for ratpack calculations based on mode.
    pub fn precision(&self) -> i32 {
        if let Some(p) = self.display_precision {
            return p;
        }
        match self.mode {
            CalculatorMode::Standard => {
                CalculatorPrecision::Standard as i32
            }
            CalculatorMode::Scientific => {
                CalculatorPrecision::Scientific as i32
            }
            CalculatorMode::Programmer => {
                CalculatorPrecision::Programmer as i32
            }
        }
    }

    /// Get the bitmask for the current word width (for programmer mode).
    pub fn word_mask(&self) -> u64 {
        match self.num_width {
            NumWidth::QWord => u64::MAX,
            NumWidth::DWord => u32::MAX as u64,
            NumWidth::Word => u16::MAX as u64,
            NumWidth::Byte => u8::MAX as u64,
        }
    }

    /// Toggle between Float and Scientific notation.
    pub fn toggle_notation(&mut self) {
        self.number_format = match self.number_format {
            NumberFormat::Float => NumberFormat::Scientific,
            NumberFormat::Scientific => NumberFormat::Float,
            NumberFormat::Engineering => NumberFormat::Float,
        };
    }
}
