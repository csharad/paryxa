import React, { Component } from 'react';
import { TextField, withStyles, MenuItem, Button } from '@material-ui/core';

const styles = theme => ({
    formContainer: {
        width: 250,
        margin: '40px auto 0 auto'
    },
    buttonMargin: {
        marginTop: theme.spacing.unit * 2
    }
});

class GettingStarted extends Component {
    render() {
        const { classes } = this.props;

        return (
            <div className={classes.formContainer}>
                Fill in your details.

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
                    >Save</Button>
                </form>
            </div>
        );
    }
}

export default withStyles(styles)(GettingStarted);
