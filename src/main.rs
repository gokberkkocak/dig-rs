use std::fs;

use structopt::StructOpt;

mod parse;
mod solve;

use parse::Parser;
use solve::Dominance;

/// Dominance Incomparability Generator
#[derive(StructOpt, Debug)]
#[structopt(name = "dig")]
struct Opt {
    /// Input Essence model
    #[structopt(short, long)]
    input: String,
    /// Output json file
    #[structopt(short, long)]
    output: String,
    /// Simplify via bdd instead of laws
    #[structopt(short, long)]
    bdd: bool,
    /// Generate dot plot
    #[structopt(short, long)]
    graph: bool,
}

fn main() {
    let opt = Opt::from_args();
    let resolve_with_bdd = opt.bdd;
    let (expr, translation_table) = Parser::parse_file(&opt.input);
    let output = opt.output;
    let produce_graph = opt.graph;
    let mut dom = Dominance::new(expr, translation_table, resolve_with_bdd, produce_graph);
    dom.resolve();
    if let Some(graph) = dom.get_graph_string() {
        fs::write(format!("{}.dot", output.split('.').next().unwrap()), graph)
            .expect("Unable to write file");
    }
    let json = serde_json::to_string(&dom).unwrap();
    fs::write(output, json).expect("Unable to write file");
}

#[cfg(test)]
mod tests {
    use boolean_expression::Expr;
    #[test]
    fn test_simple() {
        let dom_expr = Expr::not(Expr::or(
            Expr::not(Expr::Terminal("a")),
            Expr::Terminal("b"),
        ));
        let inverse_dom_expr = Expr::not(Expr::or(
            Expr::not(Expr::Terminal("a_prime")),
            Expr::Terminal("b_prime"),
        ));
        let total: Expr<&str> = Expr::and(dom_expr, inverse_dom_expr);
        let total_simplified = total.simplify_via_laws();
        let expected = Expr::and(
            Expr::and(Expr::Terminal("a"), Expr::not(Expr::Terminal("b"))),
            Expr::and(
                Expr::Terminal("a_prime"),
                Expr::not(Expr::Terminal("b_prime")),
            ),
        );
        assert_eq!(total_simplified, expected);
    }
}
