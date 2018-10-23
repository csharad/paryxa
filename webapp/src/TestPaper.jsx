import React, { Component } from 'react';
import {
    LinearProgress,
    Grid,
    Typography,
    withStyles,
    FormControlLabel,
    Radio,
    RadioGroup,
    Button
} from '@material-ui/core';
import PropTypes from 'prop-types';
import { Link } from 'react-router-dom';

const styles = theme => ({
    margin: {
        marginTop: theme.spacing.unit * 4
    },
    progressBarContainer: {
        width: '80%'
    },
    progressBar: {
        height: 10,
        borderRadius: theme.shape.borderRadius
    },
    leftTime: {
        marginLeft: theme.spacing.unit
    },
    buttonMargin: {
        marginLeft: theme.spacing.unit * 2
    }
});

class TestPaper extends Component {
    static propTypes = {
        classes: PropTypes.object.isRequired
    };

    render() {
        const { classes } = this.props;

        return (
            <Grid container justify="center" className={classes.margin}>
                <Grid item container direction="column" md={5}>
                    <Grid item container alignItems="center" >
                        <div className={classes.progressBarContainer}>
                            <Typography align="center">10 / 50 completed</Typography>

                            <LinearProgress
                                value={20}
                                variant="determinate"
                                className={classes.progressBar}
                            ></LinearProgress>
                        </div>

                        <div>
                            <Typography className={classes.leftTime}>00:30:23</Typography>
                            <Typography className={classes.leftTime}>remaining</Typography>
                        </div>
                    </Grid>

                    <Typography
                        variant="display1"
                        className={classes.margin}
                    >What is the question?
                    </Typography>

                    <RadioGroup name="opts">
                        <FormControlLabel
                            value="opt1"
                            control={<Radio />}
                            label="Option 1"
                        ></FormControlLabel>
                        <FormControlLabel
                            value="opt2"
                            control={<Radio />}
                            label="Option 2"
                        ></FormControlLabel>
                        <FormControlLabel
                            value="opt3"
                            control={<Radio />}
                            label="Option 3"
                        ></FormControlLabel>
                        <FormControlLabel
                            value="opt4"
                            control={<Radio />}
                            label="Option 4"
                        ></FormControlLabel>
                    </RadioGroup>

                    <Grid container justify="space-between" className={classes.margin}>
                        <Button>Back</Button>
                        <div>
                            <Button >Skip</Button>
                            <Button
                                variant="contained"
                                color="primary"
                                className={classes.buttonMargin}
                                component={Link}
                                to="/test-paper/completed"
                            >Next
                            </Button>
                        </div>
                    </Grid>
                </Grid>
            </Grid>
        );
    }
}

export default withStyles(styles)(TestPaper);
