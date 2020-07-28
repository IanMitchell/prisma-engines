mod ast;
mod enum_renderer;

mod field_renderer;
mod object_renderer;
mod schema_renderer;
mod type_renderer;

use crate::dmmf::DMMFMapping;
use enum_renderer::*;
use field_renderer::*;
use object_renderer::*;
use query_core::schema::*;
use schema_renderer::*;
use std::{collections::HashSet, sync::Weak};
use type_renderer::*;

pub use ast::*;

pub struct DMMFQuerySchemaRenderer;

impl QuerySchemaRenderer<(DMMFSchema, Vec<DMMFMapping>)> for DMMFQuerySchemaRenderer {
    fn render(query_schema: QuerySchemaRef) -> (DMMFSchema, Vec<DMMFMapping>) {
        let mut ctx = RenderContext::new();
        query_schema.into_renderer().render(&mut ctx);

        ctx.finalize()
    }
}

pub struct RenderContext<'a> {
    /// Aggregator for query schema
    schema: DMMFSchema,

    /// Aggregator for mappings
    mappings: Vec<DMMFMapping>,

    /// Prevents double rendering of elements that are referenced multiple times.
    /// Names of input / output types / enums / models are globally unique.
    rendered: HashSet<String>,

    /// The queue of elements that still need to be rendered. This is filled in
    /// with the top-level initially, and each iteration will add its unrendered
    /// dependencies, until there is no more.
    to_be_rendered: Vec<Box<dyn Renderer<'a>>>,
}

impl<'a> RenderContext<'a> {
    pub fn new() -> Self {
        RenderContext {
            schema: DMMFSchema::new(),
            mappings: vec![],
            rendered: HashSet::new(),
            to_be_rendered: Vec::new(),
        }
    }

    pub fn finalize(self) -> (DMMFSchema, Vec<DMMFMapping>) {
        let mut schema = self.schema;

        schema.root_query_type = "Query".into();
        schema.root_mutation_type = "Mutation".into();

        (schema, self.mappings)
    }

    pub fn already_rendered(&self, cache_key: &str) -> bool {
        self.rendered.contains(cache_key)
    }

    pub fn mark_as_rendered(&mut self, cache_key: String) {
        self.rendered.insert(cache_key);
    }

    pub fn add_enum(&mut self, name: String, dmmf_enum: DMMFEnum) {
        self.schema.enums.push(dmmf_enum);
        self.mark_as_rendered(name);
    }

    pub fn add_input_type(&mut self, input_type: DMMFInputType) {
        self.mark_as_rendered(input_type.name.clone());
        self.schema.input_types.push(input_type);
    }

    pub fn add_output_type(&mut self, output_type: DMMFOutputType) {
        self.mark_as_rendered(output_type.name.clone());
        self.schema.output_types.push(output_type);
    }

    pub fn add_mapping(&mut self, name: String, operation: Option<&SchemaQueryBuilder>) {
        if let Some(SchemaQueryBuilder::ModelQueryBuilder(m)) = operation {
            let model_name = m.model.name.clone();
            let tag_str = format!("{}", m.tag);
            let mapping = self.mappings.iter().find(|mapping| mapping.model_name == model_name);

            match mapping {
                Some(ref existing) => existing.add_operation(tag_str, name.clone()),
                None => {
                    let new_mapping = DMMFMapping::new(model_name);

                    new_mapping.add_operation(tag_str, name.clone());
                    self.mappings.push(new_mapping);
                }
            };
        }
    }
}

pub trait Renderer<'a> {
    fn render(&self, ctx: &mut RenderContext<'a>);
}

trait IntoRenderer<'a> {
    fn into_renderer(&'a self) -> Box<dyn Renderer<'a> + 'a>;
}

impl<'a> IntoRenderer<'a> for QuerySchemaRef {
    fn into_renderer(&'a self) -> Box<dyn Renderer<'a> + 'a> {
        Box::new(DMMFSchemaRenderer::new(self))
    }
}

// impl<'a> IntoRenderer<'a> for OutputTypeRef {
//     fn into_renderer(&'a self) -> Box<dyn Renderer<'a> + 'a> {
//         Box::new(DMMFTypeRenderer::Output(self))
//     }
// }

// impl<'a> IntoRenderer<'a, DMMFTypeInfo> for InputType {
//     fn into_renderer(&'a self) -> Box<dyn Renderer<'a, DMMFTypeInfo> + 'a> {
//         Box::new(DMMFTypeRenderer::Input(self))
//     }
// }

impl<'a> IntoRenderer<'a> for EnumType {
    fn into_renderer(&'a self) -> Box<dyn Renderer<'a> + 'a> {
        Box::new(DMMFEnumRenderer::new(self))
    }
}

impl<'a> IntoRenderer<'a> for InputObjectTypeRef {
    fn into_renderer(&'a self) -> Box<dyn Renderer<'a> + 'a> {
        Box::new(DMMFObjectRenderer::Input(Weak::clone(self)))
    }
}

impl<'a> IntoRenderer<'a> for ObjectTypeRef {
    fn into_renderer(&'a self) -> Box<dyn Renderer<'a> + 'a> {
        Box::new(DMMFObjectRenderer::Output(Weak::clone(self)))
    }
}
