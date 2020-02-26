pub mod epidemic;
pub mod flooding;

use crate::cla::ClaSender;
use crate::core::bundlepack::BundlePack;
use std::fmt::Debug;
use std::fmt::Display;

pub trait RoutingAgent: Debug + Send + Display {
    fn sending_failed(&mut self, _bundle_id: &str, _cla_sender: &ClaSender) {
        unimplemented!();
    }
    fn sender_for_bundle(&mut self, _bp: &BundlePack) -> (Vec<ClaSender>, bool) {
        unimplemented!();
    }
}
pub fn routing_algorithms() -> Vec<&'static str> {
    vec!["flooding", "epidemic"]
}

pub fn new(routingagent: &str) -> Box<dyn RoutingAgent> {
    match routingagent {
        "flooding" => Box::new(flooding::FloodingRoutingAgent::new()),
        "epidemic" => Box::new(epidemic::EpidemicRoutingAgent::new()),
        _ => panic!("Unknown routing agent {}", routingagent),
    }
}
