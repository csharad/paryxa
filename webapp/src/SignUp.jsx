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
import { Link } from 'react-router-dom';

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
    };

    render() {
        const { classes } = this.props;

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
                            className={classes.button}
                            fullWidth
                            component={Link}
                            to="/getting-started"
                        >
                            Next
                            <Icon>arrow_right</Icon>
                        </Button>
                    </form>
                </Paper>
            </Grid>
        );
    }
}

export default withStyles(styles)(SignUp);
