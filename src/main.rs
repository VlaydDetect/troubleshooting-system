use ndarray::prelude::*;

mod information_criterion;
mod error;
mod prelude;
mod math;
mod graph;

use information_criterion::*;
use crate::prelude::{f, W};

// use petgraph::Graph;
// use plotters::prelude::*;

fn main() {
    let matrix = array![
        [0, 1, 1, 1, 1],
        [0, 0, 0, 1, 1],
        [1, 1, 0, 1, 1],
        [1, 1, 0, 0, 1],
        [0, 0, 0, 0, 0],
    ];
    
    println!("{:?}", remove_rows(&matrix, &[2, 4]));

    let matrix: Array2<bool> = W(matrix).into();

    // println!("{:?}", num_elems(&W(matrix).into()))
    // println!("{}", information_of(3, &matrix));
    // println!("Scheme: {:?}", make_tree(&matrix).unwrap())

    // let v = vec![false, true, true, true, true, false, false, false, true, true, true, true, false, false, true];
    // let a = Array::from_vec(v).into_shape((3, 5));
    // println!("{:?}", a);

    // // Создаем новый граф
    // let mut graph = Graph::<&str, &str>::new();
    //
    // // Добавляем узлы
    // let a = graph.add_node("1");
    // let b = graph.add_node("2");
    // let c = graph.add_node("3");
    // let c = graph.add_node("4");
    //
    // // Добавляем ребра
    // graph.add_edge(a, b, "14");
    // graph.add_edge(a, c, "34");
    // graph.add_edge(b, c, "21");
    //
    // // Визуализация графа
    // let root = BitMapBackend::new("graph.png", (900, 900)).into_drawing_area();
    // root.fill(&WHITE).unwrap();
    // let mut chart = ChartBuilder::on(&root)
    //     .caption("Граф", ("sans-serif", 50).into_font())
    //     .build_cartesian_2d(0..6, 0..6).unwrap();
    //
    // chart.configure_mesh().draw().unwrap();
    //
    // // Отображаем узлы и ребра
    // chart.draw_series(
    //     graph.node_indices().map(|n| {
    //         let (x, y) = (graph[n].parse::<i32>().unwrap(), graph[n].parse::<i32>().unwrap());
    //         Circle::new((x, y), 15, Into::<ShapeStyle>::into(&RED))
    //     })
    // ).unwrap();
    //
    // chart.draw_series(
    //     graph.node_indices().map(|n| {
    //         let (x, y) = (graph[n].parse::<i32>().unwrap(), graph[n].parse::<i32>().unwrap());
    //         Text::new(f!("{}", graph[n]), (x, y), ("sans-serif", 15).into_font())
    //     })
    // ).unwrap();
    //
    // chart.draw_series(
    //     graph.edge_references().map(|e| {
    //         let (start, end) = (graph[e.source()].parse::<i32>().unwrap(), graph[e.target()].parse::<i32>().unwrap());
    //         PathElement::new(vec![(0, start), (end, end)], Into::<ShapeStyle>::into(&BLACK))
    //     })
    // ).unwrap();
    //
    // // Сохраняем в файл
    // root.present().unwrap();
}
