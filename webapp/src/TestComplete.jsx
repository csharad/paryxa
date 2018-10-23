import React, { Component } from 'react';
import { Typography, Grid, withStyles, Grow, Icon, Avatar, Button } from '@material-ui/core';
import PropTypes from 'prop-types';
import { Link } from 'react-router-dom';

const styles = theme => ({
    margin: {
        marginTop: theme.spacing.unit * 2
    },
    tick: {
        width: 64,
        height: 64,
        backgroundColor: theme.palette.secondary.main
    }
});

class TestComplete extends Component {
    static propTypes = {
        classes: PropTypes.object.isRequired
    };

    render() {
        const { classes } = this.props;

        return (
            <Grid
                container
                direction="column"
                alignItems="center"
                className={classes.margin}
            >
                <Typography variant="display1">
                    You have completed your test.
                    </Typography>
                <Typography>
                    You will get the results in a short while.
                    </Typography>

                <Grow in timeout={500}>
                    <Avatar className={`${classes.margin} ${classes.tick}`}>
                        <Icon fontSize="large">check</Icon>
                    </Avatar>
                </Grow>

                <Button
                    className={classes.margin}
                    component={Link}
                    to="/profile"
                >Go To Profile
                </Button>
            </Grid>
        );
    }
}

export default withStyles(styles)(TestComplete);
