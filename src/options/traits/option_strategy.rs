//! # Option Strategy
//!
//! The `OptionStrategy` trait provides methods for calculating the payoff of various option strategies.
//!
//! ## References
//!
//! - [Option Strategies](https://www.investopedia.com/terms/o/option-strategy.asp)
//! - [Options Strategies](https://www.optionsplaybook.com/option-strategies/)

use plotters::{
    coord::{types::RangedCoordf64, Shift},
    prelude::*,
};

use super::OptionPricing;
use crate::{
    check_is_call, check_is_put, check_same_expiration_date, log_info, log_warn,
    options::{EuropeanOption, Instrument, Option},
};

/// Trait for non-directional strategies.
pub trait OptionStrategy: OptionPricing {
    /* PLOT-FUNCTIONS */

    /// Plot the payoff of an option strategy.
    ///
    /// # Arguments
    /// * `strategy_name` - The name of the strategy.
    /// * `strategy_fn` - A closure that takes the stock price and returns (payoff, price).
    /// * `range` - Stock price range.
    /// * `file_name` - Output file path.
    ///
    /// # Returns
    /// Result containing the plot or an error.
    fn plot_strategy<F>(
        &self,
        strategy_name: &str,
        strategy_fn: F,
        range: std::ops::Range<f64>,
        file_name: &str,
    ) -> Result<(), Box<dyn std::error::Error>>
    where
        F: Fn(f64) -> (f64, f64),
    {
        self.plot_strategy_breakdown::<F, EuropeanOption>(
            strategy_name,
            strategy_fn,
            range,
            file_name,
            [].as_ref(),
        )
    }
    /// Plot the payoff of an option strategy.
    ///
    /// # Arguments
    /// * `strategy_name` - The name of the strategy.
    /// * `strategy_fn` - A closure that takes the stock price and returns (payoff, price).
    /// * `range` - Stock price range.
    /// * `file_name` - Output file path.
    /// * `options` - A list of options to plot individually in smaller graphs.
    ///
    /// # Returns
    /// Result containing the plot or an error.
    fn plot_strategy_breakdown<F, T>(
        &self,
        strategy_name: &str,
        strategy_fn: F,
        range: std::ops::Range<f64>,
        file_name: &str,
        options: &[T],
    ) -> Result<(), Box<dyn std::error::Error>>
    where
        F: Fn(f64) -> (f64, f64),
        T: Option,
    {
        let spots: Vec<f64> = (range.start as u32..=range.end as u32)
            .map(|x| x as f64)
            .collect();

        let (payoffs, prices, p_l) = Self::calculate_payoffs_prices_and_pl(&spots, &strategy_fn);
        let (min_y, max_y) = Self::calculate_y_bounds(&payoffs, &prices, &p_l);

        // Adjust canvas size if options are present
        let (upper, lower) = {
            let num_options = options.len();
            let num_columns = match num_options {
                n if n % 4 == 0 => 4,
                n if n % 3 == 0 => 3,
                _ => 2,
            };
            let num_rows = (num_options as f64 / num_columns as f64).ceil() as usize;

            let root = BitMapBackend::new(
                file_name,
                (2800, (1600.0 + (num_rows as f64 * 800.0)) as u32),
            )
            .into_drawing_area();
            root.split_vertically(1600)
        } as (DrawingArea<_, Shift>, DrawingArea<_, Shift>);

        // Plot the main strategy chart
        Self::plot_strategy_main_chart(
            &upper,
            &spots,
            &payoffs,
            &prices,
            &p_l,
            &range,
            min_y,
            max_y,
            strategy_name,
        )?;

        // Plot individual option graphs if provided
        if !options.is_empty() {
            let num_options = options.len();
            let num_columns = match num_options {
                n if n % 4 == 0 => 4,
                n if n % 3 == 0 => 3,
                _ => 2,
            };
            let num_rows = (num_options as f64 / num_columns as f64).ceil() as usize;

            let grid = lower.split_evenly((num_rows, num_columns));

            for (i, option) in options.iter().enumerate() {
                let option_plot_area = &grid[i];

                let option_payoffs: Vec<f64> = spots
                    .iter()
                    .map(|&spot| option.payoff(Some(spot)))
                    .collect();
                let option_prices: Vec<f64> =
                    spots.iter().map(|&spot| self.price(option)).collect();
                let (min_y, max_y) =
                    Self::calculate_y_bounds(&option_payoffs, &option_prices, &option_payoffs);

                let mut chart = ChartBuilder::on(option_plot_area)
                    .margin(40)
                    .x_label_area_size(100)
                    .y_label_area_size(100)
                    .build_cartesian_2d(range.clone(), min_y..max_y)?;

                chart.plotting_area().fill(&BLACK)?;

                chart
                    .configure_mesh()
                    .x_desc(format!(
                        "{:?} @ ${:.2} | {} | TTM: {:.2}y",
                        option.option_type(),
                        option.strike(),
                        if option.itm() {
                            "ITM"
                        } else if option.atm() {
                            "ATM"
                        } else {
                            "OTM"
                        },
                        option.time_to_maturity()
                    ))
                    .y_desc("Value ($)")
                    .x_label_style(("Inter", 30).into_font().color(&WHITE))
                    .y_label_style(("Inter", 30).into_font().color(&WHITE))
                    .axis_desc_style(("Inter", 42, FontStyle::Bold).into_font().color(&WHITE)) // Changed from x_desc_style and y_desc_style
                    .axis_style(WHITE.mix(0.8))
                    .light_line_style(WHITE.mix(0.2).stroke_width(1))
                    .bold_line_style(WHITE.mix(0.7).stroke_width(1))
                    .draw()?;

                // Define curves to plot
                let curves = [
                    (&option_payoffs, RGBColor(0, 255, 255), "Payoff Curve"),
                    (&option_prices, RGBColor(255, 140, 0), "Price Curve"),
                ];

                for (values, color, label) in curves {
                    chart
                        .draw_series(LineSeries::new(
                            spots.iter().zip(values.iter()).map(|(&x, &y)| (x, y)),
                            ShapeStyle {
                                color: color.to_rgba(),
                                filled: true,
                                stroke_width: 5,
                            },
                        ))?
                        .label(label)
                        .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 40, y)], color));
                }

                chart
                    .configure_series_labels()
                    .background_style(BLACK.mix(0.8).filled())
                    .border_style(WHITE.mix(0.8))
                    .label_font(("Inter", 40).into_font().color(&WHITE))
                    .draw()?;
            }
        }

        Ok(())
    }

    /// Calculate the payoff, price, and P/L for the strategy.
    fn calculate_payoffs_prices_and_pl<F>(
        spots: &[f64],
        strategy_fn: F,
    ) -> (Vec<f64>, Vec<f64>, Vec<f64>)
    where
        F: Fn(f64) -> (f64, f64),
    {
        let (payoffs, prices): (Vec<_>, Vec<_>) = spots.iter().map(|&s| strategy_fn(s)).unzip();

        let p_l: Vec<f64> = payoffs
            .iter()
            .zip(&prices)
            .map(|(payoff, price)| payoff - price)
            .collect();

        (payoffs, prices, p_l)
    }

    /// Calculate the y-axis bounds for the chart.
    fn calculate_y_bounds(payoffs: &[f64], prices: &[f64], p_l: &[f64]) -> (f64, f64) {
        let min_y = payoffs
            .iter()
            .chain(prices.iter())
            .chain(p_l.iter())
            .cloned()
            .fold(f64::INFINITY, f64::min)
            .min(0.0);

        let max_y = payoffs
            .iter()
            .chain(prices.iter())
            .chain(p_l.iter())
            .cloned()
            .fold(f64::NEG_INFINITY, f64::max)
            .max(0.0);

        (min_y - 1.0, max_y + 1.0)
    }

    #[allow(clippy::too_many_arguments)]
    fn plot_strategy_main_chart(
        upper: &DrawingArea<BitMapBackend, Shift>,
        spots: &[f64],
        payoffs: &[f64],
        prices: &[f64],
        p_l: &[f64],
        range: &std::ops::Range<f64>,
        min_y: f64,
        max_y: f64,
        strategy_name: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut chart = ChartBuilder::on(upper)
            .caption(
                format!("{strategy_name} Strategy - Payoff & P/L"),
                ("Inter", 60, FontStyle::Bold).into_font().color(&WHITE),
            )
            .margin(60)
            .x_label_area_size(110)
            .y_label_area_size(110)
            .build_cartesian_2d(range.clone(), min_y..max_y)?;

        // Configure chart appearance
        chart
            .configure_mesh()
            .x_desc("Underlying Price ($)")
            .y_desc("Value ($)")
            .x_label_style(("Inter", 48, FontStyle::Bold).into_font().color(&WHITE))
            .y_label_style(("Inter", 48, FontStyle::Bold).into_font().color(&WHITE))
            .axis_style(WHITE.mix(0.6))
            .light_line_style(WHITE.mix(0.4).stroke_width(1))
            .bold_line_style(WHITE.mix(0.5).stroke_width(2))
            .draw()?;

        // Plot curves
        let curves = [
            (payoffs, RGBColor(0, 255, 255), "Payoff Curve"),
            (prices, RGBColor(255, 140, 0), "Price Curve"),
            (p_l, RGBColor(255, 0, 255), "P/L Curve"),
        ];

        for (values, color, label) in curves {
            Self::plot_curve(&mut chart, spots, values, &RGBAColor::from(color), label)?;
        }

        // Draw horizontal zero line
        chart.draw_series(DashedLineSeries::new(
            (range.start as u32..=range.end as u32).map(|x| (x as f64, 0.0)),
            20,
            10,
            WHITE.stroke_width(2),
        ))?;

        // Configure legend
        chart
            .configure_series_labels()
            .border_style(WHITE)
            .label_font(("Inter", 48).into_font().color(&WHITE))
            .background_style(BLACK.mix(0.8).filled())
            .draw()?;

        Ok(())
    }

    /// Plot a single curve (e.g., Payoff, Price, or P/L) on the chart.
    fn plot_curve(
        chart: &mut ChartContext<BitMapBackend, Cartesian2d<RangedCoordf64, RangedCoordf64>>,
        spots: &[f64],
        values: &[f64],
        color: &RGBAColor,
        label: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let owned_color = RGBColor(color.0, color.1, color.2);
        chart
            .draw_series(
                LineSeries::new(
                    spots.iter().cloned().zip(values.iter().cloned()),
                    ShapeStyle {
                        color: color.to_rgba(),
                        filled: false,
                        stroke_width: 5,
                    },
                )
                .point_size(12),
            )?
            .label(label)
            .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 40, y)], owned_color));

        Ok(())
    }

    /* AUTO-STRATEGY */

    /// Auto-strategy that automatically selects the best strategy based on owned stock and option.
    fn auto_strategy_stock<'a, T: Option>(
        &'a self,
        stock: &'a Instrument,
        option: &'a T,
    ) -> impl Fn(f64) -> (f64, f64) + 'a {
        move |spot_price| {
            if option.is_call() {
                log_info!("Auto-strategy: Covered Call. Alternative strategy: Long Call");
                self.covered_call(stock, option)(spot_price)
            } else {
                log_info!("Auto-strategy: Protective Put. Alternative strategy: Long Put");
                self.protective_put(stock, option)(spot_price)
            }
        }
    }

    // Auto-strategy that automatically selects the best strategy based on two options.
    //fn auto_strategy<T: Option>(&self, option1: &T, option2: &T) -> f64 {
    //    if option1.time_to_maturity() < option2.time_to_maturity() {
    //        log_info!("Auto-strategy: Calendar Spread.");
    //        return self.back_spread(option2, option1);
    //    }
    //
    //    match (
    //        option1.is_call(),
    //        option2.is_call(),
    //        option1.itm(),
    //        option2.itm(),
    //        option1.atm(),
    //        option2.atm(),
    //        option1.otm(),
    //        option2.otm(),
    //        option1.time_to_maturity() < option2.time_to_maturity(),
    //    ) {
    //        (true, true, _, false, _, false, true, false, false) => {
    //            log_info!("Auto-strategy: Long Call Spread.");
    //            todo!()
    //        }
    //        (false, false, _, false, _, false, true, false, false) => {
    //            log_info!("Auto-strategy: Long Put Spread.");
    //            todo!()
    //        }
    //        (true, true, false, false, false, false, true, true, false) => {
    //            log_info!("Auto-strategy: Short Call Spread.");
    //            todo!()
    //        }
    //        (false, false, false, false, false, false, true, true, false) => {
    //            log_info!("Auto-strategy: Short Put Spread.");
    //            todo!()
    //        }
    //        (false, true, false, false, true, true, false, false, false) => {
    //            log_info!("Auto-strategy: Long Straddle.");
    //            self.straddle(option1, option2)
    //        }
    //        (true, false, false, false, true, true, false, false, false) => {
    //            log_info!("Auto-strategy: Long Straddle.");
    //            self.straddle(option2, option1)
    //        }
    //        (false, true, true, true, false, false, false, false, _) => {
    //            log_info!("Auto-strategy: Long Strangle.");
    //            self.strangle(option2, option1, 0.0).0
    //        }
    //        (true, false, true, true, false, false, false, false, _) => {
    //            log_info!("Auto-strategy: Long Strangle.");
    //            self.strangle(option2, option1, 0.0).0
    //        }
    //        (false, true, false, false, false, false, true, true, _) => {
    //            log_info!("Auto-strategy: Long Guts.");
    //            self.strangle(option2, option1, 0.0).0
    //        }
    //        (true, false, false, false, false, false, true, true, _) => {
    //            log_info!("Auto-strategy: Long Guts.");
    //            self.strangle(option2, option1, 0.0).0
    //        }
    //
    //        _ => panic!("Auto-strategy not implemented for this combination"),
    //    }
    //}

    /* STOCK & OPTION */

    /// Own underlying stock and sell a OTM (out of the money) call.
    fn covered_call<'a, T: Option>(
        &'a self,
        stock: &'a Instrument,
        call: &'a T,
    ) -> impl Fn(f64) -> (f64, f64) + 'a {
        move |spot_price| {
            check_is_call!(call);
            assert!(
                stock.spot() > 0.0 && call.otm(),
                "Stock price must be positive and call must be OTM!"
            );

            let price = stock.spot() - self.price(call);
            let payoff = spot_price - call.payoff(Some(spot_price));
            (payoff, price)
        }
    }

    /// Buy (long protective put) or sell (short protective put) a pair of ITM (in the money) stock and OTM (out of the money) put.
    fn protective_put<'a, T: Option>(
        &'a self,
        stock: &'a Instrument,
        put: &'a T,
    ) -> impl Fn(f64) -> (f64, f64) + 'a {
        move |spot_price| {
            check_is_put!(put);
            assert!(
                stock.spot() > 0.0 && put.otm(),
                "Stock price must be positive and put must be OTM!"
            );

            let price = stock.spot() + self.price(put);
            let payoff = spot_price + put.payoff(Some(spot_price));
            (payoff, price)
        }
    }

    /// The collar strategy involves owning the underlying, buying a protective put and selling a covered call with the same expiration date.
    fn collar<'a, T: Option>(
        &'a self,
        stock: &'a Instrument,
        otm_put: &'a T,
        otm_call: &'a T,
    ) -> impl Fn(f64) -> (f64, f64) + 'a {
        move |spot_price| {
            check_same_expiration_date!(otm_put, otm_call);
            check_is_put!(otm_put);
            check_is_call!(otm_call);

            assert!(
                stock.spot() > 0.0 && otm_put.otm() && otm_call.otm(),
                "Stock price must be positive and options must be OTM!"
            );

            let price = stock.spot() + self.price(otm_put) - self.price(otm_call);
            let payoff =
                spot_price + otm_put.payoff(Some(spot_price)) - otm_call.payoff(Some(spot_price));
            (payoff, price)
        }
    }

    /// The fence strategy consists of a long position in a financial instrument, a long ATM put and short positions in a OTM call and a OTM put.
    fn fence<'a, T: Option>(
        &'a self,
        stock: &'a Instrument,
        atm_put: &'a T,
        otm_put: &'a T,
        otm_call: &'a T,
    ) -> impl Fn(f64) -> (f64, f64) + 'a {
        move |spot_price| {
            check_same_expiration_date!(atm_put, otm_put);
            check_same_expiration_date!(otm_put, otm_call);
            check_is_put!(atm_put);
            check_is_put!(otm_put);
            check_is_call!(otm_call);

            assert!(
                stock.spot() > 0.0 && otm_put.otm() && otm_call.otm() && atm_put.atm(),
                "Stock price must be positive and options must be OTM and ATM!"
            );

            let price = stock.spot() + self.price(otm_put) + self.price(atm_put)
                - self.price(otm_put)
                - self.price(otm_call);
            let payoff =
                spot_price + otm_put.payoff(Some(spot_price)) + atm_put.payoff(Some(spot_price))
                    - otm_put.payoff(Some(spot_price))
                    - otm_call.payoff(Some(spot_price));
            (payoff, price)
        }
    }

    /* SIMPLE */

    /// Buy (long gut) or sell (short gut) a pair of ITM (in the money) put and call.
    /// In long guts, you profit if the stock or index moves significantly in either direction.
    /// In short guts, you profit if the stock or index remains within the two short strikes.
    fn guts<'a, T: Option>(&'a self, put: &'a T, call: &'a T) -> impl Fn(f64) -> (f64, f64) + 'a {
        move |spot_price| {
            check_same_expiration_date!(put, call);
            check_is_call!(call);
            check_is_put!(put);

            assert!(put.itm() && call.itm(), "Put and call must be ITM!");

            let price = self.price(put) + self.price(call);
            let payoff = put.payoff(Some(spot_price)) + call.payoff(Some(spot_price));
            (payoff, price)
        }
    }

    /// Buy (long straddle) or sell (short straddle) a pair of ATM (at the money) put and call.
    /// Can be used for earnings when you are anticipating that the underlying stock will move
    /// in a direction by an extent that exceeds the total to purchase both options.
    fn straddle<'a, T: Option>(
        &'a self,
        put: &'a T,
        call: &'a T,
    ) -> impl Fn(f64) -> (f64, f64) + 'a {
        move |spot_price| {
            check_same_expiration_date!(put, call);
            check_is_call!(call);
            check_is_put!(put);

            assert!(put.atm() && call.atm(), "Put and call must be ATM!");

            let price = self.price(put) + self.price(call);
            let payoff = put.payoff(Some(spot_price)) + call.payoff(Some(spot_price));
            (payoff, price)
        }
    }

    /// Buy (long strangle) or sell (short strangle) a pair of OTM (out of the money) put and call.
    /// In long strangle, you profit if the stock or index moves significantly in either direction.
    /// In short strangle, you profit if the stock or index remains within the two short strikes.
    fn strangle<'a, T: Option>(
        &'a self,
        put: &'a T,
        call: &'a T,
    ) -> impl Fn(f64) -> (f64, f64) + 'a {
        move |spot_price| {
            check_same_expiration_date!(put, call);
            check_is_call!(call);
            check_is_put!(put);

            assert!(put.otm() && call.otm(), "Put and call must be OTM!");

            let price = self.price(put) + self.price(call);
            let payoff = put.payoff(Some(spot_price)) + call.payoff(Some(spot_price));
            (payoff, price)
        }
    }

    /// A risk-reversal is an option position that consists of shorting an OTM put and being long in an OTM call expiring on the same expiration date.
    fn risk_reversal<'a, T: Option>(
        &'a self,
        put: &'a T,
        call: &'a T,
    ) -> impl Fn(f64) -> (f64, f64) + 'a {
        move |spot_price| {
            check_same_expiration_date!(put, call);
            check_is_call!(call);
            check_is_put!(put);

            assert!(put.otm() && call.otm(), "Put and call must be OTM!");

            let price = self.price(call) - self.price(put);
            let payoff = call.payoff(Some(spot_price)) - put.payoff(Some(spot_price));
            (payoff, price)
        }
    }

    /* BUTTERFLY */

    /// Long butterfly spreads use four option contracts with the same expiration but three different strike prices to create a range of prices the strategy can profit from.
    /// Note that the lower and upper wings will be long calls or puts, and the body will be short calls or puts.
    fn butterfly<'a, T: Option>(
        &'a self,
        lower: &'a T,
        body: &'a T,
        upper: &'a T,
    ) -> impl Fn(f64) -> (f64, f64) + 'a {
        move |spot_price| {
            check_same_expiration_date!(lower, body);
            check_same_expiration_date!(body, upper);
            check_same_expiration_date!(lower, upper);

            if lower.is_call() {
                check_is_call!(upper);
                check_is_call!(body);
                assert!(
                    lower.strike() < body.strike() && body.strike() < upper.strike(),
                    "Butterfly spread using calls requires ordered strikes: lower < middle < upper!"
                );
            } else {
                check_is_put!(upper);
                check_is_put!(body);
                assert!(
                    lower.strike() > body.strike() && body.strike() > upper.strike(),
                    "Butterfly spread using puts requires ordered strikes: lower > middle > upper!"
                );
            }

            // Check if the strikes are equidistant.
            let lower_to_body = (body.strike() - lower.strike()).abs();
            let body_to_upper = (upper.strike() - body.strike()).abs();
            if lower_to_body != body_to_upper {
                log_warn!("Strikes are not equidistant => constructing a broken wing / skip strike butterfly!");
            }

            let price = self.price(lower) - 2.0 * self.price(body) + self.price(upper);
            let payoff = lower.payoff(Some(spot_price)) - 2.0 * body.payoff(Some(spot_price))
                + upper.payoff(Some(spot_price));
            (payoff, price)
        }
    }

    /// The iron butterfly strategy involves buying a pair of OTM (out of the money) call and put, and shorting a pair of ATM (at the money) call and put.
    /// It is a limited-risk, limited-profit trading strategy structured for a larger probability of earning smaller limited profit when the underlying stock is perceived to have a low volatility.
    fn iron_butterfly<'a, T: Option>(
        &'a self,
        otm_put: &'a T,
        atm_put: &'a T,
        atm_call: &'a T,
        otm_call: &'a T,
    ) -> impl Fn(f64) -> (f64, f64) + 'a {
        move |spot_price| {
            check_same_expiration_date!(otm_put, atm_put);
            check_same_expiration_date!(atm_put, atm_call);
            check_same_expiration_date!(atm_call, otm_call);
            check_is_put!(otm_put);
            check_is_put!(atm_put);
            check_is_call!(atm_call);
            check_is_call!(otm_call);

            assert!(otm_put.strike() < atm_put.strike() && atm_put.strike() == atm_call.strike() && atm_call.strike() < otm_call.strike(),
                    "Iron Butterfly must have ordered strikes: lower_put < atm_put == atm_call < upper_call");

            let price = self.price(otm_put) - self.price(atm_put) - self.price(atm_call)
                + self.price(otm_call);
            let payoff = otm_put.payoff(Some(spot_price))
                - atm_put.payoff(Some(spot_price))
                - atm_call.payoff(Some(spot_price))
                + otm_call.payoff(Some(spot_price));
            (payoff, price)
        }
    }

    /// The christmas tree butterfly spread is a limited risk, limited reward strategy that profits from a stock trading in a narrow range.
    /// It is constructed by holding a long butterfly spread with either only calls or only puts, while shorting the same butterfly spread.
    fn christmas_tree_butterfly<'a, T: Option>(
        &'a self,
        lower: &'a T,
        middle1: &'a T,
        middle2: &'a T,
        middle3: &'a T,
        upper1: &'a T,
        upper2: &'a T,
    ) -> impl Fn(f64) -> (f64, f64) + 'a {
        move |spot_price| {
            check_same_expiration_date!(lower, middle1);
            check_same_expiration_date!(middle1, middle2);
            check_same_expiration_date!(middle2, middle3);
            check_same_expiration_date!(middle3, upper1);
            check_same_expiration_date!(upper1, upper2);

            if lower.is_call() {
                check_is_call!(middle1);
                check_is_call!(middle2);
                check_is_call!(middle3);
                check_is_call!(upper1);
                check_is_call!(upper2);
                assert!(
                lower.strike() < middle1.strike()
                    && middle1.strike() == middle2.strike()
                    && middle2.strike() == middle3.strike()
                    && middle3.strike() < upper1.strike()
                    && upper1.strike() == upper2.strike(),
                "Christmas Tree Butterfly using calls must have ordered strikes: lower < (middle1 == middle2 == middle3) < (upper1 == upper2)"
            );
            } else {
                check_is_put!(middle1);
                check_is_put!(middle2);
                check_is_put!(middle3);
                check_is_put!(upper1);
                check_is_put!(upper2);
                assert!(
                lower.strike() > middle1.strike()
                    && middle1.strike() == middle2.strike()
                    && middle2.strike() == middle3.strike()
                    && middle3.strike() > upper1.strike()
                    && upper1.strike() == upper2.strike(),
                "Christmas Tree Butterfly using puts must have ordered strikes: lower > (middle1 == middle2 == middle3) > (upper1 == upper2)"
            );
            }

            let price = self.price(lower)
                - (self.price(middle1) + self.price(middle2) + self.price(middle3))
                + (self.price(upper1) + self.price(upper2));

            let payoff = lower.payoff(Some(spot_price))
                - middle1.payoff(Some(spot_price))
                - middle2.payoff(Some(spot_price))
                - middle3.payoff(Some(spot_price))
                + upper1.payoff(Some(spot_price))
                + upper2.payoff(Some(spot_price));

            (payoff, price)
        }
    }

    /* CONDOR */

    /// The condor strategy involves buying one OTM and one ITM call/put (long condor spread) shorting a less OTM and less ITM call/put with different strike prices.
    fn condor<'a, T: Option>(
        &'a self,
        itm_long: &'a T,
        itm_short: &'a T,
        otm_short: &'a T,
        otm_long: &'a T,
    ) -> impl Fn(f64) -> (f64, f64) + 'a {
        move |spot_price| {
            check_same_expiration_date!(itm_long, itm_short);
            check_same_expiration_date!(itm_short, otm_short);
            check_same_expiration_date!(otm_short, otm_long);

            assert!(itm_long.itm() && itm_short.itm(), "Options must be ITM!");
            assert!(otm_short.otm() && otm_long.otm(), "Options must be OTM!");

            if itm_long.is_call() {
                check_is_call!(itm_short);
                check_is_call!(otm_short);
                check_is_call!(otm_long);

                assert!(itm_long.strike() <= itm_short.strike() && itm_short.strike() <= otm_short.strike() && otm_short.strike() <= otm_long.strike(),
            "Condor Spread w/ Call must have ordered strikes: ITM (long) <= ITM (short) <= OTM (short) <= OTM (long)");
            } else {
                check_is_put!(itm_short);
                check_is_put!(otm_short);
                check_is_put!(otm_long);

                assert!(itm_long.strike() >= itm_short.strike() && itm_short.strike() >= otm_short.strike() && otm_short.strike() >= otm_long.strike(),
            "Condor Spread w/ Puts must have ordered strikes: OTM (long) <= OTM (short) <= ITM (short) <= ITM (long)");
            }

            let price = self.price(itm_long) - self.price(itm_short) - self.price(otm_short)
                + self.price(otm_long);
            let payoff = itm_long.payoff(Some(spot_price))
                - itm_short.payoff(Some(spot_price))
                - otm_short.payoff(Some(spot_price))
                + otm_long.payoff(Some(spot_price));
            (payoff, price)
        }
    }

    /// The iron condor strategy involves buying a call and put with different strike prices and selling a call and put with different strike prices.
    fn iron_condor<'a, T: Option>(
        &'a self,
        otm_put_long: &'a T,
        otm_put_short: &'a T,
        otm_call_short: &'a T,
        otm_call_long: &'a T,
    ) -> impl Fn(f64) -> (f64, f64) + 'a {
        move |spot_price| {
            check_same_expiration_date!(otm_put_long, otm_put_short);
            check_same_expiration_date!(otm_put_short, otm_call_short);
            check_same_expiration_date!(otm_call_short, otm_call_long);

            check_is_put!(otm_put_long);
            check_is_put!(otm_put_short);
            check_is_call!(otm_call_short);
            check_is_call!(otm_call_long);

            assert!(
                otm_put_long.otm()
                    && otm_put_short.otm()
                    && otm_call_short.otm()
                    && otm_call_long.otm(),
                "Puts and calls must be OTM!"
            );

            assert!(otm_put_long.strike() <= otm_put_short.strike() && otm_put_short.strike() <= otm_call_short.strike() && otm_call_short.strike() <= otm_call_long.strike(),
            "Iron Condor must have ordered strikes: OTM Put (long) <= OTM Put (short) <= OTM Call (short) <= OTM Call (long)");

            let price =
                self.price(otm_put_long) - self.price(otm_put_short) - self.price(otm_call_short)
                    + self.price(otm_call_long);
            let payoff = otm_put_long.payoff(Some(spot_price))
                - otm_put_short.payoff(Some(spot_price))
                - otm_call_short.payoff(Some(spot_price))
                + otm_call_long.payoff(Some(spot_price));
            (payoff, price)
        }
    }

    /* SPREAD */

    /// Short an ATM (at the money) call/put and long two OTM (out of the money) calls/puts.
    fn back_spread<'a, T: Option>(
        &'a self,
        short: &'a T,
        long1: &'a T,
        long2: &'a T,
    ) -> impl Fn(f64) -> (f64, f64) + 'a {
        move |spot_price| {
            check_same_expiration_date!(short, long1);
            check_same_expiration_date!(long1, long2);

            if long1.is_call() {
                check_is_call!(long2);
                check_is_call!(short);
                assert!(
                    short.atm() && long1.otm() && long2.otm(),
                    "Short call must be ATM and long calls must be OTM!"
                );
            } else {
                check_is_put!(long2);
                check_is_put!(short);
                assert!(
                    short.atm() && long1.otm() && long2.otm(),
                    "Short put must be ATM and long puts must be OTM!"
                );
            }

            let price = self.price(long1) + self.price(long2) - self.price(short);
            let payoff = long1.payoff(Some(spot_price)) + long2.payoff(Some(spot_price))
                - short.payoff(Some(spot_price));
            (payoff, price)
        }
    }

    /// The ladder strategy, also known as a Christmas tree, is a combination of three options of the same type (all calls or all puts) at three different strike prices.
    fn ladder<'a, T: Option>(
        &'a self,
        long: &'a T,
        short1: &'a T,
        short2: &'a T,
    ) -> impl Fn(f64) -> (f64, f64) + 'a {
        move |spot_price| {
            check_same_expiration_date!(long, short1);
            check_same_expiration_date!(short1, short2);

            if long.is_call() {
                check_is_call!(short1);
                check_is_call!(short2);
                assert!(
                    long.atm() && short1.otm() && short2.otm(),
                    "Long call must be ATM and short calls must be OTM!"
                );
            } else {
                check_is_put!(short1);
                check_is_put!(short2);
                assert!(
                    long.atm() && short1.otm() && short2.otm(),
                    "Long put must be ATM and short puts must be OTM!"
                );
            }

            let price = self.price(long) - self.price(short1) - self.price(short2);
            let payoff = long.payoff(Some(spot_price))
                - short1.payoff(Some(spot_price))
                - short2.payoff(Some(spot_price));
            (payoff, price)
        }
    }

    /// Short an ATM (at the money) call/put near-term expiration ("front-month") and long an ATM call/put with expiration one month later ("back-month").
    /// Used when a trader expects a gradual or sideways movement in the short term and has more direction bias over the life of the longer-dated option.
    fn calendar_spread<'a, T: Option>(
        &'a self,
        front_month: &'a T,
        back_month: &'a T,
    ) -> impl Fn(f64) -> (f64, f64) + 'a {
        log_warn!("Flaky implementation of calendar spread. Use with caution!");
        if back_month.time_to_maturity() < front_month.time_to_maturity() {
            log_warn!("Back month is the front month => continuing with the inverse order!");
            return self.calendar_spread(back_month, front_month);
        }
        move |spot_price| {
            if front_month.strike() != back_month.strike() {
                log_warn!("Strikes are not equal. Consider choosing equal strikes!");
            }

            if !front_month.atm() || !back_month.atm() {
                log_warn!("Options are not ATM. Consider choosing ATM options!");
            }

            if back_month.time_to_maturity() - front_month.time_to_maturity() > 0.083333334 {
                log_warn!("Time to maturity delta is more than 1 month. Consider choosing a shorter expiration date!");
            }

            let price = self.price(back_month) - self.price(front_month);
            let payoff = back_month.payoff(Some(spot_price)) - front_month.payoff(Some(spot_price));
            (payoff, price)
        }
    }

    /// Short an OTM (out of the money) call/put near-term expiration ("front-month") and long a further OTM call/put with expiration one month later ("back-month").
    /// At expiration of the front-month call/put, short another OTM call/put with the same expiration as the back-month call.
    fn diagonal_spread<'a, T: Option>(
        &'a self,
        front_month: &'a T,
        back_month_short: &'a T,
        back_month_long: &'a T,
    ) -> impl Fn(f64) -> (f64, f64) + 'a {
        move |spot_price| {
            if front_month.strike() != back_month_short.strike() {
                log_warn!("Front month short and back month long strikes are not equal. Consider choosing equal strikes!");
            }

            if !front_month.otm() || !back_month_long.otm() || !back_month_short.otm() {
                log_warn!("Options are not OTM. Consider choosing OTM options!");
            }

            if front_month.time_to_maturity() > 0.083333334 {
                log_warn!("Front month expires in more than 1 month. Consider choosing a shorter expiration date!");
            }

            if (front_month.time_to_maturity() - back_month_short.time_to_maturity()).abs() > 0.0027
            {
                log_warn!("Time to maturity delta between front-month and back-month short is more than 1 day. Consider choosing a shorter expiration date!");
            }

            // Ensure back-month long expires ~1 month after the front-month
            let time_delta = back_month_long.time_to_maturity() - front_month.time_to_maturity();
            if time_delta > 0.086073059 {
                log_warn!("Back-month long expires more than 1 month after front-month. Consider a shorter expiration!");
            }
            if time_delta < 0.080593607 {
                log_warn!("Back-month long expires less than 1 month after front-month. Consider a longer expiration!");
            }

            // Check if back-month long is further OTM than back-month short.
            if back_month_long.is_call() && back_month_long.strike() < back_month_short.strike()
                || back_month_long.is_put() && back_month_long.strike() > back_month_short.strike()
            {
                log_warn!("Back-month long is not further OTM than back-month short. Consider choosing further OTM options!");
            }

            if front_month.is_call() {
                check_is_call!(back_month_long);
                check_is_call!(back_month_short);
            } else {
                check_is_put!(back_month_long);
                check_is_put!(back_month_short);
            }

            // Adjust back-month short time to maturity after front-month expires
            let mut back_month_short = back_month_short.clone();
            back_month_short
                .set_instrument(back_month_short.instrument().clone().with_spot(spot_price));

            // Calculate the net price (cost/debit of entering the trade)
            let price = self.price(back_month_long)
                - self.price(front_month)
                - self.price(&back_month_short);

            // Compute the payoff at given spot price
            let payoff = back_month_long.payoff(Some(spot_price))
                - front_month.payoff(Some(spot_price))
                - back_month_short.payoff(Some(spot_price));

            (payoff, price)
        }
    }

    /* ALIASES */
    fn christmas_tree<'a, T: Option>(
        &'a self,
        long: &'a T,
        short1: &'a T,
        short2: &'a T,
    ) -> impl Fn(f64) -> (f64, f64) {
        log_info!("Ladder strategy is equivalent to the Christmas Tree strategy!");
        self.ladder(long, short1, short2)
    }
}
