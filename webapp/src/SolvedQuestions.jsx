import React, { Component } from "react";
import {
  List,
  ListItem,
  Grid,
  ListItemText,
  Typography,
  ListItemIcon,
  Icon,
  Button,
  withStyles
} from "@material-ui/core";
import { Link } from "react-router-dom";

const styles = theme => ({
  testHeading: {
    margin: `${theme.spacing.unit * 3}px 0`
  }
});

class SolvedQuestions extends Component {
  render() {
    const questions = Array(50).fill({
      question: "What is the question?",
      answers: [
        {
          answer: "Option 1",
          isCorrect: true
        },
        {
          answer: "Option 2"
        },
        {
          answer: "Option 3"
        },
        {
          answer: "Option 4"
        }
      ]
    });

    const { classes } = this.props;

    return (
      <Grid container justify="center">
        <Grid item md={8} spacing={16}>
          <Typography variant="display1" className={classes.testHeading}>
            Title of the Test
          </Typography>

          {questions.map((question, index) => (
            <Question question={question} index={index} key={index} />
          ))}

          <Button component={Link} to="/profile">
            Go To Profile
          </Button>
        </Grid>
      </Grid>
    );
  }
}

function Question({ index, question: { question, answers } }) {
  return (
    <div style={{ marginBottom: 30 }}>
      <Typography variant="h6">
        {index + 1}. {question}
      </Typography>
      <List style={{ backgroundColor: "white" }}>
        {answers.map((answer, index) => (
          <Answer isCorrect={answer.isCorrect} index={index}>
            {answer.answer}
          </Answer>
        ))}
      </List>
    </div>
  );
}

function Answer({ children, isCorrect }) {
  return (
    <ListItem button selected={isCorrect} disabled>
      <ListItemText>{children}</ListItemText>

      {isCorrect ? (
        <ListItemIcon>
          <Icon>check</Icon>
        </ListItemIcon>
      ) : (
        ""
      )}
    </ListItem>
  );
}

export default withStyles(styles)(SolvedQuestions);
