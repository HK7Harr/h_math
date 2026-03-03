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
