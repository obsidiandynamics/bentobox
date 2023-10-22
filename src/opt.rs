#[derive(Clone, Debug)]
pub struct GradientDescentConfig {
    pub init_value: f64,
    pub step: f64,
    pub min_step: f64,
    pub max_steps: u64,
}

#[derive(Debug)]
pub struct GradientDescentOutcome {
    pub iterations: u64,
    pub optimal_value: f64,
    pub optimal_residual: f64
}

pub fn gd(config: GradientDescentConfig, mut loss_f: impl FnMut(f64) -> f64) -> GradientDescentOutcome{
    let mut iterations = 0;
    let mut residual = loss_f(config.init_value);
    let (mut value, mut step) = (config.init_value, config.step);
    let (mut optimal_value, mut optimal_residual) = (value, residual);
    let mut boost = 1.0;
    // let mut gradient: f64 = 1.0;
    while iterations < config.max_steps {
        iterations += 1;
        let new_value = value + step * boost;// * f64::min(gradient.abs(), 100.0);
        let new_residual = loss_f(new_value);
        let gradient = (new_residual - residual) / (new_value - value);
        println!("iterations: {iterations}, value: {value}, residual: {residual}, step: {step}, new_value: {new_value}, new_residual: {new_residual}, gradient: {gradient}");

        if new_residual > residual {
            step = -step * 0.5;
            if step.abs() < config.min_step {
                break;
            }
        } else if new_residual < optimal_residual {
            // boost = f64::min(new_residual / (optimal_residual - new_residual), 10.0);
            boost =  f64::min(gradient.abs(), 10.0);
            println!("optimal_residual: {optimal_residual}, new_residual: {new_residual}, boost: {boost}, diff: {}", optimal_residual - new_residual);
            optimal_residual = new_residual;
            optimal_value = new_value;
        } else if (new_residual - residual).abs() <= f64::EPSILON {
            break;
        }
        residual = new_residual;
        value = new_value;
    }
    GradientDescentOutcome {
        iterations,
        optimal_value,
        optimal_residual
    }
}

#[derive(Clone, Debug)]
pub struct MvGdConfig {
    pub init_value: f64,
    pub step: f64,
    pub min_step: f64,
}

#[derive(Default, Clone)]
struct MvGdState {

}

pub fn mv_gd(configs: &[MvGdConfig]) {
    let mut optimal_values = vec![0.0; configs.len()];
    let mut states = vec![MvGdState::default(); configs.len()];
    for (index, config) in configs.iter().enumerate() {

    }
}

#[cfg(test)]
mod tests {
    use assert_float_eq::*;
    use super::*;

    #[test]
    fn gd_sqrt() {
        let config = GradientDescentConfig {
            init_value: 0.0,
            step: 0.1,
            min_step: 0.00001,
            max_steps: 100,
        };
        let outcome = gd(config.clone(), |value| (81.0 - value.powi(2)).powi(2));
        assert_float_absolute_eq!(9.0, outcome.optimal_value, config.min_step);
    }
}