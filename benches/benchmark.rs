use criterion::{black_box, criterion_group, criterion_main, Criterion};

use learning_graph::models::Command;
use learning_graph::process_command::CommandProcessor;
use learning_graph::startup::deserialization::{read_commands, read_links, read_places};
use learning_graph::startup::graph_builder;

fn run(processor: CommandProcessor, commands: Vec<Command>) {
    let _: Vec<String> = commands.into_iter().map(|x| processor.process(x)).collect();
}

fn build() -> (CommandProcessor, Vec<Command>) {
    let places_path = "Places.csv";
    let links_path = "Links.csv";
    let command_path = "Commands.txt";

    let nodes = read_places(places_path);
    let links = read_links(links_path);

    let (graph, map) = graph_builder::build(nodes, links);

    let commands = read_commands(command_path).unwrap();

    let processor = CommandProcessor::new(graph, map);
    (processor, commands)
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("run-all-commands", |b| {
        let (proc, commands) = build();

        b.iter(|| {
            run(black_box(proc.clone()), black_box(commands.clone()));
        })
    });

    c.bench_function("build-graph", |b| {
        b.iter(|| {
            build();
        })
    });

    c.bench_function("build-and-run-all-commands", |b| {
        b.iter(|| {
            let (proc, commands) = build();
            run(black_box(proc.clone()), black_box(commands.clone()));
        })
    });
}

criterion_group! {
    name = benches;
    // This can be any expression that returns a `Criterion` object.
    config = Criterion::default().significance_level(0.1).sample_size(500);
    targets = criterion_benchmark
}
criterion_main!(benches);
