INSERT INTO osvm_posts (
    post_name, user_id,
    date_time, body,
    bbs_id, topic_id
) VALUES ($1, $2, $3, $4, $5, $6)
