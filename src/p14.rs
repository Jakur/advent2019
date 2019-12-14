use super::*;

use petgraph::algo::toposort;

use petgraph::graph::NodeIndex;
use petgraph::Graph;

#[derive(Debug)]
struct Cost<'a> {
    quantity: i64,
    name: &'a str,
}

impl<'a> Cost<'a> {
    fn new(s: &'a str) -> Cost<'a> {
        let mut sp = s.split_whitespace();
        let quantity = sp.next().unwrap().parse().unwrap();
        let name = sp.next().unwrap();
        Cost { quantity, name }
    }
}

#[derive(Debug)]
struct Recipe<'a> {
    ingredients: Vec<Cost<'a>>,
    output: Cost<'a>,
}

impl<'a> Recipe<'a> {
    fn new(ingredients: Vec<Cost<'a>>, output: Cost<'a>) -> Recipe<'a> {
        Recipe {
            ingredients,
            output,
        }
    }
}

fn parse_graph<'a>(input: &'a str) -> HashMap<&'a str, Recipe<'a>> {
    input
        .lines()
        .map(|line| {
            let mut sp = line.split(" => ");
            let ingred = sp.next().unwrap().split(",");
            let ingred: Vec<_> = ingred.map(|s| Cost::new(s)).collect();
            let out_str = sp.next().unwrap();
            let output = Cost::new(out_str);
            (output.name, Recipe::new(ingred, output))
        })
        .collect()
}

fn top_down(
    fuel_q: i64,
    map: &HashMap<&str, Recipe>,
    graph: &Graph<&str, &str>,
    topo: &[NodeIndex],
) -> i64 {
    let mut req = HashMap::new();
    let mut reserves = HashMap::new();
    req.insert("FUEL", fuel_q);
    for r in topo[1..].iter().rev() {
        let next_name = graph[*r];
        let next_q = *req.get(next_name).unwrap();

        let recipe = &map[next_name];
        let rat = (next_q / recipe.output.quantity) + (next_q % recipe.output.quantity != 0) as i64;
        for ing in recipe.ingredients.iter() {
            *req.entry(ing.name).or_insert(0) += ing.quantity * rat;
            if next_q % recipe.output.quantity != 0 {
                *reserves.entry(ing.name).or_insert(0) += ing.quantity;
            }
        }
        req.remove(next_name);
    }
    req["ORE"]
}

pub fn p14(input: &str) -> Answer {
    let map = parse_graph(input);
    let mut graph = Graph::<&str, &str>::new();
    let mut indices = HashMap::new();
    let x = graph.add_node("ORE");
    indices.insert("ORE", x);
    for (_k, v) in map.iter() {
        let index = graph.add_node(v.output.name);
        indices.insert(v.output.name, index);
    }
    for (_k, v) in map.iter() {
        let output = v.output.name;
        for input in v.ingredients.iter() {
            let a = indices[input.name];
            let b = indices[output];
            graph.add_edge(a, b, "");
        }
    }

    let topo = toposort(&graph, None).unwrap();
    let ans1 = top_down(1, &map, &graph, &topo[..]);

    // Bottom up for approximate solution
    let mut ore_costs = HashMap::new();
    ore_costs.insert("ORE", 1.0);
    for node_index in topo[1..].iter() {
        let mut cost = 0.0;
        let out_str = graph[*node_index];
        let rep = &map[out_str];
        for c in rep.ingredients.iter() {
            cost += (c.quantity as f64) * ore_costs[c.name];
        }
        ore_costs.insert(out_str, cost / (rep.output.quantity as f64));
    }

    let float_sol = 10.0f64.powf(12.0) / ore_costs["FUEL"];
    let mut ans2 = float_sol as i64;
    while top_down(ans2, &map, &graph, &topo[..]) > 1_000_000_000_000 {
        ans2 -= 1;
    }
    Answer::new(ans1, ans2)
}