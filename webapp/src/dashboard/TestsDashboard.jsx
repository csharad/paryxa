import React, { Component, Fragment } from "react";
import {
  Table,
  TableHead,
  TableBody,
  TableRow,
  TableCell,
  Paper,
  withStyles,
  Typography,
  TablePagination,
  Button
} from "@material-ui/core";
import PropTypes from "prop-types";
import { Query } from "react-apollo";
import gql from "graphql-tag";

const styles = theme => ({
  container: {
    margin: theme.spacing.unit,
    paddingTop: theme.spacing.unit
  },
  title: {
    padding: theme.spacing.unit * 3
  },
  tableMessage: {
    margin: theme.spacing.unit * 2
  }
});

class TestsDashboard extends Component {
  static propTypes = {
    classes: PropTypes.object.isRequired
  };

  render() {
    const { classes } = this.props;

    const tableRow = paper => (
      <TableRow key={paper.id}>
        <TableCell>{paper.name}</TableCell>
        <TableCell>{paper.totalQuestions}</TableCell>
        <TableCell>
          {paper.type === "SCHEDULED" ? "Scheduled" : "Free Form"}
        </TableCell>
        <TableCell>
          {paper.type === "SCHEDULED" ? (
            <Button size="small">Schedule</Button>
          ) : null}
        </TableCell>
      </TableRow>
    );

    const withHeader = (body, { page, perPage, count }, ifEmpty) => (
      <Fragment>
        <Table>
          <TableHead>
            <TableRow>
              <TableCell>Name</TableCell>
              <TableCell>Questions</TableCell>
              <TableCell>Type</TableCell>
              <TableCell>Actions</TableCell>
            </TableRow>
          </TableHead>
          {body}
        </Table>
        {ifEmpty}
        <TablePagination
          component="div"
          count={count}
          rowsPerPage={perPage}
          page={page}
          rowsPerPageOptions={[10]}
          onChangePage={() => {}}
          onChangeRowsPerPage={() => {}}
        />
      </Fragment>
    );

    const table = (
      <Query
        query={gql`
          query TestPaperList {
            testPapers {
              id
              name
              type
              totalQuestions
            }
          }
        `}
      >
        {({ data, loading }) =>
          !loading
            ? withHeader(
                <TableBody>{data.testPapers.map(tableRow)}</TableBody>,
                {
                  page: 0,
                  perPage: 10,
                  count: data.testPapers.length
                },
                data.testPapers.length === 0 ? (
                  <Typography
                    variant="subtitle2"
                    color="textSecondary"
                    align="center"
                    className={classes.tableMessage}
                  >
                    There are no records in the table.
                  </Typography>
                ) : null
              )
            : withHeader(
                null,
                {
                  page: 0,
                  perPage: 10,
                  count: 0
                },
                <Typography
                  variant="subtitle2"
                  color="textSecondary"
                  align="center"
                  className={classes.tableMessage}
                >
                  Searching...
                </Typography>
              )
        }
      </Query>
    );

    return (
      <Paper className={classes.container}>
        <Typography variant="h5" className={classes.title}>
          Tests
        </Typography>
        {table}
      </Paper>
    );
  }
}

export default withStyles(styles)(TestsDashboard);
