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
  InputBase,
  Button,
  TextField,
  MenuItem
} from "@material-ui/core";
import PropTypes from "prop-types";
import { Query, Mutation } from "react-apollo";
import gql from "graphql-tag";
import { debounce } from "debounce";

const styles = theme => ({
  container: {
    padding: `${theme.spacing.unit * 2}px ${theme.spacing.unit * 2}px 0 ${theme
      .spacing.unit * 3}px`
  },
  marginBottom: {
    marginBottom: theme.spacing.unit * 2
  },
  searchInput: {
    width: 300,
    backgroundColor: theme.palette.grey[200],
    borderRadius: theme.shape.borderRadius,
    padding: theme.spacing.unit,
    "&:hover": {
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
    query: null,
    editableUser: null,
    selectedUserType: null
  };

  setQuery = debounce(val => this.setState({ query: val }), 200);

  render() {
    const { classes } = this.props;
    let { query, editableUser, selectedUserType } = this.state;
    query = query && query.length === 0 ? null : query;

    const userType = user =>
      editableUser === user.id ? (
        <TextField
          select
          value={selectedUserType || user.type}
          onChange={ev =>
            this.setState({
              selectedUserType: ev.target.value
            })
          }
        >
          <MenuItem value="ADMIN">Admin</MenuItem>
          <MenuItem value="NORMAL">Normal</MenuItem>
        </TextField>
      ) : (
        user.type
      );

    const actions = user => (
      <Mutation
        mutation={gql`
          mutation UpdateUserType($userType: UserTypeUpdate!) {
            updateUserType(userType: $userType) {
              id
              type
            }
          }
        `}
      >
        {(updateType, { loading }) =>
          editableUser === user.id ? (
            <Fragment>
              <Button
                size="small"
                onClick={async () => {
                  if (selectedUserType) {
                    await updateType({
                      variables: {
                        userType: {
                          id: editableUser,
                          type: selectedUserType
                        }
                      }
                    });
                  }
                  this.setState({
                    editableUser: null,
                    selectedUserType: null
                  });
                }}
                disabled={loading}
              >
                Save
              </Button>
              <Button
                size="small"
                onClick={() =>
                  this.setState({
                    editableUser: null,
                    selectedUserType: null
                  })
                }
                disabled={loading}
              >
                Close
              </Button>
            </Fragment>
          ) : (
            <Button
              key={2}
              size="small"
              onClick={() =>
                this.setState({
                  editableUser: user.id
                })
              }
              disabled={loading}
            >
              Edit
            </Button>
          )
        }
      </Mutation>
    );

    const tableRow = user => (
      <TableRow key={user.id}>
        <TableCell>{user.fullName}</TableCell>
        <TableCell>{user.gender}</TableCell>
        <TableCell>{user.email}</TableCell>
        <TableCell>{user.contact}</TableCell>
        <TableCell>{userType(user)}</TableCell>
        <TableCell>{actions(user)}</TableCell>
      </TableRow>
    );

    const table = (
      <Table>
        <TableHead>
          <TableRow>
            <TableCell>Name</TableCell>
            <TableCell>Gender</TableCell>
            <TableCell>Email</TableCell>
            <TableCell>Contact</TableCell>
            <TableCell>Subscription</TableCell>
            <TableCell>Action</TableCell>
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
            query
          }}
        >
          {({ data, loading }) =>
            !loading ? <TableBody>{data.users.map(tableRow)}</TableBody> : null
          }
        </Query>
      </Table>
    );

    return (
      <div className={classes.container}>
        <Typography variant="h5" className={classes.marginBottom}>
          Users
        </Typography>

        <InputBase
          placeholder="Seach by name, e-mail, contact"
          classes={{ input: classes.searchInput }}
          onInput={ev => this.setQuery(ev.target.value)}
          className={classes.marginBottom}
        />

        <Paper>{table}</Paper>
      </div>
    );
  }
}

export default withStyles(styles)(UserDashboard);
