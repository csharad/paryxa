import React, { Component } from 'react';
import {
    Typography,
    Grid,
    withStyles,
    TextField,
    Button,
    MenuItem
} from '@material-ui/core';
import AuthenticatedUser from '../AuthenticatedUser';
import { Formik } from 'formik';
import PropTypes from 'prop-types';
import * as yup from 'yup';
import UpdateMyInfo from '../UpdateMyInfo';

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

class ProfileSetting extends Component {
    static propTypes = {
        classes: PropTypes.object.isRequired,
    };

    render() {
        const { classes } = this.props;

        const formik = (data) => (updateInfo, { loading }) => (<Formik
            initialValues={{
                firstName: data.me.firstName || '',
                lastName: data.me.lastName || '',
                gender: data.me.gender || '',
                contact: data.me.contact || '',
            }}
            validationSchema={yup.object().shape({
                firstName: yup.string(),
                lastName: yup.string(),
                gender: yup.string().oneOf(['MALE', 'FEMALE', 'OTHER']),
                contact: yup.string()
                    .matches(
                        /^\d{10}$/,
                        'This does not look like a mobile number.'
                    ),
            })}
            onSubmit={async (profile) => {
                const { firstName, lastName, gender, contact } = profile;
                await updateInfo({
                    variables: {
                        user: {
                            firstName,
                            isFirstNameNull: firstName.length === 0,
                            lastName,
                            isLastNameNull: lastName.length === 0,
                            gender: gender.length !== 0 ? gender : undefined,
                            isGenderNull: gender.length === 0,
                            contact,
                            isContactNull: contact.length === 0,
                        }
                    }
                });
            }}
        >{({
            values,
            errors,
            touched,
            handleChange,
            handleBlur,
            handleSubmit
        }) => (<form
            onSubmit={handleSubmit}
            className={classes.section}
        >
            <Typography
                variant="h6"
                className={classes.sectionTitle}
            >Personal</Typography>
            <Grid container spacing={16}>
                <Grid item md={6}>
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
                    ></TextField>
                </Grid>

                <Grid item md={6}>
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
                    ></TextField>
                </Grid>

                <Grid item md={6}>
                    <TextField
                        name="gender"
                        label="Gender"
                        margin="normal"
                        select
                        fullWidth
                        onChange={handleChange}
                        onBlur={handleBlur}
                        value={values.gender}
                        helperText={touched.gender && errors.gender}
                        error={touched.gender && !!errors.gender}
                    >
                        <MenuItem value="">Unspecified</MenuItem>
                        <MenuItem value="MALE">Male</MenuItem>
                        <MenuItem value="FEMALE">Female</MenuItem>
                        <MenuItem value="OTHER">Other</MenuItem>
                    </TextField>
                </Grid>

                <Grid item md={6}>
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
                    ></TextField>
                </Grid>
            </Grid>

            <Button
                type="submit"
                variant="contained"
                color="secondary"
                className={classes.button}
                disabled={loading}
            >Update</Button>
        </form>)}</Formik>);

        return (
            <AuthenticatedUser>
                {({ data, loading }) => !loading ? (
                    <UpdateMyInfo>{formik(data)}</UpdateMyInfo>
                ) : null}
            </AuthenticatedUser>
        );
    }
}

export default withStyles(styles)(ProfileSetting);
