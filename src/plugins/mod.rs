pub mod emitters;
pub mod transformers;

pub trait Plugin {}

pub trait TransformerPlugin: Plugin {}

pub trait EmitterPlugin: Plugin {}
