import React, { Component } from "react";
import {
  Paper,
  withStyles,
  Typography,
  Grid,
  Button,
  MenuItem
} from "@material-ui/core";
import PropTypes from "prop-types";
import { Formik } from "formik";
import * as yup from "yup";
import { FTextField } from "../Form";
import QuestionsDialog from "./QuestionsDialog";
import { Mutation } from "react-apollo";
import gql from "graphql-tag";

const styles = theme => ({
  container: {
    margin: theme.spacing.unit
  },
  title: {
    padding: `
      ${theme.spacing.unit * 3}px 
      ${theme.spacing.unit * 3}px 
      0 
      ${theme.spacing.unit * 3}px
    `,
    margin: 0
  },
  testForm: {
    padding: `
      0 
      ${theme.spacing.unit * 3}px
      ${theme.spacing.unit * 2}px
      ${theme.spacing.unit * 3}px
    `
  },
  buttonGroup: {
    marginTop: 8
  },
  questions: {
    margin: `${theme.spacing.unit * 3}px 0`
  }
});

class NewTest extends Component {
  static propTypes = {
    classes: PropTypes.object.isRequired
  };

  state = {
    openDialog: false,
    questions: []
  };

  render() {
    const { classes } = this.props;
    const { openDialog, questions } = this.state;

    const testForm = ({ isSubmitting, handleSubmit }) => (
      <form className={classes.testForm} onSubmit={handleSubmit}>
        <Grid container spacing={16}>
          <Grid item md={6}>
            <FTextField name="name" label="Name" margin="normal" fullWidth />
          </Grid>
          <Grid item md={6}>
            <FTextField
              name="type"
              label="Type"
              select
              margin="normal"
              fullWidth
            >
              <MenuItem value="SCHEDULED">Scheduled</MenuItem>
              <MenuItem value="FREE_FORM">Free-Form</MenuItem>
            </FTextField>
          </Grid>
        </Grid>
        <FTextField
          name="description"
          label="Description"
          margin="normal"
          multiline
          fullWidth
          rows={4}
          rowsMax={8}
        />

        {questions.length !== 0 ? (
          <div className={classes.questions}>
            {questions.map((quest, index) => (
              <Typography key={index} variant="subtitle2">
                {index + 1}. {quest.question}
              </Typography>
            ))}
          </div>
        ) : null}

        <Grid container justify="space-between" className={classes.buttonGroup}>
          <Button
            disabled={isSubmitting}
            onClick={() =>
              this.setState({
                openDialog: true
              })
            }
          >
            Add Questions
          </Button>

          <Button
            type="submit"
            disabled={isSubmitting}
            variant="contained"
            color="primary"
          >
            Save
          </Button>
        </Grid>
      </form>
    );

    return (
      <Paper className={classes.container}>
        <Typography variant="h5" className={classes.title}>
          New Test
        </Typography>

        <Mutation
          mutation={gql`
            mutation CreateTestPaper($testPaper: TestPaperForm!) {
              createTestPaper(testPaper: $testPaper) {
                id
              }
            }
          `}
        >
          {mutator => (
            <Formik
              initialValues={{
                name: "",
                description: "",
                type: ""
              }}
              validationSchema={yup.object().shape({
                name: yup.string().required(),
                description: yup.string(),
                type: yup
                  .string()
                  .oneOf(["SCHEDULED", "FREE_FORM"])
                  .required()
              })}
              onSubmit={async (data, action) => {
                try {
                  await mutator({
                    variables: {
                      testPaper: {
                        name: data.name,
                        description: data.description,
                        type: data.type,
                        questions: questions
                      }
                    }
                  });
                  action.resetForm();
                  this.setState({
                    questions: []
                  });
                } catch (e) {
                  console.log(e);
                }
                action.setSubmitting(false);
              }}
            >
              {testForm}
            </Formik>
          )}
        </Mutation>

        <QuestionsDialog
          questions={questions}
          open={openDialog}
          onClose={questions =>
            this.setState({
              openDialog: false,
              questions
            })
          }
        />
      </Paper>
    );
  }
}

export default withStyles(styles)(NewTest);
