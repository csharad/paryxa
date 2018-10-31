import React, { Component } from 'react';
import AuthenticatedUser from './AuthenticatedUser';
import { Formik } from 'formik';
import {
    Typography,
    Grid,
    withStyles,
    TextField,
    Button,
} from '@material-ui/core';
import PropTypes from 'prop-types';

const styles = theme => ({
    section: {
        marginTop: theme.spacing.unit * 4
    },
    sectionTitle: {
        margin: 0
    },
    button: {
        marginTop: theme.spacing.unit * 2
    }
});

class UserAccountSetting extends Component {
    static propTypes = {
        classes: PropTypes.object.isRequired,
    };

    render() {
        const { classes } = this.props;

        return (
            <AuthenticatedUser>
                {({ data, loading }) => !loading ? (
                    <Formik
                        initialValues={{
                            email: data.me.email,
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
                                    >Account
                                    </Typography>
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
                                    >Update
                                    </Button>
                                </form>
                            )}
                    </Formik>
                ) : ''}
            </AuthenticatedUser>
        );
    }
}

export default withStyles(styles)(UserAccountSetting);
