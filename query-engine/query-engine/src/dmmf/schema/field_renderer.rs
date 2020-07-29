use super::*;

pub(crate) fn render_input_field<'a>(input_field: &InputFieldRef, ctx: &mut RenderContext<'a>) -> DMMFInputField {
    let type_info = render_input_type(&input_field.field_type, ctx);

    DMMFInputField {
        name: input_field.name.clone(),
        input_type: type_info,
    }
}

pub(crate) fn render_output_field<'a>(field: &'a FieldRef, ctx: &mut RenderContext<'a>) -> DMMFField {
    let args = render_arguments(&field.arguments, ctx);
    let output_type = render_output_type(&field.field_type, ctx);
    let output_field = DMMFField {
        name: field.name.clone(),
        args,
        output_type,
    };

    ctx.add_mapping(field.name.clone(), field.query_builder.as_ref());

    output_field
}

fn render_arguments<'a>(args: &'a [Argument], ctx: &mut RenderContext<'a>) -> Vec<DMMFArgument> {
    args.iter().map(|arg| render_argument(arg, ctx)).collect()
}

fn render_argument<'a>(arg: &'a Argument, ctx: &mut RenderContext<'a>) -> DMMFArgument {
    let input_type = render_input_type(&arg.argument_type, ctx);

    DMMFArgument {
        name: arg.name.clone(),
        input_type,
    }
}
