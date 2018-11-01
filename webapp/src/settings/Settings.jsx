import React, { Component } from 'react';
import {
    Paper,
    Typography,
    Grid,
    withStyles,
} from '@material-ui/core';
import PropTypes from 'prop-types';
import ProfileSetting from './ProfileSetting';
import UserAccountSetting from './UserAccountSetting';

const styles = theme => ({
    paper: {
        margin: `${theme.spacing.unit * 2}px 0`,
        padding: `${theme.spacing.unit * 3}px ${theme.spacing.unit * 4}px`
    }
});

class Settings extends Component {
    static propTypes = {
        classes: PropTypes.object.isRequired,
    };

    render() {
        const { classes } = this.props;

        return (
            <Grid container justify="center">
                <Grid item md={5} component={Paper} className={classes.paper}>
                    <Typography variant="h5">Settings</Typography>
                    <ProfileSetting></ProfileSetting>
                    <UserAccountSetting></UserAccountSetting>
                </Grid>
            </Grid>
        );
    }
}

export default withStyles(styles)(Settings);