import React, { Component } from 'react';
import {
    Table,
    TableHead,
    TableBody,
    TableRow,
    TableCell,
    Paper,
    withStyles,
    Typography,
    InputBase,
    Select,
    Grid,
    MenuItem
} from '@material-ui/core';
import PropTypes from 'prop-types';
import { Link } from 'react-router-dom';

const styles = theme => ({
    container: {
        padding: `${theme.spacing.unit * 2}px ${theme.spacing.unit * 2}px 0 ${theme.spacing.unit * 3}px`,
    },
    marginBottom: {
        marginBottom: theme.spacing.unit * 2
    },
    searchInput: {
        backgroundColor: theme.palette.grey[200],
        borderRadius: theme.shape.borderRadius,
        padding: theme.spacing.unit,
        '&:hover': {
            backgroundColor: theme.palette.grey[300]
        }
    },
    newness: {
        padding: `
            ${theme.spacing.unit}px 
            ${theme.spacing.unit * 4}px 
            ${theme.spacing.unit}px 
            ${theme.spacing.unit * 2}px
        `,
        backgroundColor: theme.palette.grey[200]
    }
});

class UserDashboard extends Component {
    static propTypes = {
        classes: PropTypes.object.isRequired
    };

    render() {
        const { classes } = this.props;

        return (
            <div className={classes.container}>
                <Typography
                    variant="display1"
                    className={classes.marginBottom}
                >Users
                </Typography>

                <Grid container justify="space-between" alignItems="center" className={classes.marginBottom}>
                    <InputBase
                        placeholder="Name / E-mail / Contact"
                        classes={{ input: classes.searchInput }}
                    ></InputBase>

                    <Select classes={{ select: classes.newness }} value="all">
                        <MenuItem value="new">New Users</MenuItem>
                        <MenuItem value="all">All Users</MenuItem>
                    </Select>
                </Grid>

                <Paper>
                    <Table>
                        <TableHead>
                            <TableRow>
                                <TableCell>Name</TableCell>
                                <TableCell>Email</TableCell>
                                <TableCell>Contact</TableCell>
                                <TableCell>Subscription</TableCell>
                            </TableRow>
                        </TableHead>
                        <TableBody>
                            {Array(5).fill(
                                <TableRow component={Link} hover to="/dashboard/user/user-name">
                                    <TableCell>User Name</TableCell>
                                    <TableCell>user@email.com</TableCell>
                                    <TableCell>1234567890</TableCell>
                                    <TableCell>None</TableCell>
                                </TableRow>)}
                        </TableBody>
                    </Table>
                </Paper>
            </div>
        );
    }
}

export default withStyles(styles)(UserDashboard);
