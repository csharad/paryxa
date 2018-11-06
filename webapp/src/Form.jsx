import React from "react";
import { TextField, Radio } from "@material-ui/core";
import { Field } from "formik";

export function FTextField(props) {
  return <Field component={FTextFieldInner} {...props} />;
}

function FTextFieldInner({ field, form: { touched, errors }, ...props }) {
  return (
    <TextField
      {...field}
      {...props}
      helperText={touched[field.name] && errors[field.name]}
      error={touched[field.name] && !!errors[field.name]}
    />
  );
}

export function FRadioLabelless(props) {
  return <Field component={FRadioLabellessInner} {...props} />;
}

function FRadioLabellessInner({
  field: { onChange, value, name },
  form,
  ...props
}) {
  return (
    <Radio
      name={name}
      onChange={onChange}
      checked={value === props.value}
      {...props}
    />
  );
}
