// ------------------------------------ Finance ------------------------------------

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
