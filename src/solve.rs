use std::collections::HashMap;

use boolean_expression::{Expr, BDD};
use serde::Serialize;

#[derive(Serialize)]
pub struct Dominance {
    #[serde(skip_serializing)]
    dom_expr: Expr<String>,
    #[serde(skip_serializing)]
    transposed_dom_expr: Expr<String>,
    translation_table: HashMap<String, String>,
    resolve_with_bdd: bool,
    resolution_expression_str: String,
    sat_assignment: Option<HashMap<String, bool>>,
    #[serde(skip_serializing)]
    produce_graph: bool,
    #[serde(skip_serializing)]
    graph_string: Option<String>,
}

impl Dominance {
    pub fn new(
        dom_expr: Expr<String>,
        translation_table: HashMap<String, String>,
        resolve_with_bdd: bool,
        produce_graph: bool,
    ) -> Self {
        let transposed_dom_expr = dom_expr.map(|s| {
            let mut cloned = s.clone();
            cloned.push_str("_prime");
            cloned
        });
        let resolution_expression_str = String::new();
        let graph_string = None;
        Dominance {
            dom_expr,
            transposed_dom_expr,
            translation_table,
            resolve_with_bdd,
            resolution_expression_str,
            sat_assignment: None,
            produce_graph,
            graph_string,
        }
    }

    pub fn resolve(&mut self) {
        let mut resolved_expr = Expr::and(self.dom_expr.clone(), self.transposed_dom_expr.clone());
        match self.resolve_with_bdd {
            true => resolved_expr = resolved_expr.simplify_via_bdd(),
            false => resolved_expr = resolved_expr.simplify_via_laws(),
        }
        let mut dom_bdd = BDD::new();
        let f = dom_bdd.from_expr(&resolved_expr);
        self.sat_assignment = dom_bdd.sat_one(f);
        self.resolution_expression_str = format!("{:?}", resolved_expr);
        if self.produce_graph {
            self.graph_string = Some(dom_bdd.to_dot(f));
        }
    }

    pub fn get_graph_string(&mut self) -> Option<String> {
        self.graph_string.take()
    }
}
