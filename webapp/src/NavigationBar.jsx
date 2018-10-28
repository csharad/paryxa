import React, { Component, Fragment } from 'react';
import { AppBar, Toolbar, Typography, Button, withStyles } from '@material-ui/core';
import PropTypes from 'prop-types';
import { Link } from 'react-router-dom';
import AuthenticatedUser from './AuthenticatedUser';

const styles = {
    grow: {
        flexGrow: 1
    }
};

class NavigationBar extends Component {
    static propTypes = {
        classes: PropTypes.object.isRequired,
    };

    render() {
        const { classes } = this.props;

        return (
            <AuthenticatedUser>
                {({ data, isLogged }) => {
                    return (
                        <AppBar position="sticky">
                            <Toolbar>
                                <Typography variant="h6" color="inherit" className={classes.grow}>
                                    Paryksa
                                </Typography>

                                {
                                    isLogged ?
                                        <Fragment>
                                            <Button color="inherit" component={Link} to="/login">
                                                Login
                                            </Button>

                                            <Button color="inherit" component={Link} to="/signup">
                                                Sign Up
                                            </Button>
                                        </Fragment>
                                        : ''

                                }
                            </Toolbar>
                        </AppBar>
                    );
                }}
            </AuthenticatedUser>
        );
    }
}

export default withStyles(styles)(NavigationBar);
