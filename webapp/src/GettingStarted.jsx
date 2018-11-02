import React, { Component } from "react";
import {
  Paper,
  TextField,
  withStyles,
  MenuItem,
  Button,
  Typography,
  Icon
} from "@material-ui/core";
import PropTypes from "prop-types";
import { Formik } from "formik";
import * as yup from "yup";
import { Mutation } from "react-apollo";
import gql from "graphql-tag";
import { withRouter } from "react-router-dom";

const styles = theme => ({
  formContainer: {
    width: 350,
    margin: `${theme.spacing.unit * 4}px auto 0 auto`,
    padding: `${theme.spacing.unit * 3}px ${theme.spacing.unit * 4}px`
  },
  buttonMargin: {
    marginTop: theme.spacing.unit * 2
  }
});

class GettingStarted extends Component {
  static propTypes = {
    classes: PropTypes.object.isRequired,
    history: PropTypes.object.isRequired
  };

  render() {
    const { classes, history } = this.props;

    const formik = (updateProfile, { loading }) => (
      <Formik
        initialValues={{
          firstName: "",
          lastName: "",
          gender: "",
          contact: ""
        }}
        validationSchema={yup.object().shape({
          firstName: yup.string(),
          lastName: yup.string(),
          gender: yup.string().oneOf(["MALE", "FEMALE", "OTHER"]),
          contact: yup
            .string()
            .matches(/^\d{10}$/, "This does not look like a mobile number.")
        })}
        onSubmit={async profile => {
          const { firstName, lastName, gender, contact } = profile;
          await updateProfile({
            variables: {
              user: {
                firstName,
                isFirstNameNull: firstName.length === 0,
                lastName,
                isLastNameNull: lastName.length === 0,
                gender: gender.length === 0 ? undefined : gender,
                isGenderNull: gender.length === 0,
                contact,
                isContactNull: contact.length === 0
              }
            }
          });
          history.push("/profile");
        }}
      >
        {({
          values,
          errors,
          touched,
          handleBlur,
          handleChange,
          handleSubmit
        }) => (
          <form noValidate autoComplete="off" onSubmit={handleSubmit}>
            <TextField
              name="firstName"
              label="First Name"
              margin="normal"
              fullWidth
              onChange={handleChange}
              onBlur={handleBlur}
              value={values.firstName}
              helperText={touched.firstName && errors.firstName}
              error={touched.firstName && !!errors.firstName}
            />
            <TextField
              name="lastName"
              label="Last Name"
              margin="normal"
              fullWidth
              onChange={handleChange}
              onBlur={handleBlur}
              value={values.lastName}
              helperText={touched.lastName && errors.lastName}
              error={touched.lastName && !!errors.lastName}
            />
            <TextField
              name="gender"
              label="Gender"
              select
              fullWidth
              margin="normal"
              onChange={handleChange}
              onBlur={handleBlur}
              value={values.gender}
              helperText={touched.gender && errors.gender}
              error={touched.gender && !!errors.gender}
            >
              <MenuItem value="MALE">Male</MenuItem>
              <MenuItem value="FEMALE">Female</MenuItem>
              <MenuItem value="OTHER">Other</MenuItem>
              <MenuItem value="">Unspecified</MenuItem>
            </TextField>
            <TextField
              name="contact"
              label="Mobile"
              margin="normal"
              fullWidth
              onChange={handleChange}
              onBlur={handleBlur}
              value={values.contact}
              helperText={touched.contact && errors.contact}
              error={touched.contact && !!errors.contact}
            />

            <Button
              type="submit"
              variant="contained"
              color="primary"
              className={classes.buttonMargin}
              fullWidth
              disabled={loading}
            >
              Next
              <Icon>arrow_right</Icon>
            </Button>
          </form>
        )}
      </Formik>
    );

    return (
      <Paper className={classes.formContainer}>
        <Typography variant="body1">Fill in your details.</Typography>

        <Mutation
          mutation={gql`
            mutation UpdateMyInfo($user: UserInfoUpdate!) {
              updateMe(user: $user) {
                id
                firstName
                lastName
                fullName
                gender
                contact
              }
            }
          `}
        >
          {formik}
        </Mutation>
      </Paper>
    );
  }
}

export default withRouter(withStyles(styles)(GettingStarted));
