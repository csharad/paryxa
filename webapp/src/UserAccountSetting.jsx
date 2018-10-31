import React, { Component, Fragment } from 'react';
import AuthenticatedUser from './AuthenticatedUser';
import { Formik } from 'formik';
import {
    Typography,
    Grid,
    withStyles,
    TextField,
    Button,
    Dialog,
    DialogActions,
    DialogTitle,
    DialogContentText
} from '@material-ui/core';
import PropTypes from 'prop-types';
import UpdateMyCredentials from './UpdateMyCredentials';
import * as yup from 'yup';

const styles = theme => ({
    section: {
        marginTop: theme.spacing.unit * 4
    },
    sectionTitle: {
        margin: 0
    },
    button: {
        marginTop: theme.spacing.unit * 2
    },
    confirmationDialog: {
        width: 400
    },
    confirmationBody: {
        padding: `0 ${theme.spacing.unit * 3}px`
    },
    confirmationPassword: {
        paddingTop: theme.spacing.unit
    }
});

class UserAccountSetting extends Component {
    static propTypes = {
        classes: PropTypes.object.isRequired,
    };

    state = {
        isOpen: false,
        email: '',
        newPassword: '',
        action: null,
    };

    render() {
        const { classes } = this.props;
        const { isOpen } = this.state;

        return (
            <Fragment>
                <AuthenticatedUser>
                    {({ data, loading }) => !loading ? (
                        <Formik
                            initialValues={{
                                email: data.me.email,
                                newPassword: '',
                            }}
                            validationSchema={yup.object().shape({
                                email: yup.string().email().required(),
                                newPassword: yup.string().min(8),
                            })}
                            onSubmit={async (creds, action) => {
                                this.setState({
                                    isOpen: true,
                                    email: creds.email,
                                    newPassword: creds.newPassword,
                                    action,
                                });
                            }}
                        >
                            {({
                                values,
                                errors,
                                touched,
                                handleBlur,
                                handleChange,
                                handleSubmit,
                            }) => (
                                    <form
                                        onSubmit={handleSubmit}
                                        className={classes.section}
                                    >
                                        <Typography
                                            variant="h6"
                                            className={classes.sectionTitle}
                                        >Account</Typography>
                                        <Grid container spacing={16}>
                                            <Grid item md={6}>
                                                <TextField
                                                    name="email"
                                                    label="E-mail"
                                                    margin="normal"
                                                    fullWidth
                                                    onChange={handleChange}
                                                    onBlur={handleBlur}
                                                    value={values.email}
                                                    helperText={touched.email && errors.email}
                                                    error={touched.email && !!errors.email}
                                                ></TextField>
                                            </Grid>
                                            <Grid item md={6}>
                                                <TextField
                                                    name="newPassword"
                                                    label="New Password"
                                                    margin="normal"
                                                    fullWidth
                                                    type="password"
                                                    onChange={handleChange}
                                                    onBlur={handleBlur}
                                                    helperText={touched.newPassword && errors.newPassword}
                                                    error={touched.newPassword && !!errors.newPassword}
                                                ></TextField>
                                            </Grid>
                                        </Grid>

                                        <Button
                                            type="submit"
                                            variant="contained"
                                            color="secondary"
                                            className={classes.button}
                                        >Update</Button>
                                    </form>
                                )}
                        </Formik>
                    ) : ''}
                </AuthenticatedUser>


                <UpdateMyCredentials>
                    {(updateCreds, { loading }) => (
                        <Dialog
                            open={isOpen}
                            classes={{
                                paper: classes.confirmationDialog
                            }}
                        >
                            <DialogTitle>Confirm?</DialogTitle>

                            <Formik
                                initialValues={{
                                    password: '',
                                }}
                                validationSchema={yup.object().shape({
                                    password: yup.string().min(8).required(),
                                })}
                                onSubmit={async (cred, action) => {
                                    const { email, newPassword } = this.state;
                                    try {
                                        await updateCreds({
                                            variables: {
                                                user: {
                                                    email,
                                                    newPassword: newPassword.length !== 0 ?
                                                        newPassword :
                                                        undefined,
                                                    password: cred.password
                                                }
                                            }
                                        })
                                        // Update basic token after password change.
                                        if (newPassword.length !== 0) {
                                            localStorage.setItem(
                                                'paryxa-token',
                                                btoa(`${email}:${newPassword}`)
                                            );
                                        }
                                        this.setState({
                                            isOpen: false,
                                            email: '',
                                            newPassword: '',
                                            action: null,
                                        });
                                    } catch (e) {
                                        if (e.graphQLErrors[0].extensions.kind === 'NOT_UNIQUE') {
                                            this.state.action.setErrors({
                                                email: 'The e-mail is already taken.'
                                            });
                                            this.setState({
                                                isOpen: false,
                                                email: '',
                                                newPassword: '',
                                                action: null,
                                            });
                                            return;
                                        } else if (e.graphQLErrors[0].extensions.kind === 'INCORRECT_PASSWORD') {
                                            action.setErrors({
                                                password: 'The password entered is incorrect.'
                                            });
                                            return;
                                        }
                                        throw e;
                                    }
                                }}
                            >
                                {({
                                    values,
                                    errors,
                                    touched,
                                    handleBlur,
                                    handleChange,
                                    handleSubmit,
                                }) => (
                                        <form onSubmit={handleSubmit}>
                                            <div className={classes.confirmationBody}>
                                                <DialogContentText variant="body2">
                                                    Your current password is required to change your
                                                    credentials.
                                            </DialogContentText>
                                                <div className={classes.confirmationPassword}>
                                                    <TextField
                                                        name="password"
                                                        label="Current Password"
                                                        fullWidth
                                                        type="password"
                                                        onChange={handleChange}
                                                        onBlur={handleBlur}
                                                        value={values.password}
                                                        helperText={touched.password && errors.password}
                                                        error={touched.password && !!errors.password}
                                                    ></TextField>
                                                </div>
                                            </div>

                                            <DialogActions>
                                                <Button
                                                    disabled={loading}
                                                    onClick={() => this.setState({
                                                        isOpen: false,
                                                    })}
                                                >Cancel</Button>
                                                <Button
                                                    type="submit"
                                                    disabled={loading}
                                                >Ok</Button>
                                            </DialogActions>
                                        </form>
                                    )}
                            </Formik>
                        </Dialog>
                    )}
                </UpdateMyCredentials>
            </Fragment>
        );
    }
}

export default withStyles(styles)(UserAccountSetting);
