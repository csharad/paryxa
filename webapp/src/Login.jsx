import React, { Component } from 'react';
import { TextField, withStyles, Button, Paper, Avatar, Icon, Typography, Grid } from '@material-ui/core';
import PropTypes from 'prop-types';
import { Formik } from 'formik';
import * as Yup from 'yup';
import { ApolloConsumer } from 'react-apollo';
import { withRouter } from 'react-router-dom';
import { ME } from './AuthenticatedUser';

const styles = theme => ({
    paper: {
        width: 350,
        padding: theme.spacing.unit * 4,
        margin: `${theme.spacing.unit * 4}px auto 0 auto`,
    },
    loginButton: {
        marginTop: theme.spacing.unit * 2
    },
    lockAvatar: {
        backgroundColor: theme.palette.secondary.main,
        marginBottom: theme.spacing.unit
    },
    titleSpace: {
        marginBottom: theme.spacing.unit * 3
    }
});

class Login extends Component {
    static propTypes = {
        classes: PropTypes.object.isRequired,
        history: PropTypes.object.isRequired,
    };

    render() {
        const { classes } = this.props;

        const formik = (client) => (<Formik
            initialValues={{
                email: '',
                password: '',
            }}
            validationSchema={Yup.object().shape({
                email: Yup.string()
                    .email('Must be in the form of username@example.com')
                    .required('E-mail is a required field.'),
                password: Yup.string()
                    .min(8, 'Atleast 8 characters required for a valid password.')
                    .required('Password is a required field.'),
            })}
            onSubmit={(user, action) => {
                localStorage.setItem('paryxa-token', btoa(`${user.email}:${user.password}`));

                return client.query({
                    query: ME,
                    fetchPolicy: 'network-only',
                }).then(({ loading }) => {
                    if (!loading) {
                        action.setSubmitting(false);
                        // Due to some reason this query does not replace the `Me`
                        // object in state cache. For the time being, gonna use the
                        // following workaround.
                        client.reFetchObservableQueries();
                        this.props.history.push('/profile');
                    }
                }).catch((err) => {
                    const errors = err.graphQLErrors;
                    if (errors[0].extensions.kind === 'UNAUTHORIZED') {
                        action.setErrors({
                            email: 'Either the e-mail is wrong',
                            password: 'or the password is wrong.'
                        });
                        action.setSubmitting(false);
                    }
                    localStorage.removeItem('paryxa-token');
                })
            }}
        >
            {({
                values,
                handleChange,
                handleBlur,
                handleSubmit,
                isSubmitting,
                errors,
                touched,
            }) => (<form noValidate autoComplete="off" onSubmit={handleSubmit}>
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
                    color="primary"
                    variant="contained"
                    fullWidth
                    className={classes.loginButton}
                    disabled={isSubmitting}
                    type="submit"
                >Login</Button>
            </form>)}
        </Formik>);

        return (
            <Paper className={classes.paper}>
                <Grid
                    container
                    alignItems="center"
                    direction="column"
                    className={classes.titleSpace}
                >
                    <Avatar className={classes.lockAvatar}>
                        <Icon>lock</Icon>
                    </Avatar>
                    <Typography variant="h5">Sign In</Typography>
                </Grid>
                <ApolloConsumer>{formik}</ApolloConsumer>
            </Paper>
        );
    }
}

export default withRouter(withStyles(styles)(Login));
