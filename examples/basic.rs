use bentobox::capture::Capture;
use bentobox::{mc, overround};
use bentobox::mc::DilatedProbs;
use bentobox::probs::SliceExt;
use bentobox::selection::{Runner, Selection};

fn main() {
    // probs taken from a popular website
    let mut probs = vec![
        1.0 / 2.0,
        1.0 / 12.0,
        1.0 / 3.0,
        1.0 / 9.50,
        1.0 / 7.50,
        1.0 / 126.0,
        1.0 / 23.0,
        1.0 / 14.0,
    ];

    // force probs to sum to 1 and extract the approximate overround used (multiplicative method assumed)
    let win_overround = probs.normalise(1.0);

    println!("fair probs: {probs:?}");
    println!("overround: {win_overround:.3}");

    // create an MC engine for reuse
    let mut engine = mc::MonteCarloEngine::default()
        .with_iterations(100_000)
        .with_probs(Capture::Owned(
            DilatedProbs::default()
                .with_win_probs(Capture::Borrowed(&probs))
                .with_podium_places(4)
                .into(),
        ));

    // simulate top-N rankings for all runners
    // NOTE: rankings and runner numbers are zero-based
    for runner in 0..probs.len() {
        println!("runner: {runner}");
        for rank in 0..4 {
            let frac = engine.simulate(&vec![Selection::Span {
                runner: Runner::index(runner),
                ranks: 0..rank + 1,
            }]);
            println!(
                "    rank: 0..={rank}, prob: {}, fair price: {:.3}, market odds: {:.3}",
                frac.quotient(),
                1.0 / frac.quotient(),
                1.0 / frac.quotient() / win_overround
            );
        }
    }

    // simulate a same-race multi for a chosen selection vector
    let selections = vec![
        Runner::number(1).top(1),
        Runner::number(2).top(2),
        Runner::number(3).top(3),
    ];
    let frac = engine.simulate(&selections);
    println!(
        "probability of {selections:?}: {}, fair price: {:.3}, market odds: {:.3}",
        frac.quotient(),
        1.0 / frac.quotient(),
        overround::apply_with_cap(1.0 / frac.quotient(), win_overround.powi(selections.len() as i32))
    );
}
