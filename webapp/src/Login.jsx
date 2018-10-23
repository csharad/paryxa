import React, { Component } from 'react';
import { TextField, withStyles, Button, Paper, Avatar, Icon, Typography, Grid } from '@material-ui/core';
import PropTypes from 'prop-types';

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
    };

    render() {
        const { classes } = this.props;

        return (
            <Paper className={classes.paper}>
                <Grid
                    container
                    alignItems="center"
                    direction="column"
                    className={classes.titleSpace}
                >
                    <Avatar className={classes.lockAvatar}><Icon>lock</Icon></Avatar>
                    <Typography variant="h5">Sign In</Typography>
                </Grid>

                <form noValidate autoComplete="off">
                    <TextField
                        label="E-mail"
                        fullWidth
                        margin="normal"
                    ></TextField>

                    <TextField
                        label="Password"
                        fullWidth
                        type="password"
                        margin="normal"
                    ></TextField>

                    <Button
                        color="primary"
                        variant="contained"
                        fullWidth
                        className={classes.loginButton}
                    >Login
                    </Button>
                </form>
            </Paper>
        );
    }
}

export default withStyles(styles)(Login);
