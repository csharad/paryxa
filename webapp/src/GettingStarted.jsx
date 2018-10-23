import React, { Component } from 'react';
import {
    Paper,
    TextField,
    withStyles,
    MenuItem,
    Button,
    Typography,
    Icon
} from '@material-ui/core';
import { Link } from 'react-router-dom';
import PropTypes from 'prop-types';

const styles = theme => ({
    formContainer: {
        width: 350,
        margin: `${theme.spacing.unit * 4}px auto 0 auto`,
        padding: `${theme.spacing.unit * 3}px ${theme.spacing.unit * 4}px`
    },
    buttonMargin: {
        marginTop: theme.spacing.unit * 2
    }
});

class GettingStarted extends Component {
    static propTypes = {
        classes: PropTypes.object.isRequired
    };

    render() {
        const { classes } = this.props;

        return (
            <Paper className={classes.formContainer}>
                <Typography variant="body1">Fill in your details.</Typography>

                <form noValidate autoComplete="off" >
                    <TextField
                        label="First Name"
                        margin="normal"
                        fullWidth
                    ></TextField>
                    <TextField
                        label="Last Name"
                        margin="normal"
                        fullWidth
                    ></TextField>
                    <TextField
                        label="Gender"
                        select
                        fullWidth
                        margin="normal"
                    >
                        <MenuItem value="male">Male</MenuItem>
                        <MenuItem value="female">Female</MenuItem>
                        <MenuItem value="other">Other</MenuItem>
                    </TextField>
                    <TextField
                        label="Mobile"
                        margin="normal"
                        fullWidth
                    ></TextField>

                    <Button
                        variant="contained"
                        color="primary"
                        className={classes.buttonMargin}
                        fullWidth
                        component={Link}
                        to="/profile"
                    >
                        Next
                        <Icon>arrow_right</Icon>
                    </Button>
                </form>
            </Paper>
        );
    }
}

export default withStyles(styles)(GettingStarted);
