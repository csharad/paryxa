import React, { Component, Fragment } from 'react';
import { AppBar, Toolbar, Button, withStyles, Icon } from '@material-ui/core';
import PropTypes from 'prop-types';
import { Link } from 'react-router-dom';
import AuthenticatedUser from './AuthenticatedUser';

const styles = theme => ({
    grow: {
        flexGrow: 1
    },
    profileIcon: {
        marginRight: theme.spacing.unit
    }
});

class NavigationBar extends Component {
    static propTypes = {
        classes: PropTypes.object.isRequired,
    };

    render() {
        const { classes } = this.props;

        return (
            <AuthenticatedUser>
                {({ data, unauthorized }) => {
                    const me = data && data.me;

                    return (
                        <AppBar position="sticky" elevation={0}>
                            <Toolbar>
                                <div className={classes.grow}>
                                    <Button color="inherit" component={Link} to="/">
                                        Paryksa
                                    </Button>
                                </div>
                                {
                                    !unauthorized ?
                                        <Fragment>
                                            {
                                                me && me.type === 'ADMIN' ?
                                                    <Button
                                                        color="inherit"
                                                        component={Link}
                                                        to="/dashboard/users"
                                                    >Dashboard</Button> :
                                                    ''
                                            }

                                            <Button
                                                color="inherit"
                                                component={Link}
                                                to="/profile"
                                            >
                                                <Icon className={classes.profileIcon}>account_circle</Icon>
                                                Profile
                                            </Button>
                                        </Fragment>
                                        :
                                        <Fragment>
                                            <Button color="inherit" component={Link} to="/login">
                                                Login
                                            </Button>

                                            <Button color="inherit" component={Link} to="/signup">
                                                Sign Up
                                            </Button>
                                        </Fragment>

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
