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

class ReportCard extends Component {
    static propTypes = {
        title: PropTypes.string,
        time: PropTypes.string,
        onClose: PropTypes.func.isRequired,
        open: PropTypes.bool,
        classes: PropTypes.object.isRequired
    }

    render() {
        const { title, time, onClose, open, classes } = this.props;

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
                        <Typography variant="body1">0/100</Typography>
                        <Icon fontSize="large">score</Icon>
                        <Typography variant="caption">Score</Typography>
                    </Grid>

                    <Grid item className={classes.alignCenter}>
                        <Typography variant="body1">0/12312</Typography>
                        <Icon fontSize="large">event_seat</Icon>
                        <Typography variant="caption">Rank</Typography>
                    </Grid>

                    <Grid item className={classes.alignCenter}>
                        <Typography variant="body1">57 minutes</Typography>
                        <Icon fontSize="large">av_timer</Icon>
                        <Typography variant="caption">Duration</Typography>
                    </Grid>
                </Grid>

                <DialogActions>
                    <Button component={Link} to="/solved-questions">View Your Answers</Button>
                </DialogActions>
            </Dialog>
        );
    }
}

export default withStyles(styles)(ReportCard);
