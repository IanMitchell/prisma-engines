use super::*;

pub struct DMMFSchemaRenderer<'a> {
    query_schema: &'a QuerySchemaRef,
}

impl<'a> Renderer<'a> for DMMFSchemaRenderer<'a> {
    fn render(&self, ctx: &mut RenderContext<'a>) {
        render_output_type(&self.query_schema.query, ctx);
        render_output_type(&self.query_schema.mutation, ctx);
    }
}

impl<'a> DMMFSchemaRenderer<'a> {
    pub fn new(query_schema: &'a QuerySchemaRef) -> DMMFSchemaRenderer<'a> {
        DMMFSchemaRenderer { query_schema }
    }
}
