// Plots the graph of a relation over ℝ².

mod ast;
mod dyn_relation;
mod graph;
mod interval_set;
mod parse;
mod visitor;

use crate::{dyn_relation::*, graph::*};
use clap::{App, AppSettings, Arg};
use inari::{const_interval, interval, Interval};

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

// [🐌]: Takes a long time to finish graphing.

// "sin(x) == cos(y)"

// "y - x == sin(exp(x + y))"

// "(x^2 + y^2) == 1 || y == -cos(x)"

// From Fig. 1a in Tupper (2001)
// "y < sqrt(x)"

// From Fig. 17 in Tupper (2001) [🐌]
// "y == x - atan(tan(x))"

// Some intriguing examples from GrafEq

// 📂 Single Relation/Abstract/Simple/
//  📄 Parabolic Waves.gqs
// "abs(sin(sqrt(x^2 + y^2))) == abs(cos(x))"

//  📄 Pythagorean Pairs.gqs [🐌]
// "floor(x)^2 + floor(y)^2 == floor(sqrt(floor(x)^2 + floor(y)^2))^2"

//  📄 Pythagorean Triples.gqs [🐌]
// "floor(x)^2 + floor(y)^2 == 25"

// 📂 Single Relation/Abstract/Traditionally Difficult/
//  📄 Infinite Frequency.gqs
// "y == sin(40/x)"

//  📄 O Spike.gqs
// "(x*(x - 3)/(x - 3.001))^2 + (y*(y - 3)/(y - 3.001))^2 == 81"

//  📄 Solid Disc.gqs
// "81 - x^2 - y^2 == abs(81 - x^2 - y^2)"

//  📄 Spike.gqs
// "y == x*(x - 3)/(x - 3.001)"

//  📄 Step.gqs
// TODO

//  📄 Upper Triangle.gqs
// "x + y == abs(x + y)"

//  📄 Wave.gqs (GrafEq seems to handle sin(x)/x specially like this)
//y.eq(&x.sin_over_x())

// 📂 Single Relation/Enumerations/Trees/
//  📄 bi-infinite binary tree.gqs [🐌]
// "sin(exp2(floor(y))*x + pi/4*(y - floor(y)) - pi/2) == 0 || sin(exp2(floor(y))*x - pi/4*(y - floor(y)) - pi/2) == 0"

// 📂 Single Relation/Linelike/
//  📄 Frontispiece #2.gqs
//let c0 = TupperIntervalSet::from(C0);
//let xy = &x * &y;
//let a = x.div(&x.cos(), Some(0));
//let b = y.div(&y.cos(), Some(1));
//let c = xy.div(&xy.cos(), Some(2));
//let apb = &a + &b;
//let amb = &a - &b;
//(&apb + &c).eq(&c0) | (&apb - &c).eq(&c0) | (&amb + &c).eq(&c0) | (&amb - &c).eq(&c0)

//  📄 Frontispiece.gqs
//let c0 = TupperIntervalSet::from(C0);
//let xy = &x * &y;
//let a = x.sin_over_x().recip(Some(0));
//let b = y.sin_over_x().recip(Some(1));
//let c = xy.sin_over_x().recip(Some(2));
//let apb = &a + &b;
//let amb = &a - &b;
//(&apb + &c).eq(&c0) | (&apb - &c).eq(&c0) | (&amb + &c).eq(&c0) | (&amb - &c).eq(&c0)

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
//(&e1 + &f1).eq(&c0)
//    | (&e1 + &f2).eq(&c0)
//    | (&e1 + &f3).eq(&c0)
//    | (&e1 + &f4).eq(&c0)
//    | (&e2 + &f1).eq(&c0)
//    | (&e2 + &f2).eq(&c0)
//    | (&e2 + &f3).eq(&c0)
//    | (&e2 + &f4).eq(&c0)
//    | (&e3 + &f1).eq(&c0)
//    | (&e3 + &f2).eq(&c0)
//    | (&e3 + &f3).eq(&c0)
//    | (&e3 + &f4).eq(&c0)
//    | (&e4 + &f1).eq(&c0)
//    | (&e4 + &f2).eq(&c0)
//    | (&e4 + &f3).eq(&c0)
//    | (&e4 + &f4).eq(&c0)

//  📄 Highwire.gqs [🐌]
// "abs(x*cos(x) - y*sin(y)) == abs(x*cos(y) - y*sin(x))"

//  📄 Trapezoidal Fortress.gqs [🐌]
// "abs(x*cos(x) + y*sin(y)) == x*cos(y) - y*sin(x)"

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
// "sin(abs(x + y)) > max(cos(x^2), sin(y^2))"

fn main() {
    let matches = App::new("inari-graph")
        .setting(AppSettings::AllowLeadingHyphen)
        .arg(Arg::new("relation").index(1))
        .arg(Arg::new("output").index(2))
        .arg(
            Arg::with_name("bounds")
                .short('b')
                .number_of_values(4)
                .default_values(&["-10", "10", "-10", "10"])
                .value_names(&["XMIN", "XMAX", "YMIN", "YMAX"]),
        )
        .arg(
            Arg::with_name("size")
                .short('s')
                .number_of_values(2)
                .default_values(&["1024", "1024"])
                .value_names(&["WIDTH", "HEIGHT"]),
        )
        .get_matches();

    let relation = matches.value_of_t_or_exit::<String>("relation");
    let output = matches.value_of_t_or_exit::<String>("output");
    let bounds = matches.values_of_t_or_exit::<f64>("bounds");
    let size = matches.values_of_t_or_exit::<u32>("size");

    let mut rel = DynRelation::new(&relation);
    let mut g = Graph::new(
        Relation(|x, y| rel.evaluate(x, y)),
        Region::new(bounds[0], bounds[1], bounds[2], bounds[3]),
        size[0],
        size[1],
    );
    let mut prev_stat = g.get_statistics();

    print_statistics_header();

    loop {
        let result = g.step();

        let stat = g.get_statistics();
        print_statistics(&stat, &prev_stat);
        prev_stat = stat;

        let im = g.get_image();
        im.save(&output).unwrap();

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
