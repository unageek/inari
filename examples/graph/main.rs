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
