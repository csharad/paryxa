import React, { Component, Fragment } from "react";
import {
  Drawer,
  List,
  ListItem,
  ListItemText,
  withStyles,
  Icon,
  ListItemIcon
} from "@material-ui/core";
import PropTypes from "prop-types";
import { Link, Route } from "react-router-dom";
import UserDashboard from "./UserDashboard";
import TestsDashboard from "./TestsDashboard";
import NewTest from "./NewTest";

const drawerWidth = 220;

const styles = {
  drawerPaper: {
    width: drawerWidth
  },
  rightSide: {
    marginLeft: drawerWidth
  }
};

class Dashboard extends Component {
  static propTypes = {
    classes: PropTypes.object.isRequired
  };

  render() {
    const { classes } = this.props;

    return (
      <Fragment>
        <Drawer variant="permanent" classes={{ paper: classes.drawerPaper }}>
          <List>
            <ListItem button component={Link} to="/dashboard/users">
              <ListItemIcon>
                <Icon>supervisor_account</Icon>
              </ListItemIcon>
              <ListItemText>Users</ListItemText>
            </ListItem>
            <ListItem button component={Link} to="/dashboard/tests">
              <ListItemIcon>
                <Icon>receipt</Icon>
              </ListItemIcon>
              <ListItemText>Tests</ListItemText>
            </ListItem>
            <ListItem button component={Link} to="/dashboard/new-test">
              <ListItemIcon>
                <Icon>plus_one</Icon>
              </ListItemIcon>
              <ListItemText>New Test</ListItemText>
            </ListItem>
          </List>
        </Drawer>

        <div className={classes.rightSide}>
          <Route path="/dashboard/users" component={UserDashboard} />
          <Route path="/dashboard/tests" component={TestsDashboard} />
          <Route path="/dashboard/new-test" component={NewTest} />
        </div>
      </Fragment>
    );
  }
}

export default withStyles(styles)(Dashboard);
