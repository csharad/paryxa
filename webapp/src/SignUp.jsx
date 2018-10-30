import React, { Component } from 'react';
import {
    TextField,
    withStyles,
    Button,
    Typography,
    Icon,
    Paper,
    Grid
} from '@material-ui/core';
import PropTypes from 'prop-types';
import { Formik } from 'formik';
import * as yup from 'yup';
import { Mutation } from 'react-apollo';
import gql from 'graphql-tag';
import { withRouter } from 'react-router-dom';

const styles = theme => ({
    paper: {
        padding: theme.spacing.unit * 4,
        width: 350,
        marginTop: theme.spacing.unit * 4
    },
    button: {
        marginTop: theme.spacing.unit * 2
    },
    userLogo: {
        color: theme.palette.secondary.main,
        fontSize: '3rem'
    },
    titleSpace: {
        marginBottom: theme.spacing.unit * 3
    }
});

class SignUp extends Component {
    static propTypes = {
        classes: PropTypes.object.isRequired,
        history: PropTypes.object.isRequired,
    };

    render() {
        const { classes, history } = this.props;

        return (
            <Grid container justify="center">
                <Paper className={classes.paper}>
                    <Grid
                        container
                        direction="column"
                        alignItems="center"
                        className={classes.titleSpace}
                    >
                        <Icon fontSize="large" className={classes.userLogo}>account_circle</Icon>
                        <Typography color="inherit" variant="h5" align="center">
                            Sign Up
                        </Typography>
                    </Grid>

                    <Mutation
                        mutation={gql`
                            mutation NewUserSignUp($user: UserForm!) {
                                createUser(user: $user) {
                                    id
                                    firstName
                                    lastName
                                    email
                                    gender
                                    type
                                }
                            }
                        `}
                    >
                        {(createUser, { client }) => (
                            <Formik
                                initialValues={{
                                    email: '',
                                    password: '',
                                }}
                                validationSchema={yup.object().shape({
                                    email: yup.string().email().required(),
                                    password: yup.string().min(8).required(),
                                })}
                                onSubmit={async (user, action) => {
                                    try {
                                        await createUser({ variables: { user } });
                                        // Immediately login the user.
                                        localStorage.setItem(
                                            'paryxa-token',
                                            btoa(`${user.email}:${user.password}`)
                                        );
                                        // Due to some reason this query does not replace the `Me`
                                        // object in state cache. For the time being, gonna use the
                                        // following workaround.
                                        await client.reFetchObservableQueries();
                                        
                                        history.push('/getting-started');
                                        action.setSubmitting(false);
                                    } catch (e) {
                                        if (e.graphQLErrors[0].extensions.kind === 'NOT_UNIQUE') {
                                            action.setErrors({
                                                email: 'This e-mail already exists. Enter a new one.'
                                            });
                                            return;
                                        }
                                        action.setSubmitting(false);
                                        throw e;
                                    }
                                }}
                            >
                                {({
                                    values,
                                    touched,
                                    errors,
                                    handleBlur,
                                    handleChange,
                                    handleSubmit,
                                    isSubmitting,
                                }) => (
                                        <form
                                            noValidate
                                            autoComplete="off"
                                            onSubmit={handleSubmit}
                                        >
                                            <TextField
                                                name="email"
                                                label="E-mail"
                                                fullWidth
                                                margin="normal"
                                                onChange={handleChange}
                                                onBlur={handleBlur}
                                                value={values.email}
                                                helperText={touched.email && errors.email}
                                                error={touched.email && !!errors.email}
                                            ></TextField>

                                            <TextField
                                                name="password"
                                                label="Password"
                                                fullWidth
                                                type="password"
                                                margin="normal"
                                                onChange={handleChange}
                                                onBlur={handleBlur}
                                                value={values.password}
                                                helperText={touched.password && errors.password}
                                                error={touched.password && !!errors.password}
                                            ></TextField>

                                            <Button
                                                type="submit"
                                                color="primary"
                                                variant="contained"
                                                className={classes.button}
                                                fullWidth
                                                disabled={isSubmitting}
                                            >
                                                Next
                                                <Icon>arrow_right</Icon>
                                            </Button>
                                        </form>
                                    )}
                            </Formik>
                        )}
                    </Mutation>
                </Paper>
            </Grid>
        );
    }
}

export default withRouter(withStyles(styles)(SignUp));
