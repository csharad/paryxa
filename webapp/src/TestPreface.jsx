import React, { Component } from 'react';
import { Dialog, Grid, DialogTitle, DialogActions, Typography, withStyles, Icon, Button } from '@material-ui/core';
import PropTypes from 'prop-types';
import { Link } from 'react-router-dom';

const styles = theme => ({
    padding: {
        paddingLeft: theme.spacing.unit * 3,
        paddingRight: theme.spacing.unit * 3,
    },
    reportValues: {
        paddingTop: theme.spacing.unit * 3,
        paddingBottom: theme.spacing.unit * 3
    },
    alignCenter: {
        textAlign: 'center'
    }
});

class TestPreface extends Component {
    static propTypes = {
        title: PropTypes.string,
        time: PropTypes.string,
        liveTest: PropTypes.bool,
        onClose: PropTypes.func.isRequired,
        open: PropTypes.bool,
        classes: PropTypes.object.isRequired
    }

    render() {
        const { title, time, onClose, open, classes, liveTest } = this.props;

        return (
            <Dialog onClose={onClose} open={open} fullWidth>
                <DialogTitle disableTypography>
                    <Typography variant="h5">{title}</Typography>
                    <Typography>{time}</Typography>
                </DialogTitle>

                <Typography className={classes.padding}>
                    Lorem ipsum dolor sit amet consectetur, adipisicing elit.
                    Itaque ratione totam vel architecto eligendi, eos, modi culpa
                    velit omnis quibusdam nihil. Numquam at asperiores magni nemo
                    ad repudiandae temporibus quibusdam.
                </Typography>

                <Grid container justify="space-evenly" className={classes.reportValues}>
                    <Grid item className={classes.alignCenter}>
                        <Typography variant="body1">100</Typography>
                        <Icon fontSize="large">score</Icon>
                        <Typography variant="caption">Full Score</Typography>
                    </Grid>

                    <Grid item className={classes.alignCenter}>
                        <Typography variant="body1">60 minutes</Typography>
                        <Icon fontSize="large">av_timer</Icon>
                        <Typography variant="caption">Total Duration</Typography>
                    </Grid>
                </Grid>

                {
                    liveTest ?
                        <DialogActions>
                            <Button component={Link} to="/test-paper/notice">Start Test</Button>
                        </DialogActions> :
                        ''
                }
            </Dialog>
        );
    }
}

export default withStyles(styles)(TestPreface);
