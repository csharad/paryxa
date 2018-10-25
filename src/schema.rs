table! {
    use diesel::sql_types::*;
    use db_types::*;

    question_answers (id) {
        id -> Int4,
        test_room_id -> Int4,
        test_question_id -> Int4,
        answered_option -> Int4,
    }
}

table! {
    use diesel::sql_types::*;
    use db_types::*;

    question_options (id) {
        id -> Int4,
        uuid -> Uuid,
        option -> Text,
        test_question_id -> Int4,
        is_correct -> Nullable<Bool>,
    }
}

table! {
    use diesel::sql_types::*;
    use db_types::*;

    test_papers (id) {
        id -> Int4,
        uuid -> Uuid,
        name -> Text,
        description -> Nullable<Text>,
        #[sql_name = "type"]
        type_ -> Test_type,
    }
}

table! {
    use diesel::sql_types::*;
    use db_types::*;

    test_questions (id) {
        id -> Int4,
        uuid -> Uuid,
        question -> Text,
        test_paper_id -> Int4,
    }
}

table! {
    use diesel::sql_types::*;
    use db_types::*;

    test_rooms (id) {
        id -> Int4,
        uuid -> Uuid,
        user_id -> Int4,
        test_paper_id -> Int4,
        test_schedule_id -> Int4,
        start_time -> Timestamp,
        finish_time -> Nullable<Timestamp>,
        has_withdrawn -> Nullable<Bool>,
    }
}

table! {
    use diesel::sql_types::*;
    use db_types::*;

    test_schedules (id) {
        id -> Int4,
        uuid -> Uuid,
        test_paper_id -> Int4,
        time -> Timestamp,
        duration -> Int4,
    }
}

table! {
    use diesel::sql_types::*;
    use db_types::*;

    test_subscriptions (id) {
        id -> Int4,
        uuid -> Uuid,
        user_id -> Int4,
        test_paper_id -> Int4,
        test_schedule_id -> Int4,
    }
}

table! {
    use diesel::sql_types::*;
    use db_types::*;

    users (id) {
        id -> Int4,
        uuid -> Uuid,
        first_name -> Nullable<Text>,
        last_name -> Nullable<Text>,
        email -> Text,
        password -> Text,
        gender -> Nullable<Gender_type>,
        contact -> Nullable<Text>,
        #[sql_name = "type"]
        type_ -> User_type,
    }
}

joinable!(question_answers -> question_options (answered_option));
joinable!(question_answers -> test_questions (test_question_id));
joinable!(question_answers -> test_rooms (test_room_id));
joinable!(question_options -> test_questions (test_question_id));
joinable!(test_questions -> test_papers (test_paper_id));
joinable!(test_schedules -> test_papers (test_paper_id));
joinable!(test_subscriptions -> test_papers (test_paper_id));
joinable!(test_subscriptions -> test_schedules (test_schedule_id));
joinable!(test_subscriptions -> users (user_id));

allow_tables_to_appear_in_same_query!(
    question_answers,
    question_options,
    test_papers,
    test_questions,
    test_rooms,
    test_schedules,
    test_subscriptions,
    users,
);
