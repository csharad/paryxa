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
import { Query } from 'react-apollo';
import gql from 'graphql-tag';
import { debounce } from 'debounce';

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

    state = {
        query: undefined,
    };

    setQuery = debounce(val => this.setState({ query: val }), 200);

    render() {
        const { classes } = this.props;
        let { query } = this.state;
        query = query && query.length === 0 ? undefined : query;

        return (
            <div className={classes.container}>
                <Typography
                    variant="h5"
                    className={classes.marginBottom}
                >Users
                </Typography>

                <InputBase
                    placeholder="Name / E-mail / Contact"
                    classes={{ input: classes.searchInput }}
                    onInput={ev => this.setQuery(ev.target.value)}
                    className={classes.marginBottom}
                ></InputBase>

                <Paper>
                    <Table>
                        <TableHead>
                            <TableRow>
                                <TableCell>Name</TableCell>
                                <TableCell>Gender</TableCell>
                                <TableCell>Email</TableCell>
                                <TableCell>Contact</TableCell>
                                <TableCell>Subscription</TableCell>
                            </TableRow>
                        </TableHead>
                        <Query
                            query={gql`
                            query UserList($query: String) {
                                users(query: $query) {
                                    id
                                    fullName
                                    email
                                    gender
                                    contact
                                    type
                                }
                            }
                        `}
                            variables={{
                                query,
                            }}
                        >{({ data, loading }) => !loading ? (
                            <TableBody>
                                {data.users.map(user => (
                                    <TableRow key={user.id}>
                                        <TableCell>{user.fullName}</TableCell>
                                        <TableCell>{user.gender}</TableCell>
                                        <TableCell>{user.email}</TableCell>
                                        <TableCell>{user.contact}</TableCell>
                                        <TableCell>{user.type}</TableCell>
                                    </TableRow>
                                ))}
                            </TableBody>
                        ) : null}
                        </Query>
                    </Table>
                </Paper>
            </div>
        );
    }
}

export default withStyles(styles)(UserDashboard);
