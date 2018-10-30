import React, { Component, Fragment } from 'react';
import {
    Avatar,
    colors,
    withStyles,
    Grid,
    Typography,
    Icon,
    IconButton
} from '@material-ui/core';
import PropTypes from 'prop-types';
import TestList from './TestList';
import ProfileSettingsMenu from './ProfileSettingsMenu';
import AuthenticatedUser from './AuthenticatedUser';

const styles = theme => ({
    userAvatar: {
        backgroundColor: colors.red[500],
        width: 96,
        height: 96,
    },
    containerSpacing: {
        marginTop: theme.spacing.unit * 2,
    },
    testBlock: {
        width: '100%',
        marginTop: theme.spacing.unit * 4
    },
    listRoot: {
        backgroundColor: theme.palette.background.paper,
        width: '100%',
        marginTop: theme.spacing.unit * 2
    },
    testName: {
        display: 'block'
    }
});

class Profile extends Component {
    static propTypes = {
        classes: PropTypes.object.isRequired,
    };

    state = {
        settingsMenuShown: false,
    };

    render() {
        const { classes } = this.props;
        const { settingsMenuShown } = this.state;

        return (
            <Fragment>
                <Grid
                    container
                    direction="column"
                    alignItems="center"
                    className={classes.containerSpacing}
                >
                    <Grid
                        item
                        container
                        md={6}
                    >
                        <Grid
                            item
                            container
                            justify="space-between"
                            alignItems="center"
                        >
                            <Grid
                                item
                                container
                                md={8}
                                alignItems="center"
                                spacing={40}
                            >
                                <AuthenticatedUser>
                                    {({ data, loading }) => (
                                        <Fragment>
                                            <Grid item>
                                                <Avatar className={classes.userAvatar}>
                                                    {loading ?
                                                        'U' :
                                                        data.me.fullName ?
                                                            data.me.fullName[0].toUpperCase() :
                                                            'U'}
                                                </Avatar>
                                            </Grid>
                                            <Grid item>
                                                <Typography variant="h5">
                                                    {loading ? '' : data.me.fullName || 'Anonymous User'}
                                                </Typography>
                                            </Grid>
                                        </Fragment>
                                    )}
                                </AuthenticatedUser>
                            </Grid>
                            <Grid item >
                                <IconButton onClick={this.showSettingsMenu}>
                                    <Icon>settings</Icon>
                                </IconButton>
                            </Grid>
                        </Grid>

                        <div className={classes.testBlock}>
                            <Typography variant="h6">Current Tests</Typography>

                            <TestList newTest className={classes.listRoot} list={[
                                {
                                    title: 'Name of the Test',
                                    time: 'Time of the Test',
                                    liveTest: true,
                                }
                            ]}></TestList>
                        </div>

                        <div className={classes.testBlock}>
                            <Typography variant="h6">Your Upcoming Tests</Typography>

                            <TestList newTest className={classes.listRoot} list={[
                                {
                                    title: 'Name of the Test',
                                    time: 'Time of the Test'
                                },
                                {
                                    title: 'Name of the Test',
                                    time: 'Time of the Test'
                                },
                                {
                                    title: 'Name of the Test',
                                    time: 'Time of the Test'
                                },
                                {
                                    title: 'Name of the Test',
                                    time: 'Time of the Test'
                                },
                            ]}></TestList>
                        </div>


                        <div className={classes.testBlock}>
                            <Typography variant="h6">Completed Tests</Typography>

                            <TestList className={classes.listRoot} list={[
                                {
                                    title: 'Name of the Test',
                                    time: 'Time of the Test'
                                },
                                {
                                    title: 'Name of the Test',
                                    time: 'Time of the Test'
                                },
                                {
                                    title: 'Name of the Test',
                                    time: 'Time of the Test'
                                },
                                {
                                    title: 'Name of the Test',
                                    time: 'Time of the Test'
                                },
                            ]}></TestList>
                        </div>


                        <div className={classes.testBlock}>
                            <Typography variant="h6">All Upcoming Tests</Typography>

                            <TestList className={classes.listRoot} list={[
                                {
                                    title: 'Name of the Test',
                                    time: 'Time of the Test'
                                },
                                {
                                    title: 'Name of the Test',
                                    time: 'Time of the Test'
                                },
                                {
                                    title: 'Name of the Test',
                                    time: 'Time of the Test'
                                },
                                {
                                    title: 'Name of the Test',
                                    time: 'Time of the Test'
                                },
                            ]}></TestList>
                        </div>
                    </Grid>
                </Grid>

                <ProfileSettingsMenu
                    open={settingsMenuShown}
                    onClose={this.hideSettingsMenu}
                ></ProfileSettingsMenu>
            </Fragment>
        );
    }

    showSettingsMenu = () => {
        this.setState({
            settingsMenuShown: true
        });
    };

    hideSettingsMenu = () => {
        this.setState({
            settingsMenuShown: false
        });
    };
}

export default withStyles(styles)(Profile);
