// Plots the graph of a relation over ℝ².

mod graph;
mod interval_set;

use crate::graph::*;
use crate::interval_set::*;
use hexf::*;
use inari::{const_dec_interval, const_interval, interval, DecoratedInterval, Interval};

fn print_statistics_header() {
    println!(
        "  {:^14}  {:^30}  {:^30}",
        "Eval. Time (s)", "Area Proven (%)", "# of Evaluations"
    );
    println!("  {:-^14}  {:-^30}  {:-^30}", "", "", "");
}

fn print_statistics(cur: &GraphingStatistics, prev: &GraphingStatistics) {
    fn make_interval(x: f64) -> Interval {
        interval!(x, x).unwrap()
    }

    let i100 = const_interval!(100.0, 100.0);
    let ipx = make_interval(cur.pixels as f64);
    let area = i100 * make_interval(cur.pixels_proven as f64) / ipx;
    let delta_area = i100 * make_interval((cur.pixels_proven - prev.pixels_proven) as f64) / ipx;

    println!(
        "  {:>14.3}  {:>14}  {:>14}  {:>14}  {:>14}",
        cur.time_elapsed.as_secs_f64(),
        &format!("{:7.3}", area)[1..8],
        format!("(+{:>11})", &format!("{:7.3}", delta_area)[1..8]),
        cur.evaluations,
        format!("(+{:11})", cur.evaluations - prev.evaluations),
    );
}

fn f(x: TupperIntervalSet, y: TupperIntervalSet) -> EvaluationResult {
    const C0: DecoratedInterval = const_dec_interval!(0.0, 0.0);
    const C1: DecoratedInterval = const_dec_interval!(1.0, 1.0);
    const C3: DecoratedInterval = const_dec_interval!(3.0, 3.0);
    // Hull of 3.001.
    const C3_001: DecoratedInterval = const_dec_interval!(
        hexf64!("0x3.004189374bc6ap+0"),
        hexf64!("0x3.004189374bc6cp+0")
    );
    const C5: DecoratedInterval = const_dec_interval!(5.0, 5.0);
    const C8: DecoratedInterval = const_dec_interval!(8.0, 8.0);
    const C9: DecoratedInterval = const_dec_interval!(9.0, 9.0);
    const C25: DecoratedInterval = const_dec_interval!(25.0, 25.0);
    const C40: DecoratedInterval = const_dec_interval!(40.0, 40.0);
    const C81: DecoratedInterval = const_dec_interval!(81.0, 81.0);

    x.sin().eq(&y.cos())

    //(&y - &x).eq(&(&x + &y).exp().sin())

    //let c0 = TupperIntervalSet::from(C0);
    //let c1 = TupperIntervalSet::from(C1);
    //(&(&(&x.sqr() + &y.sqr()) - &c1) * &(&y + &x.cos())).eq(&c0)

    // From Fig. 1a in Tupper (2001)
    //y.lt(&x.sqrt())

    // From Fig. 17 in Tupper (2001)
    //y.eq(&(&x - &x.tan(None).atan()))

    // Some intriguing examples from GrafEq
    // Icons after each filename:
    //   🐌: Very slow to finish

    // 📂 Single Relation/Abstract/Simple/
    //  📄 Parabolic Waves.gqs
    //(&x.sqr() + &y.sqr()).sqrt().sin().abs().eq(&x.cos().abs())

    //  📄 Pythagorean Pairs.gqs [🐌]
    //let a = &x.floor(Some(0)).sqr() + &y.floor(Some(1)).sqr();
    //a.eq(&a.sqrt().floor(None).sqr())

    //  📄 Pythagorean Triples.gqs [🐌]
    //let c25 = TupperIntervalSet::from(C25);
    //(&x.floor(None).sqr() + &y.floor(None).sqr()).eq(&c25)

    //  📄 Wave.gqs (GrafEq seems to handle sin(x)/x specially)
    //y.eq(&x.sin_over_x())

    // 📂 Single Relation/Abstract/Traditionally Difficult/
    //  📄 Infinite Frequency.gqs
    //let c40 = TupperIntervalSet::from(C40);
    //y.eq(&c40.div(&x, None).sin())

    //  📄 O Spike.gqs
    //let c3 = TupperIntervalSet::from(C3);
    //let c3_001 = TupperIntervalSet::from(C3_001);
    //let c81 = TupperIntervalSet::from(C81);
    //(&(&x * &(&x - &c3)).div(&(&x - &c3_001), None).sqr()
    //    + &(&y * &(&y - &c3)).div(&(&y - &c3_001), None).sqr())
    //    .eq(&c81)

    //  📄 Solid Disc.gqs
    //let c81 = TupperIntervalSet::from(C81);
    //let a = &c81 - &(&x.sqr() + &y.sqr());
    //a.eq(&a.abs())

    //  📄 Spike.gqs
    //let c3 = TupperIntervalSet::from(C3);
    //let c3_001 = TupperIntervalSet::from(C3_001);
    //y.eq(&(&x * &(&x - &c3)).div(&(&x - &c3_001), None))

    //  📄 Step.gqs
    //let c1 = TupperIntervalSet::from(C1);
    //let c9 = TupperIntervalSet::from(C9);
    //y.eq(&(&(&c9.sqr() * &c9) * &(&x - &c1)).atan())

    //  📄 Upper Triangle.gqs
    //let a = &x + &y;
    //a.eq(&a.abs())

    // 📂 Single Relation/Enumerations/Trees/
    //  📄 bi-infinite binary tree.gqs [🐌]
    //let c0 = TupperIntervalSet::from(C0);
    //let frac_pi_2 = TupperIntervalSet::from(DecoratedInterval::FRAC_PI_2);
    //let frac_pi_4 = TupperIntervalSet::from(DecoratedInterval::FRAC_PI_4);
    //let fy = y.floor(Some(0));
    //let a = &(&fy.exp2() * &x) - &frac_pi_2;
    //let b = &frac_pi_4 * &(&y - &fy);
    //(&(&a + &b).sin() * &(&a - &b).sin()).eq(&c0)

    // 📂 Single Relation/Linelike/
    //  📄 Frontispiece #2.gqs
    //let c0 = TupperIntervalSet::from(C0);
    //let xy = &x * &y;
    //let a = x.div(&x.cos(), Some(0));
    //let b = y.div(&y.cos(), Some(1));
    //let c = xy.div(&xy.cos(), Some(2));
    //let apb = &a + &b;
    //let amb = &a - &b;
    //(&(&(&apb + &c) * &(&apb - &c)) * &(&(&amb + &c) * &(&amb - &c))).eq(&c0)

    //  📄 Frontispiece.gqs
    //let c0 = TupperIntervalSet::from(C0);
    //let xy = &x * &y;
    //let a = x.sin_over_x().recip(Some(0));
    //let b = y.sin_over_x().recip(Some(1));
    //let c = xy.sin_over_x().recip(Some(2));
    //let apb = &a + &b;
    //let amb = &a - &b;
    //(&(&(&apb + &c) * &(&apb - &c)) * &(&(&amb + &c) * &(&amb - &c))).eq(&c0)

    //  📄 Hair.gqs
    // Should be plotted over Region::new(4.0, 6.5, 2.0, 4.5).
    //let c0 = TupperIntervalSet::from(C0);
    //let cx = x.cos();
    //let cy = y.cos();
    //let sx = x.sin();
    //let sy = y.sin();
    //let a1 = &x + &sy;
    //let a2 = &x - &sy;
    //let b1 = &sx + &y;
    //let b2 = &sx - &y;
    //let c1 = &sx + &cy;
    //let c2 = &sx - &cy;
    //let d1 = &sy + &cx;
    //let d2 = &sy - &cx;
    //let e1 = (&a1 * &b1).sin();
    //let e2 = (&a1 * &b2).sin();
    //let e3 = (&a2 * &b1).sin();
    //let e4 = (&a2 * &b2).sin();
    //let f1 = -&(&c1 * &d1).sin().cos();
    //let f2 = -&(&c1 * &d2).sin().cos();
    //let f3 = -&(&c2 * &d1).sin().cos();
    //let f4 = -&(&c2 * &d2).sin().cos();
    //(&(&e1 + &f1)
    //    * &(&(&e1 + &f2)
    //        * &(&(&e1 + &f3)
    //            * &(&(&e1 + &f4)
    //                * &(&(&e2 + &f1)
    //                    * &(&(&e2 + &f2)
    //                        * &(&(&e2 + &f3)
    //                            * &(&(&e2 + &f4)
    //                                * &(&(&e3 + &f1)
    //                                    * &(&(&e3 + &f2)
    //                                        * &(&(&e3 + &f3)
    //                                            * &(&(&e3 + &f4)
    //                                                * &(&(&e4 + &f1)
    //                                                    * &(&(&e4 + &f2)
    //                                                        * &(&(&e4 + &f3)
    //                                                            * &(&e4 + &f4))))))))))))))))
    //    .eq(&c0)

    //  📄 Highwire.gqs [🐌]
    //(&(&x * &x.cos()) - &(&y * &y.sin()))
    //    .abs()
    //    .eq(&(&(&x * &y.cos()) - &(&y * &x.sin())).abs())

    //  📄 Trapezoidal Fortress.gqs [🐌]
    //(&(&x * &x.cos()) + &(&y * &y.sin()))
    //    .abs()
    //    .eq(&(&(&x * &y.cos()) - &(&y * &x.sin())))

    // 📂 Single Relation/Solid/
    //  📄 Sharp Threesome.gqs
    //let c0 = TupperIntervalSet::from(C0);
    //let c5 = TupperIntervalSet::from(C5);
    //let c8 = TupperIntervalSet::from(C8);
    //let xp5 = &x + &c5;
    //let xm5 = &x - &c5;
    //let yp5 = &y + &c5;
    //let ym5 = &y - &c5;
    //let a = (&xp5.sqr() + &y.sqr()).sqrt().sin();
    //let b = (&c8 * &y.div(&xp5, None).atan()).cos();
    //let c = (&xm5.sqr() + &ym5.sqr()).sqrt().sin();
    //let d = (&c8 * &ym5.div(&xm5, None).atan()).cos();
    //let e = (&x.sqr() + &yp5.sqr()).sqrt().sin();
    //let f = (&c8 * &yp5.div(&x, None).atan()).cos();
    //(&a * &(&b * &(&c * &(&d * &(&e * &f))))).gt(&c0)

    //  📄 The Disco Hall.gqs
    //(&x + &y).abs().sin().gt(&x.sqr().cos().max(&y.sqr().sin()))
}

fn main() {
    let mut g = Graph::new(
        Relation(f),
        Region::new(-10.0, 10.0, -10.0, 10.0),
        1024,
        1024,
    );
    let mut prev_stat = g.get_statistics();

    print_statistics_header();

    loop {
        let result = g.step();

        let stat = g.get_statistics();
        print_statistics(&stat, &prev_stat);
        prev_stat = stat;

        let im = g.get_image();
        im.save("/mnt/e/graph.png").unwrap();

        match result {
            Ok(true) => break,
            Err(e) => {
                println!("Warning: {}", e);
                break;
            }
            _ => (),
        }
    }
}
