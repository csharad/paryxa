import React, { Component } from 'react';
import {
    Table,
    TableHead,
    TableBody,
    TableRow,
    TableCell,
    Paper,
    withStyles,
    Typography
} from '@material-ui/core';
import PropTypes from 'prop-types';

const styles = theme => ({
    container: {
        padding: `${theme.spacing.unit * 2}px ${theme.spacing.unit * 2}px 0 ${theme.spacing.unit * 3}px`,
    },
    title: {
        marginBottom: theme.spacing.unit * 2
    }
});

class TestsDashboard extends Component {
    static propTypes = {
        classes: PropTypes.object.isRequired
    };

    render() {
        const { classes } = this.props;

        return (
            <div className={classes.container}>
                <Typography
                    variant="display1"
                    className={classes.title}
                >Tests
                </Typography>

                <Paper>
                    <Table>
                        <TableHead>
                            <TableRow>
                                <TableCell>Name</TableCell>
                                <TableCell>Questions</TableCell>
                                <TableCell>Total Time</TableCell>
                                <TableCell>Scheduled On</TableCell>
                                <TableCell>Actions</TableCell>
                            </TableRow>
                        </TableHead>
                        <TableBody>

                            <TableRow>
                                <TableCell>Name</TableCell>
                                <TableCell>Questions</TableCell>
                                <TableCell>Total Time</TableCell>
                                <TableCell>Scheduled On</TableCell>
                                <TableCell>Actions</TableCell>
                            </TableRow>
                            <TableRow>
                                <TableCell>Name</TableCell>
                                <TableCell>Questions</TableCell>
                                <TableCell>Total Time</TableCell>
                                <TableCell>Scheduled On</TableCell>
                                <TableCell>Actions</TableCell>
                            </TableRow>
                            <TableRow>
                                <TableCell>Name</TableCell>
                                <TableCell>Questions</TableCell>
                                <TableCell>Total Time</TableCell>
                                <TableCell>Scheduled On</TableCell>
                                <TableCell>Actions</TableCell>
                            </TableRow>
                            <TableRow>
                                <TableCell>Name</TableCell>
                                <TableCell>Questions</TableCell>
                                <TableCell>Total Time</TableCell>
                                <TableCell>Scheduled On</TableCell>
                                <TableCell>Actions</TableCell>
                            </TableRow>
                        </TableBody>
                    </Table>
                </Paper>
            </div>
        );
    }
}

export default withStyles(styles)(TestsDashboard);
