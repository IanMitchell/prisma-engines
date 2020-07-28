use super::*;

pub struct DMMFSchemaRenderer<'a> {
    query_schema: &'a QuerySchemaRef,
}

impl<'a> Renderer<'a> for DMMFSchemaRenderer<'a> {
    fn render(&self, ctx: &mut RenderContext<'a>) {
        DMMFTypeRenderer::Output(self.query_schema.query.as_ref())
            .render_output_type(self.query_schema.query.as_ref(), ctx);
        DMMFTypeRenderer::Output(self.query_schema.mutation.as_ref())
            .render_output_type(self.query_schema.mutation.as_ref(), ctx);
    }
}

impl<'a> DMMFSchemaRenderer<'a> {
    pub fn new(query_schema: &'a QuerySchemaRef) -> DMMFSchemaRenderer<'a> {
        DMMFSchemaRenderer { query_schema }
    }
}
