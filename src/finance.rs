






// ------------------------------------ Finance ------------------------------------


/// This trait calculates the return on investment (ROI) given a new value. The ROI is calculated as ((new_value - original_value) / original_value) * 100, where original_value is the value of the investment before the change. The function will return the ROI as a percentage. For example, if the original value of an investment is 100 and the new value is 150, the ROI would be ((150 - 100) / 100) * 100 = 50%, indicating a 50% return on the original investment.
/// Example usage:
/// let original_value = 100.0;
/// let new_value = 150.0;
/// let roi = original_value.h_return_on_investment(new_value);
/// The result will be 50.0, because the return on investment is ((150 - 100) / 100) * 100 = 50%.
pub trait ROI {
    fn h_return_on_investment(&self, new_value: f64) -> f64;
}

impl<T> ROI for T
where
    T: Copy + Into<f64>,
{
    fn h_return_on_investment(&self, new_value: f64) -> f64 {
        let start = (*self).into();
        (new_value - start) / start * 100.0
    }
}

/// This trait calculates the discounted price of an item given a percentage decrease. 
/// The discounted price is calculated as original_price * (1 - decrease_percent / 100), 
/// where original_price is the initial price of the item,
///  and decrease_percent is the percentage by which the price is reduced. 
/// The function will return the discounted price. For example, if the original price of an item is 100,
///  and the decrease percent is 20, the discounted price would be 100 * (1 - 20 / 100) = 80, 
/// indicating that the item now costs 80 after a 20% discount.
pub trait DiscountedPrice {
    fn h_decreased_price(&self, decrease_percent: f64) -> f64;
}

impl<T> DiscountedPrice for T
where
    T: Copy + Into<f64>,
{
    fn h_decreased_price(&self, decrease_percent: f64) -> f64 {
        let percent_discount_opposite: f64 = 1.0 - decrease_percent / 100.0;
        percent_discount_opposite * (*self).into()
    }
}


/// This trait calculates the increased price of an item given a percentage increase. 
/// The increased price is calculated as original_price * (1 + increase_percent / 100),
/// where original_price is the initial price of the item, and increase_percent is the percentage 
/// by which the price is increased.
/// The function will return the increased price. For example, if the original price of an item is 100, 
/// and the increase percent is 20, the increased price would be 100 * (1 + 20 / 100) = 120, 
/// indicating that the item now costs 120 after a 20% increase.
pub trait IncreasedPrice {
    fn h_increased_price(&self, increase_percent: f64) -> f64;
}

impl<T> IncreasedPrice for T
where
    T: Copy + Into<f64>,
{
    fn h_increased_price(&self, increase_percent: f64) -> f64 {
        let percent_increas_plus_one: f64 = 1.0 + increase_percent / 100.0;
        percent_increas_plus_one * (*self).into()
    }
}



/// this trait calculates the percentage increase from an original value to a new value.
/// The percentage increase is calculated as ((new_value - original_value) / original_value) * 100, 
/// where original_value is the initial value before the increase. 
/// The function will return the percentage increase as a positive value. For example, 
/// if the original value is 100 and the new value is 150, the percentage increase would be 
/// ((150 - 100) / 100) * 100 = 50%, indicating a 50% increase from the original value.
/// Example usage:
/// let original_value = 100.0;
/// let new_value = 150.0;
/// 
/// let percent_increase = original_value.h_increase_percent_from_original(new_value);
/// The result will be 50.0, because the percentage increase is ((150 -
pub trait IncreasePercentFromOriginal {
    fn h_increase_percent_from_original(&self, new_value: f64) -> f64;
}

impl<T> IncreasePercentFromOriginal for T
where
    T: Copy + Into<f64>,
{
    fn h_increase_percent_from_original(&self, new_value: f64) -> f64 {
        let original_value = (*self).into();
        ((new_value - original_value) / original_value) * 100.0
    }
}


/// This trait calculates the percentage decrease from an original value to a new value.
/// The percentage decrease is calculated as ((original_value - new_value) / original_value) * 100,
///  where original_value is the initial value before the decrease.
/// The function will return the percentage decrease as a positive value.
///  For example, if the original value is 100 and the new value is 80, the percentage decrease would be 
/// ((100 - 80) / 100) * 100 = 20%, indicating a 20% decrease from the original value.
/// Example usage:
/// let original_value = 100.0;
/// let new_value = 80.0;
/// let percent_decrease = original_value.h_decrease_percent_from_original(new_value);
/// The result will be 20.0, because the percentage decrease is ((100 - 80) / 100) * 100 = 20%.
pub trait DecreasePercentFromOriginal {
    fn h_decrease_percent_from_original(&self, new_value: f64) -> f64;
}
    
impl<T> DecreasePercentFromOriginal for T
where
    T: Copy + Into<f64>,
{
    fn h_decrease_percent_from_original(&self, new_value: f64) -> f64 {
        let original_value = (*self).into();
        ((original_value - new_value) / original_value) * 100.0
    }
}



/// This trait calculates the total tax paid based on a set of tax brackets. 
///Each tax bracket is defined by a start income, an end income, and a tax percentage. 
/// The function will iterate through the tax brackets and calculate the tax paid for
///  each bracket based on the income and the corresponding tax rate.
///  The total tax paid will be the sum of the taxes calculated for each bracket. For example, 
/// if the income is 50,000 and there are two tax brackets: (0, 30,000, 10%) and (30,000, 100,000, 20%), 
/// the total tax paid would be (30,000 * 10%) + (20,000 * 20%) = 3,000 + 4,000 = 7,000.
/// 
/// it returns None if the income is less than or equal to 0, 
/// if any tax percentage is greater than 100, 
/// or if the tax brackets are not properly ordered (i.e., i
/// f the start of a bracket is greater than the end of the previous bracket).
pub trait BracketTaxPaid<Start, End, Percent> 
where   
    Start: Copy + Into<f64>,
    End: Copy + Into<f64>,
    Percent: Copy + Into<f64>,
{
    fn h_bracket_tax_paid(&self, brackets: Vec<(Start, End, Percent)>) -> Option<f64>;
}

impl<V, Start, End, Percent> BracketTaxPaid<Start, End, Percent> for V 
where 
    V: Copy + PartialEq + Into<f64>,
    Start: Copy + PartialEq + Into<f64>,
    End: Copy + PartialEq + Into<f64>,
    Percent: Copy + PartialEq + Into<f64>,
{
    fn h_bracket_tax_paid(&self, brackets: Vec<(Start, End, Percent)>) -> Option<f64> {
        if (*self).into() <= 0.0 {
            return None;
        }

        let mut prev_item: Option<&(Start, End, Percent)> = None;
        let income: f64 = (*self).into();
        let mut sum: f64 = 0.0;

        for item in &brackets {
            if item.2.into() > 100.0 {
                return None;
            }
            if let Some(prev) = prev_item {
                if prev.0.into() > item.0.into() || prev.1.into() >= item.1.into() {
                    return None;
                }
            }
            prev_item = Some(item);

            let start: f64 = item.0.into() - 1.0;
            let end: f64 = item.1.into();
            let rate: f64 = item.2.into() / 100.0;

            if income <= start {
                break;
            }

            let taxable = end.min(income) - start;
            sum += taxable * rate;

            if income < end {
                break;
            }
        }
        return Some(sum);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roi() {
        assert_eq!(100.0.h_return_on_investment(150.0), 50.0);
        assert_eq!(200.0.h_return_on_investment(180.0), -10.0);
    }

    #[test]
    fn test_decreased_price() {
        assert_eq!(100.0.h_decreased_price(20.0), 80.0);
        assert_eq!(200.0.h_decreased_price(10.0), 180.0);
    }

    #[test]
    fn test_increased_price() {
        assert!((100.0.h_increased_price(20.0) - 120.0).abs() < 1e-10);
        assert!((200.0.h_increased_price(10.0) - 220.0).abs() < 1e-10);
    }

    #[test]
    fn test_increase_percent_from_original() {
        assert_eq!(100.0.h_increase_percent_from_original(150.0), 50.0);
        assert_eq!(200.0.h_increase_percent_from_original(180.0), -10.0);
    }

    #[test]
    fn test_decrease_percent_from_original() {
        assert_eq!(100.0.h_decrease_percent_from_original(80.0), 20.0);
        assert_eq!(200.0.h_decrease_percent_from_original(220.0), -10.0);
    }

    #[test]
    fn test_bracket_tax_paid() {
        // full brackets
        // 100 income, brackets: 1-50 at 10%, 51-100 at 20%
        // 50 * 0.10 = 5.0, 50 * 0.20 = 10.0, total = 15.0
        let brackets = vec![(1, 50, 10), (51, 100, 20)];
        assert_eq!(100_i32.h_bracket_tax_paid(brackets), Some(15.0));

        // partial bracket
        // 75 income, brackets: 1-50 at 10%, 51-100 at 20%
        // 50 * 0.10 = 5.0, 25 * 0.20 = 5.0, total = 10.0
        let brackets = vec![(1, 50, 10), (51, 100, 20)];
        assert_eq!(75_i32.h_bracket_tax_paid(brackets), Some(10.0));

        // zero income returns None
        let brackets = vec![(1, 50, 10), (51, 100, 20)];
        assert_eq!(0_i32.h_bracket_tax_paid(brackets), None);

        // invalid percent returns None
        let brackets = vec![(1, 50, 101)];
        assert_eq!(100_i32.h_bracket_tax_paid(brackets), None);
}
    
}







