import React, { Component } from "react";
import {
  Dialog,
  withStyles,
  Button,
  Grid,
  Typography
} from "@material-ui/core";
import PropTypes from "prop-types";
import { FTextField, FRadioLabelless } from "../Form";
import { Formik } from "formik";
import * as yup from "yup";

const styles = theme => ({
  dialog: {
    padding: `${theme.spacing.unit * 2}px ${theme.spacing.unit * 3}px`,
    width: 500
  },
  buttons: {
    marginTop: theme.spacing.unit * 2
  },
  buttonSpacing: {
    marginLeft: theme.spacing.unit
  }
});

function isStrEmpty(str) {
  if (typeof str === "string" && str.trim().length === 0) {
    return true;
  } else {
    return false;
  }
}

class QuestionsDialog extends Component {
  static propTypes = {
    open: PropTypes.bool.isRequired,
    onClose: PropTypes.func.isRequired,
    classes: PropTypes.object.isRequired,
    questions: PropTypes.arrayOf(PropTypes.object).isRequired
  };

  state = {
    questions: [],
    done: false
  };

  render() {
    const { classes, open, onClose } = this.props;
    const { questions } = this.state;

    const form = ({ handleChange, handleSubmit, values }) => (
      <form onSubmit={handleSubmit}>
        <FTextField
          name="question"
          label="Question"
          margin="normal"
          fullWidth
          multiline
        />
        <Grid container alignItems="center">
          <Grid item>
            <FRadioLabelless name="correct" value="option0" />
          </Grid>
          <Grid item>
            <FTextField
              name="option0"
              label="Option"
              margin="normal"
              multiline
              InputLabelProps={{
                shrink: true
              }}
            />
          </Grid>
        </Grid>
        <Grid container alignItems="center">
          <Grid item>
            <FRadioLabelless name="correct" value="option1" />
          </Grid>
          <Grid item>
            <FTextField
              name="option1"
              label="Option"
              margin="normal"
              multiline
              InputLabelProps={{
                shrink: true
              }}
            />
          </Grid>
        </Grid>
        <Grid container alignItems="center">
          <Grid item>
            <FRadioLabelless name="correct" value="option2" />
          </Grid>
          <Grid item>
            <FTextField
              name="option2"
              label="Option"
              margin="normal"
              multiline
              InputLabelProps={{
                shrink: true
              }}
            />
          </Grid>
        </Grid>
        <Grid container alignItems="center">
          <Grid item>
            <FRadioLabelless name="correct" value="option3" />
          </Grid>
          <Grid item>
            <FTextField
              name="option3"
              label="Option"
              margin="normal"
              multiline
              InputLabelProps={{
                shrink: true
              }}
            />
          </Grid>
        </Grid>

        <Grid container justify="flex-end" className={classes.buttons}>
          <Button
            onClick={() => {
              if (
                isStrEmpty(values.question) &&
                isStrEmpty(values.option0) &&
                isStrEmpty(values.option1) &&
                isStrEmpty(values.option2) &&
                isStrEmpty(values.option3)
              ) {
                onClose(questions);
              } else {
                this.setState(
                  {
                    done: true
                  },
                  () => {
                    handleSubmit();
                  }
                );
              }
            }}
          >
            Finish
          </Button>
          <Button
            type="submit"
            className={classes.buttonSpacing}
            color="primary"
          >
            Add 1 More
          </Button>
        </Grid>
      </form>
    );

    return (
      <Dialog
        open={open}
        onClose={() => {
          onClose(questions);
        }}
        classes={{
          paper: classes.dialog
        }}
      >
        <Typography variant="h5" color="textSecondary">
          # {questions.length + 1}
        </Typography>
        <Formik
          initialValues={{
            question: "",
            option0: "",
            option1: "",
            option2: "",
            option3: "",
            correct: ""
          }}
          validationSchema={yup.object().shape({
            question: yup.string().required(),
            option0: yup.string().required(),
            option1: yup.string().required(),
            option2: yup.string().required(),
            option3: yup.string().required(),
            correct: yup
              .string()
              .oneOf(["option0", "option1", "option2", "option3"])
              .required()
          })}
          onSubmit={this.addQuestion}
        >
          {form}
        </Formik>
      </Dialog>
    );
  }

  addQuestion = (
    { question, option0, option1, option2, option3, correct },
    actions
  ) => {
    const { done, questions } = this.state;
    this.setState(
      {
        questions: [
          ...questions,
          {
            question,
            options: [
              { option: option0, isCorrect: correct === "option0" },
              { option: option1, isCorrect: correct === "option1" },
              { option: option2, isCorrect: correct === "option2" },
              { option: option3, isCorrect: correct === "option3" }
            ]
          }
        ]
      },
      () => {
        actions.resetForm();
        if (done) {
          this.setState(
            {
              done: false
            },
            () => {
              this.props.onClose(this.state.questions);
            }
          );
        }
      }
    );
  };
}

export default withStyles(styles)(QuestionsDialog);
