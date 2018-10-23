import React, { Component } from 'react';
import {
    Paper,
    Typography,
    Grid,
    withStyles,
    TextField,
    Button,
    MenuItem
} from '@material-ui/core';
import PropTypes from 'prop-types';

const styles = theme => ({
    paper: {
        margin: `${theme.spacing.unit * 2}px 0`,
        padding: `${theme.spacing.unit * 3}px ${theme.spacing.unit * 4}px`
    },
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

class Settings extends Component {
    static propTypes = {
        classes: PropTypes.object.isRequired,
    };

    render() {
        const { classes } = this.props;

        return (
            <Grid container justify="center">
                <Grid item md="6" component={Paper} className={classes.paper}>
                    <Typography variant="display1">Settings</Typography>

                    <section className={classes.section}>
                        <Typography
                            variant="h6"
                            className={classes.sectionTitle}
                        >Personal
                        </Typography>
                        <form>

                            <Grid container spacing={8}>
                                <Grid item md={4}>
                                    <TextField
                                        label="First Name"
                                        margin="normal"
                                        fullWidth
                                    ></TextField>
                                </Grid>

                                <Grid item md={4}>
                                    <TextField
                                        label="Last Name"
                                        margin="normal"
                                        fullWidth
                                    ></TextField>
                                </Grid>

                                <Grid item md={4}>
                                    <TextField
                                        label="Gender"
                                        margin="normal"
                                        select
                                        fullWidth
                                    >
                                        <MenuItem value="male">Male</MenuItem>
                                        <MenuItem value="female">Female</MenuItem>
                                        <MenuItem value="other">Other</MenuItem>
                                    </TextField>
                                </Grid>
                            </Grid>
                        </form>
                    </section>

                    <section className={classes.section}>
                        <Typography
                            variant="h6"
                            className={classes.sectionTitle}
                        >Account
                        </Typography>
                        <form>
                            <Grid container spacing={8}>
                                <Grid item md={4}>
                                    <TextField
                                        label="E-mail"
                                        margin="normal"
                                        fullWidth
                                    ></TextField>
                                </Grid>
                                <Grid item md={4}>
                                    <TextField
                                        label="Contact"
                                        margin="normal"
                                        fullWidth
                                    ></TextField>
                                </Grid>
                            </Grid>
                        </form>
                    </section>

                    <section className={classes.section}>
                        <Typography
                            variant="h6"
                            className={classes.sectionTitle}
                        >Security
                        </Typography>
                        <form>
                            <Grid container spacing={8}>
                                <Grid item md={4}>
                                    <TextField
                                        label="Old Password"
                                        margin="normal"
                                        fullWidth
                                        type="password"
                                    ></TextField>
                                </Grid>
                                <Grid item md={4}>
                                    <TextField
                                        label="New Password"
                                        margin="normal"
                                        fullWidth
                                        type="password"
                                    ></TextField>
                                </Grid>
                            </Grid>
                        </form>
                    </section>


                    <Button
                        variant="contained"
                        color="secondary"
                        className={classes.button}
                    >Update
                    </Button>
                </Grid>
            </Grid>
        );
    }
}

export default withStyles(styles)(Settings);