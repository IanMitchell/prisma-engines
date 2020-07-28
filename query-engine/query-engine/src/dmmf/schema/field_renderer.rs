use super::*;

/// Render the input field, and informs the RenderContext of its dependencies in
/// the process.
pub fn render_dmmf_input_field<'a>(field: &'a InputFieldRef, ctx: &mut RenderContext<'a>) -> DMMFInputField {
    let type_info = DMMFTypeRenderer::Input(&field.field_type).render_input_type(&field.field_type, ctx);
    let field = DMMFInputField {
        name: field.name.clone(),
        input_type: type_info,
    };

    field
}

/// Render the output field, and informs the RenderContext of its dependencies in
/// the process.
pub fn render_dmmf_output_field<'a>(field: &'a FieldRef, ctx: &mut RenderContext<'a>) -> DMMFField {
    let args = render_arguments(&field.arguments, ctx);
    let output_type = DMMFTypeRenderer::Output(&field.field_type).render_output_type(&field.field_type, ctx);
    let output_field = DMMFField {
        name: field.name.clone(),
        args,
        output_type,
    };

    ctx.add_mapping(field.name.clone(), field.query_builder.as_ref());

    output_field
}

// #[derive(Debug)]
// pub enum DMMFFieldRenderer {
//     Input(InputFieldRef),
//     Output(FieldRef),
// }

// impl<'a> Renderer<'a, DMMFFieldWrapper> for DMMFFieldRenderer {
//     fn render(&self, ctx: &mut RenderContext) -> DMMFFieldWrapper {
//         match self {
//             DMMFFieldRenderer::Input(input) => self.render_input_field(Arc::clone(input), ctx),
//             DMMFFieldRenderer::Output(output) => self.render_output_field(Arc::clone(output), ctx),
//         }
//     }
// }

//     fn render_output_field(&self, field: FieldRef, ctx: &mut RenderContext) -> DMMFFieldWrapper {
//         let args = self.render_arguments(&field.arguments, ctx);
//         let output_type = field.field_type.into_renderer().render(ctx);
//         let output_field = DMMFField {
//             name: field.name.clone(),
//             args,
//             output_type,
//         };

//         ctx.add_mapping(field.name.clone(), field.query_builder.as_ref());
//         DMMFFieldWrapper::Output(output_field)
//     }

fn render_arguments<'a>(args: &'a [Argument], ctx: &mut RenderContext<'a>) -> Vec<DMMFArgument> {
    args.iter().map(|arg| render_argument(arg, ctx)).collect()
}

fn render_argument<'a>(arg: &'a Argument, ctx: &mut RenderContext<'a>) -> DMMFArgument {
    let input_type = DMMFTypeRenderer::Input(&arg.argument_type).render_input_type(&arg.argument_type, ctx);
    let rendered_arg = DMMFArgument {
        name: arg.name.clone(),
        input_type,
    };

    rendered_arg
}
// }
