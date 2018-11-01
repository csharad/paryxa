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
import { Query } from 'react-apollo';
import gql from 'graphql-tag';

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

        const tableRow = (paper) => (
            <TableRow key={paper.id}>
                <TableCell>{paper.name}</TableCell>
                <TableCell>{paper.type}</TableCell>
                <TableCell>Actions</TableCell>
            </TableRow>
        );

        const table = (
            <Table>
                <TableHead>
                    <TableRow>
                        <TableCell>Name</TableCell>
                        <TableCell>Type</TableCell>
                        <TableCell>Actions</TableCell>
                    </TableRow>
                </TableHead>
                <TableBody>
                    <Query
                        query={gql`
                            query TestPaperList {
                                testPapers {
                                    id
                                    name
                                    type
                                }
                            }
                        `}
                    >{({ data, loading }) => !loading ? data.testPapers.map(tableRow) : null}
                    </Query>
                </TableBody>
            </Table>
        );

        return (
            <div className={classes.container}>
                <Typography
                    variant="h5"
                    className={classes.title}
                >Tests</Typography>

                <Paper>{table}</Paper>
            </div>
        );
    }
}

export default withStyles(styles)(TestsDashboard);
