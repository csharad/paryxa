import React, { Component } from "react";
import { Typography, Grid, withStyles, Button, Icon } from "@material-ui/core";
import PropTypes from "prop-types";
import { Link } from "react-router-dom";

const styles = theme => ({
  spacing: {
    marginTop: theme.spacing.unit * 2
  }
});

class InfoBeforeTest extends Component {
  static propTypes = {
    classes: PropTypes.object.isRequired
  };

  render() {
    const { classes } = this.props;

    return (
      <Grid container justify="center" className={classes.spacing}>
        <Grid item container md={6}>
          <Typography variant="display1" align="center">
            Before you begin the test
          </Typography>

          <Typography className={classes.spacing}>
            Lorem ipsum dolor sit amet consectetur adipisicing elit. Quibusdam
            nesciunt iusto nihil aut voluptate omnis ipsa illum dolore similique
            facilis, non exercitationem saepe quisquam suscipit voluptas
            architecto officiis veritatis sapiente. Lorem ipsum dolor sit amet
            consectetur adipisicing elit. Quibusdam nesciunt iusto nihil aut
            voluptate omnis ipsa illum dolore similique facilis, non
            exercitationem saepe quisquam suscipit voluptas architecto officiis
            veritatis sapiente. Lorem ipsum dolor sit amet consectetur
            adipisicing elit. Quibusdam nesciunt iusto nihil aut voluptate omnis
            ipsa illum dolore similique facilis, non exercitationem saepe
            quisquam suscipit voluptas architecto officiis veritatis sapiente.
            <ol>
              <li>Lorem ipsum dolor, sit amet consectetur adipisicing elit.</li>
              <li>Lorem ipsum dolor, sit amet consectetur adipisicing elit.</li>
              <li>Lorem ipsum dolor, sit amet consectetur adipisicing elit.</li>
              <li>Lorem ipsum dolor, sit amet consectetur adipisicing elit.</li>
              <li>Lorem ipsum dolor, sit amet consectetur adipisicing elit.</li>
            </ol>
          </Typography>

          <Grid item container justify="flex-end">
            <Button
              variant="contained"
              color="secondary"
              component={Link}
              to="/test-paper"
            >
              Begin
              <Icon>arrow_right</Icon>
            </Button>
          </Grid>
        </Grid>
      </Grid>
    );
  }
}

export default withStyles(styles)(InfoBeforeTest);
